mod matrix;
mod markov;

fn main() {
    let mut world: matrix::Matrix<markov::State> = matrix::Matrix::new(markov::State::NormalState(0.0), 4, 3);

    world.set_state(markov::State::StartState(0.0), 0, 2);
    world.set_state(markov::State::ProhibitedState, 1, 1);
    world.set_state(markov::State::TerminalState(1.0), 3, 0);
    world.set_state(markov::State::TerminalState(-1.0), 3, 1);

    println!("Hello, {:#?}", world);
}
