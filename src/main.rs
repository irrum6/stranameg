use std::env;

use stranameg::stringer::{get_config_from_commands, print_help, run_generator, Config};

fn main() {
    use std::fs::read_to_string;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    if "-h" == args[1] || "printh" == args[1] {
        print_help();
        return;
    }

    let config = if "-a" == args[1] || "alt" == args[1] {
        let mut v = Vec::new();
        for x in 2..args.len() {
            v.push(args[x].as_ref());
        }
        get_config_from_commands(v)
    } else if "pf" == args[1] || "paramsfile" == args[1] {
        let mut v = Vec::new();
        let mut fileargs = String::from(".params");
        if args.len() > 2 {
            fileargs = match read_to_string(&args[2]) {
                Ok(content) => content,
                Err(e) => panic!("{}", e),
            };
        }
        for line in fileargs.lines() {
            v.push(line.trim());
        }
        get_config_from_commands(v)
    } else {
        Config::new(&args)
    };

    match run_generator(config) {
        Ok(_result) => {}
        Err(e) => {
            println!("Error:{}", e);
        }
    }
    return;
}
