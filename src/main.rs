mod world;
mod markov;

fn main() {
    let mut world: world::World = world::World::new(4, 3);

    world.set_state(world::State::StartState, 0, 2);
    world.set_state(world::State::ProhibitedState, 1, 1);
    world.set_state(world::State::TerminalState(1.0), 3, 0);
    world.set_state(world::State::TerminalState(-1.0), 3, 1);

    println!("Hello, {:#?}", world);
}
