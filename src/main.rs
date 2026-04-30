use stranameg::stringer::{
    fast_switch, print_help, run_generator, run_repl, Config,CommandParser
};

fn main() {
    use std::env;
    use std::fs::read_to_string;
    const VERSION: &str = "0.15.1";

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    // trimmed
    let flag = args[1].trim();
    // -h -H H Helpt help
    if flag == "-H" || flag == "--help" {
        print_help();
        return;
    }
    if flag == "--version" || flag == "-V" {
        println!("{}", VERSION);
        return;
    }

    if flag == "repl" || flag == "-R" {
        run_repl();
        return;
    }

    let config = match flag {
        "-a" | "alt" => {
            let mut v = Vec::new();
            for x in 2..args.len() {
                v.push(args[x].as_ref());
            }
            CommandParser::parse_config(v)
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
            CommandParser::parse_config(v)
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
                Config::from(&args)
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
