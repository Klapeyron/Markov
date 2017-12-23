use matrix;

struct Markov {
    world: matrix::Matrix<State>,
    gama: f64,
    cost_of_move: f64,
    p1: f64,
    p2: f64,
    p3: f64,
    p4: f64
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    ProhibitedState,
    StartState(f64),
    TerminalState(f64),
    SpecialState(f64),
    NormalState(f64)
}

impl State {
    pub fn to_reward (maybe_state: &Option<&State>) -> f64 {
        match maybe_state {
            &Some(state) => match state {
                &State::ProhibitedState => 0.0,
                &State::StartState(value) => value,
                &State::NormalState(value) => value,
                &State::TerminalState(value) => value,
                &State::SpecialState(value) => value
            },
            &None => 0.0
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right
}

// impl fmt::Debug for State {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // write!(f, "{}", 7.0)
//     }
// }

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
            gama: 0.98,
            cost_of_move: -0.04,
            p1: 0.8,
            p2: 0.1,
            p3: 0.1,
            p4: 0.0
        }
    }

    fn state_after_action(self: &Markov, action: &Action, x: usize, y: usize) -> Option<&State> {
        let position_after_action = |action: &Action, x: usize, y: usize| -> (Option<usize>, Option<usize>) {
            match action {
                &Action::Left => (x.checked_sub(1), Some(y)),
                &Action::Right => (x.checked_add(1), Some(y)),
                &Action::Up => (Some(x), y.checked_sub(1)),
                &Action::Down => (Some(x), y.checked_add(1))
            }
        };

        let (x,y) = position_after_action(&action, x, y);

        match (x,y) {
            (Some(_), Some(_)) => self.world.read_state(x.unwrap(), y.unwrap()),
            _ => None
        }
    }

    fn evaluate_action(self: &Markov, state: &State, action: &Action, x: usize, y: usize) -> (f64) {
        let forward_state  = self.state_after_action(action, x, y);
        let left_state     = self.state_after_action(&left_operation(action), x, y);
        let right_state    = self.state_after_action(&right_operation(action), x, y);
        let backward_state = self.state_after_action(&reverse_operation(action), x, y);

        let forward_reward  = self.p1*State::to_reward(&forward_state);
        let left_reward     = self.p2*State::to_reward(&left_state);
        let right_reward    = self.p3*State::to_reward(&right_state);
        let backward_reward = self.p4*State::to_reward(&backward_state);

        return forward_reward + left_reward + right_reward + backward_reward;
    }

    fn evaluate_field(self: &Markov, state: &State, x: usize, y: usize) -> (State, Action) {
        let up_reward    :f64 = self.evaluate_action(&state, &Action::Up,    x, y);
        let left_reward  :f64 = self.evaluate_action(&state, &Action::Left,  x, y);
        let right_reward :f64 = self.evaluate_action(&state, &Action::Right, x, y);
        let down_reward  :f64 = self.evaluate_action(&state, &Action::Down,  x, y);

        // // TODO: calculate max
        let max = up_reward.max(left_reward.max(right_reward.max(down_reward)));

        // // TODO: calculate result
        let result = elem + self.gama*max - self.cost_of_move;

        // TODO: logic of state changes, skiping invalid

        let value_to_state = |state: &State, new_value: f64| -> State {
            match state {
                &State::ProhibitedState => State::ProhibitedState,
                &State::StartState(value) => State::StartState(new_value),
                &State::NormalState(value) => State::NormalState(new_value),
                &State::TerminalState(value) => State::TerminalState(new_value),
                &State::SpecialState(value) => State::SpecialState(new_value)
            }
        };

        (value_to_state(state, result), Action::Left) // TODO: remove hardcoded optimal action
    }

    pub fn evaluate(self: &Markov) -> matrix::Matrix<State> {
        // TODO: create copy of the world
        let mut new_world = self.world.clone();

        // TODO: iterate over all elements
        for (x, row) in self.world.matrix().iter().enumerate() {
            for (y, elem) in row.iter().enumerate() {
                let (new_state, action) = self.evaluate_field(elem, x, y);
                new_world.set_state(new_state, x, y);
            }
        }

        return new_world;

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

// #[test]
// fn calculate_standard_world() {
//     let mut markov = Markov::new();

//     markov.world.set_state(State::StartState(0.0), 0, 2);
//     markov.world.set_state(State::ProhibitedState, 1, 1);
//     markov.world.set_state(State::TerminalState(1.0), 3, 0);
//     markov.world.set_state(State::TerminalState(-1.0), 3, 1);

//     markov.evaluate();

//     assert_eq!(Some(&world::State::NormalState(0.812)), markov.world.read_state(0,0));
//     assert_eq!(Some(&world::State::NormalState(0.868)), markov.world.read_state(1,0));
//     assert_eq!(Some(&world::State::NormalState(0.912)), markov.world.read_state(2,0));
//     assert_eq!(Some(&world::State::TerminalState(1.0)), markov.world.read_state(3,0));

//     assert_eq!(Some(&world::State::NormalState(0.762)), markov.world.read_state(0,1));
//     assert_eq!(Some(&world::State::ProhibitedState), markov.world.read_state(1,1));
//     assert_eq!(Some(&world::State::NormalState(0.660)), markov.world.read_state(2,1));
//     assert_eq!(Some(&world::State::TerminalState(-1.0)), markov.world.read_state(3,1));

//     assert_eq!(Some(&world::State::StartState(0.705)), markov.world.read_state(0,2));
//     assert_eq!(Some(&world::State::NormalState(0.655)), markov.world.read_state(1,2));
//     assert_eq!(Some(&world::State::NormalState(0.611)), markov.world.read_state(2,2));
//     assert_eq!(Some(&world::State::NormalState(0.388)), markov.world.read_state(3,2));
// }

#[test]
fn update_normal_state() {
    let mut markov: Markov = Markov::new();

    assert_eq!(Some(&State::NormalState(0.0)), markov.world.read_state(1,1));

    markov.world.set_state(State::NormalState(4.2), 1,1);
    assert_eq!(Some(&State::NormalState(4.2)), markov.world.read_state(1,1));

    markov.world.set_state(State::NormalState(6.6), 1,1);
    assert_eq!(Some(&State::NormalState(6.6)), markov.world.read_state(1,1));
}

// #[test]
// fn not_allow_normal_state_update_from_different_type() {
//     let mut world: Matrix<State> = Matrix::new(State::NormalState(0.0), 4, 3);

//     assert_eq!(true, world.set_state(State::ProhibitedState, 1, 1));
//     assert_eq!(false, world.set_state(State::NormalState(4.2), 1, 1));
// }

// #[test]
// fn not_allow_immutable_state_update() {
//     let mut world: Matrix<State> = Matrix::new(State::NormalState(0.0), 4, 3);

//     assert_eq!(true, world.set_state(State::StartState(0.0), 1, 1));
//     assert_eq!(false, world.set_state(State::StartState(0.0), 1, 1));
// }
