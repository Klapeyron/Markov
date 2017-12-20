#[derive(Debug, PartialEq)]
pub struct World {
    data: Vec<Vec<State>>,
    x: usize,
    y: usize,
    out_of_range: State
}

impl World {
    pub fn new(xsize: usize, ysize: usize) -> World {
        World {
            data: vec![vec![State::NormalState(0.0); xsize]; ysize],
            x: xsize,
            y: ysize,
            out_of_range: State::ProhibitedState
        }
    }

    pub(self) fn is_in_range(self: &World, x: usize, y: usize) -> bool {
        return x < self.x && y < self.y;
    }

    pub fn set_state(self: &mut World, new_state: State, x: usize, y: usize) {
        match (&self.data[y][x], &new_state)
        {
            (&State::NormalState(_), &State::NormalState(_)) => {
                println!("Updated value at [{}][{}] from {:?} to {:?}", x, y, &self.data[y][x], &new_state);
            }
            (&State::NormalState(_), _) => {
                println!("Created {:?} on position [{}][{}]", &new_state, x, y);
            }
            _ => {
                panic!("Invalid operation {:?} on position [{}][{}]", new_state, x, y);
            }
        }
        self.data[y][x] = new_state;
    }

    pub fn read_state(self: &World, x: usize, y: usize) -> &State {
        if self.is_in_range(x, y) {
            return &self.data[y][x];
        }
        else {
            return &self.out_of_range;
        }
    }

    pub fn walk(self: &World, predicate: &mut FnMut(&State, usize, usize)) {
        for (x, row) in self.data.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                predicate(column, x, y);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum State {
    ProhibitedState,
    StartState,
    TerminalState(f64),
    SpecialState,
    NormalState(f64)
}

#[test]
fn correct_indexing() {
    let mut world: World = World::new(4, 3);
    world.set_state(State::TerminalState(1.0), 3, 2);

    assert_eq!(4, world.x);
    assert_eq!(3, world.y);
}

#[test]
fn update_normal_state() {
    let mut world: World = World::new(4, 3);

    assert_eq!(&State::NormalState(0.0), world.read_state(1,1));

    world.set_state(State::NormalState(4.2), 1,1);
    assert_eq!(&State::NormalState(4.2), world.read_state(1,1));

    world.set_state(State::NormalState(6.6), 1,1);
    assert_eq!(&State::NormalState(6.6), world.read_state(1,1));
}

#[test]
#[should_panic]
fn not_allow_normal_state_update_from_different_type() {
    let mut world: World = World::new(4, 3);

    world.set_state(State::ProhibitedState, 1, 1);
    world.set_state(State::NormalState(4.2), 1, 1);
}

#[test]
#[should_panic]
fn not_allow_immutable_state_update() {
    let mut world: World = World::new(4, 3);

    world.set_state(State::StartState, 1, 1);
    world.set_state(State::StartState, 1, 1);
}

#[test]
fn prepare_standard_world() {
    let mut world: World = World::new(4, 3);

    world.set_state(State::StartState, 0, 2);
    world.set_state(State::ProhibitedState, 1, 1);
    world.set_state(State::TerminalState(1.0), 3, 0);
    world.set_state(State::TerminalState(-1.0), 3, 1);

    assert_eq!(&State::StartState, world.read_state(0,2));
    assert_eq!(&State::ProhibitedState, world.read_state(1,1));
    assert_eq!(&State::TerminalState(1.0), world.read_state(3,0));
    assert_eq!(&State::TerminalState(-1.0), world.read_state(3,1));
}

#[test]
fn return_prohibited_state_if_accessed_out_of_range() {
    let world: World = World::new(4, 3);
    assert_eq!(&State::ProhibitedState, world.read_state(5,3));
    assert_eq!(&State::ProhibitedState, world.read_state(4,4));
    assert_eq!(&State::ProhibitedState, world.read_state(4,4));
}

#[test]
#[should_panic]
fn panic_if_inserted_out_of_range_x_range() {
    let mut world: World = World::new(4, 3);
    world.set_state(State::StartState, 5, 3);
}

#[test]
#[should_panic]
fn panic_if_inserted_out_of_range_y_range() {
    let mut world: World = World::new(4, 3);
    world.set_state(State::StartState, 4, 4);
}

#[test]
fn walk_through_all_fields() {
    let world: World = World::new(4, 3);
    let mut numberOfCalls = 0;

    let mut walk_predicate = |_field: &State, x: usize, y: usize|
    {
        numberOfCalls += 1;
        println!("Iterating over {} {}", x, y);
    };

    world.walk(&mut walk_predicate);
    // assert_eq!(4*3, numberOfCalls);
}
