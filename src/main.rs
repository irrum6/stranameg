use std::env;

use stranameg::stringer::{
    command_parser, fast_switch, print_help, run_generator, run_repl, Config,
};

fn main() {
    use std::fs::read_to_string;
    const VERSION: &str = "0.14.4";

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    // trimmed
    let flag = args[1].trim();
    // -h -H H Helpt help
    if "-h" == flag || "printh" == flag || "help" == flag {
        print_help();
        return;
    }
    if "--version" == flag || "-V" == flag {
        println!("{}", VERSION);
        return;
    }

    if "repl" == flag || "-R" == flag {
        run_repl();
        return;
    }

    let config = match flag {
        "-a" | "alt" => {
            let mut v = Vec::new();
            for x in 2..args.len() {
                v.push(args[x].as_ref());
            }
            command_parser::get_config(v)
        }
        "pf" | "paramsfile" => {
            let mut v = Vec::new();
            let mut fileargs = String::new();
            if args.len() > 2 {
                fileargs = match read_to_string(&args[2]) {
                    Ok(content) => content,
                    Err(e) => panic!("{}", e),
                };
            }
            for line in fileargs.lines() {
                v.push(line.trim());
            }
            command_parser::get_config(v)
        }
        _ => {
            if flag.contains("-f") {
                // fastswitch
                let stronk = String::from(flag);
                fast_switch::get_fsconf(stronk)
            } else if fast_switch::is_alias(flag) {
                let next = String::new();
                let fast = if args.len() > 2 {
                    fast_switch::alias_config(args[1].clone(), args[2].clone())
                } else {
                    fast_switch::alias_config(args[1].clone(), next)
                };
                fast
            } else {
                Config::new(&args)
            }
        }
    };

    match run_generator(&config) {
        Ok(_result) => {}
        Err(e) => {
            println!("Error:{}", e);
        }
    }
    return;
}
