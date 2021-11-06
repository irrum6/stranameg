use std::env;
use std::io::Error;

mod help;
mod modes;
mod parse;
mod rng;
mod strgen;
mod tests;

use help::help::print_help;
use modes::modes::*;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::{
    AlphaBetStringGenerator as ABCGenerator, Languages, ListStringGenerator as ListGenerator,
    ListType, StringGenerator,
};

use parse::parse::{fill as fill_list, fill2 as fill_list2};

fn run_generator(len: u32, amount: u32, mode: u32, next: String) -> Result<(), Error> {
    if mode / 10 == 1 || (mode > 99 && mode / 100 == 1) {
        return mode1x(len, amount, mode, next);
    }
    if mode / 10 == 2 || (mode > 99 && mode / 100 == 2) {
        return mode2x(len, amount, mode, next);
    }
    if mode / 10 == 3 || (mode > 99 && mode / 100 == 3) {
        return mode3x(amount, mode, next);
    }
    return Ok(());
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return Ok(());
    }
    if args[1] == "-h" {
        print_help();
        return Ok(());
    }

    let amount = args[1].trim().parse().expect("type a number");

    let mut length: u32 = 12;
    let mut mode: u32 = 10;
    let mut next: String = String::new();

    if args.len() > 2 {
        length = args[2].trim().parse().expect("type a number");
    }

    if args.len() > 3 {
        mode = args[3].trim().parse().expect("type a number");
    }
    if args.len() > 4 {
        next = args[4].clone();
    }
    return run_generator(length, amount, mode, next);
}
