use std::env;

use stranameg::stringer::{
    get_config_from_commands, print_help, run_generator, Config, Languages, Modes,
};

fn main() {
    use std::fs::read_to_string;
    let version = "0.10.1";
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
        use std::io::stdin;
        //repl mode
        println!("Welcome to repl mode");
        let mut conf = Config::default();
        let mut exit = false;
        let mut line = String::new();
        // evaluate statements
        loop {
            if exit {
                break;
            }
            stdin().read_line(&mut line).unwrap();
            if ".exit" == line.trim() {
                exit = true;
                line.truncate(0);
                continue;
            }

            if "run" == line.trim() {
                run_generator(conf.clone());
                line.truncate(0);
                continue;
            }
            // mode
            if line.trim().contains("mode") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let mode = Modes::from(split[1]);
                conf.set_mode(mode);
                line.truncate(0);
                continue;
            }
            // language
            if line.trim().contains("lang") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let la = String::from(split[1]);
                conf.set_next(la);
                line.truncate(0);
                continue;
            }
            // number
            if line.trim().contains("num") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let num: u32 = split[1].parse().expect("number isnot?!");
                conf.set_amount(num);
                line.truncate(0);
                continue;
            }
            // length
            if line.trim().contains("len") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let num: u32 = split[1].parse().expect("number isnot?!");
                conf.set_length(num);
                line.truncate(0);
                continue;
            }
            //run generator
        }
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
        let mut conf = Config::default();
        let split: Vec<&str> = args[1].split("-f").collect();
        // println!("{}",split[0]);
        let s: String = String::from(split[1]);
        let zero = s.chars().nth(0).unwrap();
        if 'n' == zero {
            // number
            // -fn16
            let split: Vec<&str> = s.split("n").collect();
            let num: u32 = split[1].parse().expect("number isnot?!");
            conf.set_amount(num);
        } else if 's' == zero {
            // length/size
            // -fs32
            let split: Vec<&str> = s.split("s").collect();
            let num: u32 = split[1].parse().expect("number isnot?!");
            conf.set_length(num);
        } else if 'm' == zero {
            // mode
            // -fmrla
            let split: Vec<&str> = s.split("m").collect();
            let mode = Modes::from(split[1]);
            conf.set_mode(mode);
        } else if 'l' == zero {
            // language
            // -flka
            let split: Vec<&str> = s.split("l").collect();
            let la = String::from(split[1]);
            conf.set_next(la);
        } else {
            // do nothing
        }
        conf
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
