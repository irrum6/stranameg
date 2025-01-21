pub mod repl {
    pub fn run_repl() {
        use std::io::stdin;
        //crate modules
        use crate::stringer::run_generator;
        use crate::stringer::Config;
        use crate::stringer::Modes;
        //repl mode
        println!("Welcome to REPL mode");
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
            // mode
            if line.trim().contains("mode") {
                let split: Vec<&str> = line.trim().split(" ").collect();

                let mode = Modes::from(split[1]);
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
    }
}
