use std::env;
use std::fs::read_to_string as fs_read;

mod parse;
mod rng;
mod strgen;
mod tests;

use rng::rng::{RNGWheel, RNG};

use strgen::strgen::{
    AlphaBetStringGenerator as ABCGenerator, Languages, ListStringGenerator as ListGenerator,
    ListType, StringGenerator,
};

use parse::parse::{fill as fill_list, fill2 as fill_list2};

pub fn lang_mapper(s: &String) -> Languages {
    let result = match s.as_ref() {
        "11" => Languages::Georgian,
        "12" => Languages::English,
        _ => Languages::English,
    };
    return result;
}

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
    let lan = lang_mapper(&next);
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

fn mode3x(amount: u32, mode: u32, next: String) {
    let lan = lang_mapper(&next);

    let list_typ1 = ListType::Adjectives;
    let mut list_typ2 = ListType::Nouns;

    if mode == 32 {
        list_typ2 = ListType::Names;
    }
    let mut lsg1 = ListGenerator::new(list_typ1, lan.clone());
    let mut lsg2 = ListGenerator::new(list_typ2, lan);

    if mode == 31 || mode == 32 {
        fill_list(&mut lsg1);
        fill_list(&mut lsg2);
    }
    if mode == 33 {
        let names: Vec<&str> = next.split(":").collect();
        let name0 = String::from(names[0]);
        let name1 = String::from(names[1]);
        fill_list2(&mut lsg1, name0);
        fill_list2(&mut lsg2, name1);
    }
    for _i in 0..amount {
        let strang = lsg1.get_single_word();
        let strang2 = lsg2.get_single_word();
        let strong = format!("{}_{}", strang, strang2);
        print!("{}:{}\n", strong, _i);
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
