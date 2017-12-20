use world;

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Up,
    Down,
    Left,
    Right
}

fn model(_state: world::State, action: Action, _new_state: world::State) -> f64 {
    match action {
        Action::Up => 0.8,
        Action::Left => 0.1,
        Action::Right => 0.1,
        Action::Down => 0.0
    }
}

fn evaluate(world: &mut world::World, x: usize, y: usize){
    let world_size_x = 4;
    let world_size_y = 3;
}
