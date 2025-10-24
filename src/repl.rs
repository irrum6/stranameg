pub mod repl {
    pub fn repl_help() {
        let help_str = "
            6 REPL (Interactive) mode
            6.1 REPL commands
            The following commands are available in REPL mode
            .exit or exit or .x - exits application
            run - runs generator and prints strings in stdout (console)
            mode <value> - sets mode to value
            language <value> or lang <value> - sets language to value
            number <value> or num <value> - sets number to value
            length <value> or len <value> - sets length to value

            help or .help - prints this message
            
            6.2 examples
            mode rls
            len 128
            run";

        print!("{}", help_str);
    }

    pub fn run_repl() {
        use std::io::stdin;
        //crate modules
        use crate::stringer::run_generator;
        use crate::stringer::Config;
        use crate::stringer::Modes;
        //repl mode
        println!("Welcome to REPL mode");
        let mut conf = Config::default();
        let mut exit: bool;
        let mut line = String::new();
        // evaluate statements
        loop {
            stdin().read_line(&mut line).unwrap();
            //line trimmed
            let linet = line.trim();

            exit = match linet {
                ".exit" => true,
                "exit" => true,
                ".x" => true,
                _ => false,
            };

            if exit {
                break;
            }

            if "run" == linet {
                match run_generator(&conf) {
                    Ok(_result) => {}
                    Err(e) => {
                        println!("Error:{}", e);
                        break;
                    }
                }
                line.truncate(0);
                continue;
            }

            if "help" == linet|| ".help" == linet {
                repl_help();
                line.truncate(0);
                continue;
            }

            // mode
            if linet.contains("mode") {
                let split: Vec<&str> = linet.split(" ").collect();

                let mode = Modes::from(split[1]);
                println!("{}", split[1]);

                let required_length = match mode {
                    Modes::RandomLettersFromCustomAlphabet => 3,
                    Modes::RandomLettersFromAlphabetFile => 3,
                    Modes::CoupledWordsListFiles => 3,
                    _ => 2,
                };
                if split.len() < required_length {
                    println!("pass other parameters for this mode");
                    line.truncate(0);
                    continue;
                }
                conf.set_mode(mode);

                if required_length > 2 {
                    let next = String::from(split[2]);
                    conf.set_next(next);
                }
                line.truncate(0);
                continue;
            }
            // language
            if linet.contains("lang") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let la = String::from(split[1]);
                conf.set_next(la);
                line.truncate(0);
                continue;
            }
            // number
            if linet.contains("num") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let num: u32 = split[1].parse().expect("number isnot?!");
                conf.set_amount(num);
                line.truncate(0);
                continue;
            }
            // length
            if linet.contains("len") {
                let split: Vec<&str> = line.trim().split(" ").collect();
                let num: u32 = split[1].parse().expect("number isnot?!");
                conf.set_length(num);
                line.truncate(0);
                continue;
            }
            line.truncate(0);
            //run generator
        }
    }
}
