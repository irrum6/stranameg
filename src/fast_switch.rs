pub mod fast_switch {
    use crate::stringer::Config;
    use crate::stringer::Modes;
    pub fn get_fsconf(onkstr: String) -> Config {
        let mut conf = Config::default();
        let split: Vec<&str> = onkstr.split("-f").collect();
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
        return conf;
    }

    pub fn alias_config(strung: String, next: String) -> Config {
        let mut conf = Config::default();
        let option = strung.trim();
        let required_length = match option {
            "R2" | "R3" | "C1" | "C2" | "C3" | "W1" | "W2" => 1,
            _ => 0,
        };

        if next.len() < required_length {
            println!("one paramter missing, returning defualt config");
            return conf;
        }
        match option {
            // mode shortcuts
            "R1" => {
                conf.set_mode(Modes::RandomLetters);
            }
            "R2" => {
                conf.set_mode(Modes::RandomLettersFromCustomAlphabet);
                conf.set_next(next);
            }
            "R3" => {
                conf.set_mode(Modes::RandomLettersFromAlphabetFile);
                conf.set_next(next);
            }
            "C1" => {
                conf.set_mode(Modes::CoupledWordsNouns);
                conf.set_next(next);
            }
            "C2" => {
                conf.set_mode(Modes::CoupledWordsNames);
                conf.set_next(next);
            }
            "C3" => {
                conf.set_mode(Modes::CoupledWordsListFiles);
                conf.set_next(next);
            }
            
            "N1" => {
                conf.set_mode(Modes::SimpleSentences);
                conf.set_next(next);
            }
            // length shortcuts
            "S1" => {
                conf.set_length(16);
            }
            "S2" => {
                conf.set_length(32);
            }
            "S3" => {
                conf.set_length(48);
            }
            "S4" => {
                conf.set_length(64);
            }
            "S5" => {
                conf.set_length(80);
            }
            "S6" => {
                conf.set_length(96);
            }
            "S7" => {
                conf.set_length(112);
            }
            "S8" => {
                conf.set_length(128);
            }
            "S9" => {
                conf.set_length(144);
            }
            "SX" => {
                conf.set_length(160);
            }
            "PWD" =>{
                conf.set_mode(Modes::Password);
            }
            "PWD84" =>{
                conf.set_mode(Modes::Password84);
            }
            _ => {}
        }
        return conf;
    }

    pub fn is_alias(strong: &str) -> bool {
        return match strong {
            "R1" | "R2" | "R3" | "C1" | "C2" | "C3" | "N1" | "S1" | "S2" | "S3"
            | "S4" | "S5" | "S6" | "S7" | "S8" | "S9" | "SX" | "PWD" | "PWD84" => true,
            _ => false,
        };
    }
}
