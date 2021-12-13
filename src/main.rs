use std::env;

mod help;
mod parse;
mod rng;
mod strgen;
mod tests;

use help::help::print_help;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::run_generator;
use strgen::strgen::{Config, LettterSequence, StringGenerator,RandomWord,ListType,Languages};

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
    run_generator(config);

    return;
}

use std::fs::File;
use std::io::{self, BufRead, ErrorKind};
use std::path::Path;

//copied from rust site and modified
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    match File::open(filename) {
        Ok(file) => return Ok(io::BufReader::new(file).lines()),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("File not found"),
            _ => panic!("Error when opening file"),
        },
    };
}
