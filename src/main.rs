use std::env;

use stranameg::stringer::{
    fast_switch, get_config_from_commands, print_help, run_generator, run_repl, Config, Modes,
};

fn main() {
    use std::fs::read_to_string;
    let version = "0.10.5";
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("pass enough parameters to calculate");
        print_help();
        return;
    }
    // -h -H H Helpt help
    if "-h" == args[1] || "printh" == args[1] || "help" == args[1] {
        print_help();
        return;
    }
    if "--version" == args[1] || "-V" == args[1] {
        println!("{}", version);
        return;
    }

    if "repl" == args[1] || "-R" == args[1] {
        run_repl();
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
    } else if args[1].contains("-f") {
        // fastswitch
        let stronk = args[1].clone();
        fast_switch::get_fswitch_conf(stronk)
    } else {
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
