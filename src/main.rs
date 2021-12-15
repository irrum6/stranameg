use std::env;

use stranameg::stringer::{print_help, run_generator, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    if "-h"== args[1] || "printh" == args[1]  {
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
