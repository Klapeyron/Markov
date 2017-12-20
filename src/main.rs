#[derive(Debug, PartialEq)]
struct World {
    data: Vec<Vec<Field>>,
    x: usize,
    y: usize
}

impl World {
    pub fn new(xsize: usize, ysize: usize) -> World {
        World {
            data: vec![vec![Field::NormalState(0.0); xsize]; ysize],
            x: xsize,
            y: ysize
        }
    }
    pub fn set_state(self: &mut World, new_state: Field, x: usize, y: usize) {
        let x_index = x - 1;
        let y_index = self.y - y;

        match (&self.data[y_index][x_index], &new_state)
        {
            (&Field::NormalState(_), &Field::NormalState(_)) => {
                println!("Updated value at [{}][{}] from {:?} to {:?}", x, y, &self.data[y_index][x_index], &new_state);
            }
            (&Field::NormalState(_), _) => {
                println!("Created {:?} on position [{}][{}]", &new_state, x, y);
            }
            _ => {
                panic!("Invalid operation {:?} on position [{}][{}]", new_state, x, y);
            }
        }
        self.data[y_index][x_index] = new_state;
    }
    pub fn read_state(self: &World, x: usize, y: usize) -> &Field {
        return &self.data[self.y-y][x-1];
    }
    pub fn walk(self: &World, predicate: &mut FnMut(&Field, usize, usize)) {
        for (x, row) in self.data.iter().enumerate() {
            for (y, column) in row.iter().enumerate() {
                predicate(column, x+1, y+1);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Field {
    ProhibitedState,
    StartState,
    TerminalState(f64),
    SpecialState,
    NormalState(f64)
}

fn evaluate(world: &mut World, world_x: usize, world_y: usize){
    world.set_state(Field::TerminalState(1.0), 4, 3);
}

fn main() {
    let mut world: World = World::new(4, 3);

    world.set_state(Field::StartState, 1, 1);
    world.set_state(Field::ProhibitedState, 2, 2);
    world.set_state(Field::TerminalState(1.0), 4, 3);
    world.set_state(Field::TerminalState(-1.0), 4, 2);

    println!("Hello, {:#?}", world);
}

#[test]
fn correct_indexing() {
    let mut world: World = World::new(4, 3);
    world.set_state(Field::TerminalState(1.0), 4, 3);

    assert_eq!(4, world.x);
    assert_eq!(3, world.y);
}

#[test]
fn save_on_correct_positions() {
    let mut world: World = World::new(4, 3);

    world.set_state(Field::StartState, 1, 1);
    world.set_state(Field::ProhibitedState, 2, 2);
    world.set_state(Field::TerminalState(1.0), 4, 3);
    world.set_state(Field::TerminalState(-1.0), 4, 2);

    assert_eq!(Field::StartState, world.data[2][0]);
    assert_eq!(Field::ProhibitedState, world.data[1][1]);
    assert_eq!(Field::TerminalState(1.0), world.data[0][3]);
    assert_eq!(Field::TerminalState(-1.0), world.data[1][3]);
}

#[test]
fn update_normal_state() {
    let mut world: World = World::new(4, 3);

    assert_eq!(&Field::NormalState(0.0), world.read_state(1,1));

    world.set_state(Field::NormalState(4.2), 1,1);
    assert_eq!(&Field::NormalState(4.2), world.read_state(1,1));

    world.set_state(Field::NormalState(6.6), 1,1);
    assert_eq!(&Field::NormalState(6.6), world.read_state(1,1));
}

#[test]
#[should_panic]
fn not_allow_normal_state_update_from_different_type() {
    let mut world: World = World::new(4, 3);

    world.set_state(Field::ProhibitedState, 1, 1);
    world.set_state(Field::NormalState(4.2), 1, 1);
}

#[test]
#[should_panic]
fn not_allow_immutable_state_update() {
    let mut world: World = World::new(4, 3);

    world.set_state(Field::StartState, 1, 1);
    world.set_state(Field::StartState, 1, 1);
}

#[test]
fn prepare_standard_world() {
    let mut world: World = World::new(4, 3);

    world.set_state(Field::StartState, 1, 1);
    world.set_state(Field::ProhibitedState, 2, 2);
    world.set_state(Field::TerminalState(1.0), 4, 3);
    world.set_state(Field::TerminalState(-1.0), 4, 2);

    assert_eq!(&Field::StartState, world.read_state(1,1));
    assert_eq!(&Field::ProhibitedState, world.read_state(2,2));
    assert_eq!(&Field::TerminalState(1.0), world.read_state(4,3));
    assert_eq!(&Field::TerminalState(-1.0), world.read_state(4,2));
}

#[test]
#[should_panic]
fn panic_if_accessed_out_of_range_x_range() {
    let world: World = World::new(4, 3);
    world.read_state(5,3);
}

#[test]
#[should_panic]
fn panic_if_accessed_out_of_range_y_range() {
    let world: World = World::new(4, 3);
    world.read_state(4,4);
}

#[test]
#[should_panic]
fn panic_if_inserted_out_of_range_x_range() {
    let mut world: World = World::new(4, 3);
    world.set_state(Field::StartState, 5, 3);
}

#[test]
#[should_panic]
fn panic_if_inserted_out_of_range_y_range() {
    let mut world: World = World::new(4, 3);
    world.set_state(Field::StartState, 4, 4);
}

#[test]
fn walk_through_all_fields() {
    let world: World = World::new(4, 3);
    let mut numberOfCalls = 0;

    let mut walk_predicate = |field: &Field, x: usize, y: usize|
    {
        numberOfCalls += 1;
        println!("Iterating over {} {}", x, y);
    };

    world.walk(&mut walk_predicate);
    // assert_eq!(4*3, numberOfCalls);
}
