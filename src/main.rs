use std::env;

mod rng;
use rng::rng::RNGWheel;
mod strgen;
mod tests;

use strgen::strgen::AlphaBetStringGenerator as ABCGenerator;
use strgen::strgen::StringGenerator;

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
        if mode == 10 && args.len() > 4 && args[4] == "11" {
            charski = kachars.chars().collect();
        }
        if mode == 11 && args.len() > 4 {
            charski = args[4].trim().chars().collect();
        }
    }

    let mut sg = ABCGenerator::new(charski);

    for _i in 0..amount {
        let strang = sg.get(length as usize);
        print!("{}:{}\n", strang, _i);
    }
}
