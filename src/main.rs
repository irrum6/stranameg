use std::env;

use stranameg::stringer::{
    fast_switch, command_parser, print_help, run_generator, run_repl, Config, Modes,
};

fn main() {
    use std::fs::read_to_string;
    let version = "0.10.9";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    // trimmed
    let flag =  args[1].trim();
    // -h -H H Helpt help
    if "-h" == flag || "printh" == flag || "help" == flag {
        print_help();
        return;
    }
    if "--version" == flag || "-V" == flag {
        println!("{}", version);
        return;
    }

    if "repl" == flag || "-R" == flag {
        run_repl();
        return;
    }

    let config = if "-a" == flag || "alt" == flag {
        let mut v = Vec::new();
        for x in 2..args.len() {
            v.push(args[x].as_ref());
        }
        command_parser::get_config(v)
    } else if "pf" == flag || "paramsfile" == flag {
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
        command_parser::get_config(v)
    } else if args[1].contains("-f") {
        // fastswitch
        let stronk = args[1].clone();
        fast_switch::get_fsconf(stronk)
    } else if fast_switch::is_alias(flag){
        let next = String::new();
        let fast = if args.len() >2      {
            fast_switch::alias_config(args[1].clone(), args[2].clone())
        } else {
            fast_switch::alias_config(args[1].clone(), next)
        };
        fast
    }
     else {
        Config::new(&args)
    };

    match run_generator(&config) {
        Ok(_result) => {}
        Err(e) => {
            println!("Error:{}", e);
        }
    }
    return;
}
