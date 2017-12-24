use matrix;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Markov {
    world: matrix::Matrix<State>,
    gama: f64,
    cost_of_move: f64,
    p1: f64,
    p2: f64,
    p3: f64,
    p4: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MarkovBuilder {
    x: usize,
    y: usize,
    states: Vec<(State, usize, usize)>,
    gama: f64,
    cost_of_move: f64,
    p1: f64,
    p2: f64,
    p3: f64,
    p4: f64
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum State {
    ProhibitedState,
    StartState(f64),
    TerminalState(f64),
    SpecialState(f64),
    NormalState(f64)
}

#[derive(Clone, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right
}

impl fmt::Debug for State {
    // from specification http://sequoia.ict.pwr.wroc.pl/~witold/ai/MDPRL_assignment.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &State::ProhibitedState => write!(f, "F"),
            &State::StartState(value) => write!(f, "S({:.3})", value),
            &State::TerminalState(value) => write!(f, "T({:.3})", value),
            &State::SpecialState(value) => write!(f, "B({:.3})", value),
            &State::NormalState(value) => write!(f, "N({:.3})", value),
        }
    }
}

impl fmt::Debug for Action {
    // from specification http://sequoia.ict.pwr.wroc.pl/~witold/ai/MDPRL_assignment.html
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Up => write!(f, "^"),
            Left => write!(f, "<"),
            Right => write!(f, ">"),
            Down => write!(f, "v")
        }
    }
}

trait Roundable {
    fn round_to(&self, precision: i32) -> f64;
}

impl Roundable for f64 {
    fn round_to(&self, precision: i32) -> f64 {
        let divisor = 10_f64.powi(precision);
        (*self*divisor).round()/divisor
    }
}

impl MarkovBuilder {
    // pub fn new() -> MarkovBuilder {
    //     MarkovBuilder {
    //         x: 0,
    //         y: 0,
    //         states: Vec::new(),
    //         gama: 1.0,
    //         cost_of_move: 0.0,
    //         p1: 0.8,
    //         p2: 0.1,
    //         p3: 0.1,
    //         p4: 0.0
    //     }
    // }

    pub fn finalize(&self) -> Markov {
        let mut matrix = matrix::Matrix::new(State::NormalState(0.0), 4, 3);
        for &(ref state, x, y) in self.states.iter() {
            matrix.set_state(state.clone(), x, y);
        }
        Markov {
            world: matrix,
            gama: self.gama,
            cost_of_move: self.cost_of_move,
            p1: self.p1,
            p2: self.p2,
            p3: self.p3,
            p4: self.p4
        }
    }
}

pub fn left_operation(action: &Action) -> Action {
    match action {
        &Action::Up => Action::Left,
        &Action::Left => Action::Down,
        &Action::Down => Action::Right,
        &Action::Right => Action::Up
    }
}

pub fn right_operation(action: &Action) -> Action {
    match action {
        &Action::Up => Action::Right,
        &Action::Right => Action::Down,
        &Action::Down => Action::Left,
        &Action::Left => Action::Up
    }
}

pub fn reverse_operation(action: &Action) -> Action {
    match action {
        &Action::Up => Action::Down,
        &Action::Left => Action::Right,
        &Action::Down => Action::Up,
        &Action::Right => Action::Left
    }
}

impl Markov {
    fn new() -> Markov {
        Markov {
            world: matrix::Matrix::new(State::NormalState(0.0), 4, 3),
            gama: 1.0,
            cost_of_move: -0.04,
            p1: 0.8,
            p2: 0.1,
            p3: 0.1,
            p4: 0.0
        }
    }

    fn state_after_action(self: &Markov, action: &Action, x: usize, y: usize) -> &State {
        let (maybe_x, maybe_y) = match action {
            &Action::Left => (x.checked_sub(1), Some(y)),
            &Action::Right => (x.checked_add(1), Some(y)),
            &Action::Up => (Some(x), y.checked_sub(1)),
            &Action::Down => (Some(x), y.checked_add(1))
        };

        match (maybe_x, maybe_y) {
            (Some(_), Some(_)) => {
                let maybe_state_after_move = self.world.read_state(maybe_x.unwrap(), maybe_y.unwrap());
                match maybe_state_after_move {
                    Some(state_after_move) => match state_after_move {
                        &State::ProhibitedState => self.world.read_state(x, y).unwrap(), // return my current place (wall bump)
                        _ => state_after_move // all other places are valid, so just return them
                    }
                    None => self.world.read_state(x, y).unwrap() // going outside world (some index too high), return current place
                }
            },
            _ => self.world.read_state(x, y).unwrap() // going outside world (index overflow), return current place
        }
    }

    fn evaluate_action(self: &Markov, action: &Action, x: usize, y: usize) -> (f64) {
        let forward_state  = self.state_after_action(action, x, y);
        let left_state     = self.state_after_action(&left_operation(action), x, y);
        let right_state    = self.state_after_action(&right_operation(action), x, y);
        let backward_state = self.state_after_action(&reverse_operation(action), x, y);

        let state_to_reward = |state: &State| -> f64 {
            match state {
                &State::ProhibitedState => panic!("It should not be able to obtain value from prohibited state"),
                &State::StartState(value) => value,
                &State::NormalState(value) => value,
                &State::TerminalState(value) => value,
                &State::SpecialState(value) => value
            }
        };

        let forward_reward  = self.p1*state_to_reward(&forward_state);
        let left_reward     = self.p2*state_to_reward(&left_state);
        let right_reward    = self.p3*state_to_reward(&right_state);
        let backward_reward = self.p4*state_to_reward(&backward_state);

        return forward_reward + left_reward + right_reward + backward_reward;
    }

    fn evaluate_field(self: &Markov, state: &State, x: usize, y: usize) -> (State, Action) {
        match state {
            &State::TerminalState(value) => { return (State::TerminalState(value), Action::Left); },
            &State::ProhibitedState => { return (State::ProhibitedState, Action::Left); }
            _ => {}
        }

        let up_reward    = self.evaluate_action(&Action::Up, x, y);
        let left_reward  = self.evaluate_action(&Action::Left, x, y);
        let right_reward = self.evaluate_action(&Action::Right, x, y);
        let down_reward  = self.evaluate_action(&Action::Down, x, y);

        let max = up_reward.max(left_reward.max(right_reward.max(down_reward)));

        let result = max*self.gama + self.cost_of_move;

        let updated_state = match state {
            &State::ProhibitedState => State::ProhibitedState,
            &State::StartState(_) => State::StartState(result),
            &State::NormalState(_) => State::NormalState(result),
            &State::TerminalState(_) => State::TerminalState(result),
            &State::SpecialState(_) => State::SpecialState(result)
        };

        (updated_state, Action::Left) // TODO: remove hardcoded optimal action
    }

    pub fn evaluate(self: &mut Markov) -> f64 {
        let mut new_world = self.world.clone();
        let mut error = 0.0;

        let calculate_error = |previous_state: &State, new_state: &State| -> f64 {
            match (previous_state, new_state) {
                (&State::ProhibitedState, &State::ProhibitedState) => 0.0,
                (&State::NormalState(previous_value), &State::NormalState(new_value)) => (new_value - previous_value).abs(),
                (&State::StartState(previous_value), &State::StartState(new_value)) => (new_value - previous_value).abs(),
                (&State::SpecialState(previous_value), &State::SpecialState(new_value)) => (new_value - previous_value).abs(),
                (&State::TerminalState(previous_value), &State::TerminalState(new_value)) => (new_value - previous_value).abs(),
                _ => panic!("Type of state changed, it should never happen")
            }
        };

        for (y, row) in self.world.matrix().iter().enumerate() {
            for (x, elem) in row.iter().enumerate() {
                let (new_state, action) = self.evaluate_field(elem, x, y);
                error += calculate_error(&elem, &new_state);
                new_world.set_state(new_state, x, y);
            }
        }

        self.world = new_world;

        return error;
        // U(s) = R(s) + gama max{T(s,a,s')U(s')}

        // match (&self.data[y][x], &new_state)
        // {
        //     (&State::NormalState(_), &State::NormalState(_)) => {
        //         println!("Updated value at [{}][{}] from {:?} to {:?}", x, y, &self.data[y][x], &new_state);
        //     }
        //     (&State::NormalState(_), _) => {
        //         println!("Created {:?} on position [{}][{}]", &new_state, x, y);
        //     }
        //     _ => {
        //         println!("Invalid operation {:?} on position [{}][{}]", new_state, x, y);
        //         return false;
        //     }
        // }
    }
}

#[test]
fn left_operation_calculations() {
    assert_eq!(Action::Left, left_operation(&Action::Up));
    assert_eq!(Action::Down, left_operation(&Action::Left));
    assert_eq!(Action::Right, left_operation(&Action::Down));
    assert_eq!(Action::Up, left_operation(&Action::Right));
}

#[test]
fn right_operation_calculations() {
    assert_eq!(Action::Right, right_operation(&Action::Up));
    assert_eq!(Action::Up, right_operation(&Action::Left));
    assert_eq!(Action::Left, right_operation(&Action::Down));
    assert_eq!(Action::Down, right_operation(&Action::Right));
}

#[test]
fn reverse_operation_calculations() {
    assert_eq!(Action::Left, reverse_operation(&Action::Right));
    assert_eq!(Action::Down, reverse_operation(&Action::Up));
    assert_eq!(Action::Up, reverse_operation(&Action::Down));
    assert_eq!(Action::Right, reverse_operation(&Action::Left));
}

#[test]
fn prepare_standard_world() {
    let mut markov: Markov = Markov::new();

    markov.world.set_state(State::StartState(0.0), 0, 2);
    markov.world.set_state(State::ProhibitedState, 1, 1);
    markov.world.set_state(State::TerminalState(1.0), 3, 0);
    markov.world.set_state(State::TerminalState(-1.0), 3, 1);

    assert_eq!(Some(&State::StartState(0.0)), markov.world.read_state(0,2));
    assert_eq!(Some(&State::ProhibitedState), markov.world.read_state(1,1));
    assert_eq!(Some(&State::TerminalState(1.0)), markov.world.read_state(3,0));
    assert_eq!(Some(&State::TerminalState(-1.0)), markov.world.read_state(3,1));
}

#[test]
fn calculate_state_after_action() {
    let mut markov = Markov::new();

    markov.world.set_state(State::StartState(0.0), 0, 2);
    markov.world.set_state(State::ProhibitedState, 1, 1);
    markov.world.set_state(State::TerminalState(1.0), 3, 0);
    markov.world.set_state(State::TerminalState(-1.0), 3, 1);

    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Up, 0, 0));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Left, 0, 0));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Down, 0, 0));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Right, 0, 0));

    assert_eq!(&State::StartState(0.0), markov.state_after_action(&Action::Down, 0, 2));
    assert_eq!(&State::StartState(0.0), markov.state_after_action(&Action::Left, 0, 2));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Up, 0, 2));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Right, 0, 2));

    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Right, 3, 2));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Down, 3, 2));
    assert_eq!(&State::TerminalState(-1.0), markov.state_after_action(&Action::Up, 3, 2));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Left, 3, 2));

    assert_eq!(&State::TerminalState(1.0), markov.state_after_action(&Action::Right, 3, 0));
    assert_eq!(&State::TerminalState(1.0), markov.state_after_action(&Action::Up, 3, 0));
    assert_eq!(&State::TerminalState(-1.0), markov.state_after_action(&Action::Down, 3, 0));
    assert_eq!(&State::NormalState(0.0), markov.state_after_action(&Action::Left, 3, 0));
}

#[test]
fn calculate_evaluation_of_action() {
    // example from slide 17 at http://ais.informatik.uni-freiburg.de/teaching/ss03/ams/DecisionProblems.pdf
    let mut markov = Markov::new();

    markov.world.set_state(State::NormalState(1.0), 1, 1);
    markov.world.set_state(State::NormalState(1.0), 1, 2);
    markov.world.set_state(State::NormalState(5.0), 0, 1);
    markov.world.set_state(State::NormalState(-8.0), 2, 1);
    markov.world.set_state(State::NormalState(10.0), 1, 0);

    assert_eq!(0.5, markov.evaluate_action(&Action::Down, 1, 1).round_to(3));
    assert_eq!(7.7, markov.evaluate_action(&Action::Up, 1, 1).round_to(3));
    assert_eq!(5.1, markov.evaluate_action(&Action::Left, 1, 1).round_to(3));
    assert_eq!(-5.3, markov.evaluate_action(&Action::Right, 1, 1).round_to(3));
}

#[test]
fn calculate_standard_world() {
    // example from slide 4 at http://sequoia.ict.pwr.wroc.pl/~witold/ai/aie_rlearn_s.pdf
    let mut markov = Markov::new();

    markov.world.set_state(State::StartState(0.0), 0, 2);
    markov.world.set_state(State::ProhibitedState, 1, 1);
    markov.world.set_state(State::TerminalState(1.0), 3, 0);
    markov.world.set_state(State::TerminalState(-1.0), 3, 1);

    // after ~20 iterations should converge to fast zero error
    for _ in 0..30 {
        markov.evaluate();
    }

    assert_eq!(Some(&State::NormalState(0.8115582189599785)), markov.world.read_state(0,0));
    assert_eq!(Some(&State::NormalState(0.8678082191773653)), markov.world.read_state(1,0));
    assert_eq!(Some(&State::NormalState(0.9178082191779183)), markov.world.read_state(2,0));
    assert_eq!(Some(&State::TerminalState(1.0)),              markov.world.read_state(3,0));

    assert_eq!(Some(&State::NormalState(0.7615582184462935)), markov.world.read_state(0,1));
    assert_eq!(Some(&State::ProhibitedState),                 markov.world.read_state(1,1));
    assert_eq!(Some(&State::NormalState(0.6602739726022764)), markov.world.read_state(2,1));
    assert_eq!(Some(&State::TerminalState(-1.0)),             markov.world.read_state(3,1));

    assert_eq!(Some(&State::StartState(0.7053082070401893)),  markov.world.read_state(0,2));
    assert_eq!(Some(&State::NormalState(0.6553081816744336)), markov.world.read_state(1,2));
    assert_eq!(Some(&State::NormalState(0.611415441839725)),  markov.world.read_state(2,2));
    assert_eq!(Some(&State::NormalState(0.3879247270957595)), markov.world.read_state(3,2));
}

#[test]
fn update_normal_state() {
    let mut markov: Markov = Markov::new();

    assert_eq!(Some(&State::NormalState(0.0)), markov.world.read_state(1,1));

    markov.world.set_state(State::NormalState(4.2), 1,1);
    assert_eq!(Some(&State::NormalState(4.2)), markov.world.read_state(1,1));

    markov.world.set_state(State::NormalState(6.6), 1,1);
    assert_eq!(Some(&State::NormalState(6.6)), markov.world.read_state(1,1));
}

#[test]
#[ignore]
fn not_allow_normal_state_update_from_different_type() {
    let mut world = matrix::Matrix::new(State::NormalState(0.0), 4, 3);

    assert_eq!(true, world.set_state(State::ProhibitedState, 1, 1));
    assert_eq!(false, world.set_state(State::NormalState(4.2), 1, 1));
}

#[test]
#[ignore]
fn not_allow_immutable_state_update() {
    let mut world = matrix::Matrix::new(State::NormalState(0.0), 4, 3);

    assert_eq!(true, world.set_state(State::StartState(0.0), 1, 1));
    assert_eq!(false, world.set_state(State::StartState(0.0), 1, 1));
}
