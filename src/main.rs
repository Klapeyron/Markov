#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io::{self, Read};

mod matrix;
mod markov;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).expect("Invalid input");

    let markov_builder: markov::MarkovBuilder = serde_json::from_str(&buffer).expect("Invalid structure of data");

    let mut markov = markov_builder.finalize();
    let mut number_of_iterations = 0;

    while markov.evaluate() > 0.0001 {
        number_of_iterations += 1;
    }

    println!("Algorithm finished after {} iterations with result: {:#?}", number_of_iterations, markov);
}
