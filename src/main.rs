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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        return;
    }
    let amount: u32 = args[1].trim().parse().expect("type a number");

    let mut length: u32 = 12;

    if args.len() > 2 {
        length = args[2].trim().parse().expect("type a number");
    }

    let latinchars = "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy";
    let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ";

    let mut charski: Vec<char> = latinchars.chars().collect();

    if args.len() > 3 {
        let mode: u32 = args[3].trim().parse().expect("type a number");
        //mode 1x randome letter string generator
        if mode == 10 && args.len() > 4 && args[4] == "11" {
            charski = kachars.chars().collect();
        }
        if mode == 11 && args.len() > 4 {
            charski = args[4].trim().chars().collect();
        }
        if mode == 12 && args.len() > 4 {
            let contents =
                fs_read(args[4].clone().trim()).expect("Something went wrong reading the file");
            charski = contents.chars().collect();
        }

        //mode 2x list string generator
        if mode == 21 && args.len() > 4 && args[4] == "12" {
            let lst = ListType::Nouns;
            let lan = Languages::English;
            let mut lsg = ListGenerator::new(lst, lan);
            fill_list(&mut lsg);

            for _i in 0..amount {
                let strang = lsg.get(length as usize);
                print!("{}:{}\n", strang, _i);
            }
            return;
        }
        if mode == 22 && args.len() > 4 {
            let lst = ListType::Nouns;
            let lan = Languages::English;
            let mut lsg = ListGenerator::new(lst, lan);
            fill_list2(&mut lsg, args[4].clone());
            for _i in 0..amount {
                let strang = lsg.get(length as usize);
                print!("{}:{}\n", strang, _i);
            }
            return;
        }
    }

    let mut sg = ABCGenerator::new(charski);

    for _i in 0..amount {
        let strang = sg.get(length as usize);
        print!("{}:{}\n", strang, _i);
    }
}
