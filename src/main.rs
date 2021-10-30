use std::env;

mod parse;
mod rng;
mod strgen;
mod tests;
mod modes;

use modes::modes::*;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::{
    AlphaBetStringGenerator as ABCGenerator, Languages, ListStringGenerator as ListGenerator,
    ListType, StringGenerator,
};

use parse::parse::{fill as fill_list, fill2 as fill_list2};

fn run_generator(len: u32, amount: u32, mode: u32, next: String) {
    if mode / 10 == 1 {
        mode1x(len, amount, mode, next);
        return;
    }
    if mode / 10 == 2 {
        mode2x(len, amount, mode, next);
        return;
    }
    if mode / 10 == 3 {
        mode3x(amount, mode, next);
        return;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        return;
    }
    let amount: u32 = args[1].trim().parse().expect("type a number");

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
    run_generator(length, amount, mode, next);
}
