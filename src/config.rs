pub mod config {
    use crate::stringer::safe_u32;

    #[derive(Clone)]
    pub enum Modes {
        Password,
        Password84,
        RandomLetters,
        RandomLettersFromCustomAlphabet,
        RandomLettersFromAlphabetFile,
        CoupledWordsNouns,
        CoupledWordsNames,
        CoupledWordsListFiles,
    }
    impl Modes {
        pub fn from(s: &str) -> Modes {
            return match s {
                "pass" => Modes::Password,
                "pass2" => Modes::Password84,
                "rls" => Modes::RandomLetters,
                "rla" => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" => Modes::RandomLettersFromAlphabetFile,
                "cow" | "cwo" => Modes::CoupledWordsNouns,
                "cowe" | "cwe" => Modes::CoupledWordsNames,
                "cowf" | "cwf" => Modes::CoupledWordsListFiles,
                _ => Modes::RandomLetters,
            };
        }
    }

    #[derive(Clone)]
    pub struct Config {
        mode: Modes,
        length: u32,
        amount: u32,
        write_to_file: bool,
        dont_write_indices: bool,
        next: String,
    }
    impl Config {
        pub fn default() -> Config {
            let amount = 8;
            let mode = Modes::RandomLetters;
            let write_to_file = false;
            let dont_write_indices = false;

            let next = String::new();

            let length: u32 = 12;
            return Config {
                mode,
                length,
                amount,
                write_to_file,
                next,
                dont_write_indices,
            };
        }
        pub fn from(args: &[String]) -> Config {
            let mut amount = 16;
            let mut mode = Modes::RandomLetters;
            let mut write_to_file = false;
            let mut dont_write_indices = false;

            let mut next = String::new();

            let mut length: u32 = 12;

            if args.len() > 1 {
                amount = safe_u32(args[1].clone(), 16);
            }

            if args.len() > 2 {
                //println!("{}", &args[2]);
                length = safe_u32(args[2].clone(), 12);
            }
            if args.len() > 3 {
                mode = Modes::from(args[3].as_ref());
            }
            if args.len() > 4 {
                next = args[4].clone();
            }

            if args.len() > 5 {
                write_to_file = args[5] == "1";
            }

            if args.len() > 6 {
                dont_write_indices = args[6] == "1";
            }
            return Config {
                mode,
                length,
                amount,
                write_to_file,
                next,
                dont_write_indices,
            };
        }
        pub fn set_mode(&mut self, mode: Modes) {
            self.mode = mode;
        }
        pub fn get_mode(&self) -> Modes {
            return self.mode.clone();
        }

        pub fn set_length(&mut self, length: u32) {
            self.length = length;
        }
        pub fn get_length(&self) -> u32 {
            return self.length;
        }

        pub fn set_amount(&mut self, amount: u32) {
            self.amount = amount;
        }
        pub fn get_amount(&self) -> u32 {
            return self.amount;
        }

        pub fn set_write_to_file(&mut self, wtf: bool) {
            self.write_to_file = wtf;
        }
        pub fn get_write_to_file(&self) -> bool {
            return self.write_to_file;
        }

        pub fn set_write_indices(&mut self, dwi: bool) {
            self.dont_write_indices = dwi;
        }
        pub fn get_write_indices(&self) -> bool {
            return self.dont_write_indices;
        }

        pub fn set_next(&mut self, next: String) {
            self.next = next;
        }
        pub fn get_next(&self) -> String {
            return self.next.clone();
        }
    }
}
