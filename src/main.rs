use std::env;
use std::fs::read_to_string as fs_read;

mod parse;
mod rng;
mod strgen;
mod tests;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::{
    AlphaBetStringGenerator as ABCGenerator, CoupledWordsGenerator as Coupler, Languages,
    ListStringGenerator as ListGenerator, ListType, StringGenerator,
};

use parse::parse::{fill as fill_list, fill2 as fill_list2, fill_coupled, fill_coupled2};
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
        mode3x(len, amount, mode, next);
        return;
    }
}

fn mode1x(len: u32, amount: u32, mode: u32, next: String) {
    let latinchars = "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy";
    let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ";
    let mut charski: Vec<char> = latinchars.chars().collect();
    if mode == 10 && next == "11" {
        charski = kachars.chars().collect();
    }
    if mode == 11 {
        charski = next.trim().chars().collect();
    }
    if mode == 12 {
        let contents = fs_read(next.clone().trim()).expect("Something went wrong reading the file");
        charski = contents.chars().collect();
    }
    let mut sg = ABCGenerator::new(charski);

    for _i in 0..amount {
        let strang = sg.get(len as usize);
        print!("{}:{}\n", strang, _i);
    }
}
fn mode2x(len: u32, amount: u32, mode: u32, next: String) {
    let lst = ListType::Nouns;
    let lan = match next.as_ref() {
        "11" => Languages::Georgian,
        "12" => Languages::English,
        _ => Languages::English,
    };
    let mut lsg = ListGenerator::new(lst, lan);

    if mode == 21 {
        fill_list(&mut lsg);
    }
    if mode == 22 {
        fill_list2(&mut lsg, next.clone());
    }
    for _i in 0..amount {
        let strang = lsg.get(len as usize);
        print!("{}:{}\n", strang, _i);
    }
    return;
}

fn mode3x(len: u32, amount: u32, mode: u32, next: String) {
    let lan = match next.as_ref() {
        "11" => Languages::Georgian,
        "12" => Languages::English,
        _ => Languages::English,
    };
    let mut cw = Coupler::new(lan);

    if mode == 31 {
        fill_coupled(&mut cw);
    }
    if mode == 32 {
        let names: Vec<&str> = next.split(":").collect();
        let name0 = String::from(names[0]);
        let name1 = String::from(names[1]);
        fill_coupled2(&mut cw, name0, name1);
    }
    for _i in 0..amount {
        let strang = cw.get(len as usize);
        print!("{}:{}\n", strang, _i);
    }
    return;
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
