use world;

struct Markov {
    world: world::World,
    gama: f64,
    cost_of_move: f64
}

#[derive(Debug, Clone, PartialEq)]
enum Action {
    Up,
    Down,
    Left,
    Right
}

impl Markov {
    fn new() -> Markov {
        Markov {
            world: world::World::new(4,3),
            gama: 0.98,
            cost_of_move: -0.04
        }
    }

    fn model(state: &world::State, action: &Action, new_state: &world::State) -> f64 {
        match action {
            &Action::Up => 0.8,
            &Action::Left => 0.1,
            &Action::Right => 0.1,
            &Action::Down => 0.0
        }
    }

    fn evaluate(world: &mut world::World, x: usize, y: usize) {
        let turnLeft  = world.read_state(x.checked_sub(1).unwrap_or(0), y);
        let turnRight = world.read_state(x.checked_add(1).unwrap_or(0), y);
        let turnUp    = world.read_state(x, y.checked_add(1).unwrap_or(0));
        let turnDown  = world.read_state(x, y.checked_sub(1).unwrap_or(0));

        // U(s) = R(s) + gama max{T(s,a,s')U(s')}
    }
}
