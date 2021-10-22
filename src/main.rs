use std::env;

mod rng;
use rng::rng::RNGWheel;
mod strgen;
mod tests;

use strgen::strgen::StringGenerator;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        return;
    }
    let amount: u32 = args[1].trim().parse().expect("type a number");

    let mut length: u32 = 12; //default 12 character length password

    if args.len() > 2 {
        length = args[2].trim().parse().expect("type a number");
    }

    let latinchars = "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy";
    let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ";

    let mut charski: Vec<char> = kachars.chars().collect();

    if args.len() > 3 {
        let mode: u32 = args[3].trim().parse().expect("type a number");
        if mode == 2 {
            charski = latinchars.chars().collect();
        }
    }

    let mut sg = StringGenerator::new(charski);

    for _i in 0..amount {
        let strang = sg.get(length as usize);
        print!("{}:{}\n", strang, _i);
    }
}
