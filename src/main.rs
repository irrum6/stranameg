use std::env;

// mod reader;
mod rng;
mod strgen;
mod tests;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::{run_generator, Config};

use stranameg::stringer::print_help;
use stranameg::stringer::read_lines;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    if args[1] == "-h" {
        print_help();
        return;
    }
    let config = Config::new(&args);
    match run_generator(config) {
        Ok(_result) => {}
        Err(e) => {
            println!("Error:{}", e);
        }
    }

    return;
}
