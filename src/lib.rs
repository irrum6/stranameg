pub mod help;
pub mod reader;
pub mod rng;

pub mod grammar;
pub mod languages;
pub mod modes;

pub mod command_parser;

pub mod strgen;

pub mod fast_switch;
pub mod repl;

pub mod stringer {
    use std::fs::File;
    use std::io::{Error, Write};

    pub use super::command_parser::command_parser;
    pub use super::grammar::grammar::GermanNounList;
    pub use super::help::help::print_help2 as print_help;
    pub use super::languages::languages::Languages;
    pub use super::modes::modes::Modes;
    pub use super::reader::reader::read_lines;
    pub use super::rng::rng::{RNGWheel, RNG};

    use super::strgen::string_generator_module::*;

    pub use super::fast_switch::fast_switch;
    pub use super::repl::repl::run_repl;

    #[derive(Clone)]
    pub enum ListType {
        Nouns,
        Adjectives,
        Verbs,
        Names,
    }
    impl ListType {
        pub fn is_noun(&self) -> bool {
            return matches!(*self, ListType::Nouns);
        }
    }

    pub fn stringer(conf: Config) -> Box<dyn StringGenerator> {
        let result_box: Box<dyn StringGenerator> = match conf.mode {
            Modes::Password => Box::new(LettterSequence::pass_generator( 16)),
            Modes::Password84 => Box::new(LettterSequence::pass_generator84( 16)),
            Modes::RandomLetters => Box::new(LettterSequence::new("abc", 16)),
            
            Modes::CoupledWordsNouns => Box::new(CoupledWords::new(
                ListType::Nouns,
                Languages::from(conf.next.as_ref()),
            )),
            Modes::CoupledWordsNames => Box::new(CoupledWords::new(
                ListType::Names,
                Languages::from(conf.next.as_ref()),
            )),
            Modes::CoupledWordsListFiles => Box::new(CoupledWords::new(
                ListType::Names,
                Languages::from(conf.next.as_ref()),
            )),
            //for now english only
            Modes::SimpleSentences => Box::new(SimpleSentences::new(Languages::English)),
            _ => Box::new(LettterSequence::new("abc", 16)),
        };
        return result_box;
    }
    pub fn run_generator(conf: &Config) -> Result<(), Error> {
        const OUTPUT_NAME: &str = "strings.textout";
        let mut sg = stringer(conf.clone());
        sg.setup(&conf)?;
        let mut output = File::create(OUTPUT_NAME)?;
        for _i in 0..conf.amount {
            let strang = sg.get();
            if conf.write_to_file {
                writeln!(output, "{}", strang)?;
            } else {
                let mut strong = format!("{}:{}\n", strang, _i);
                if conf.dont_write_indices {
                    strong = format!("{}\n", strang);
                }
                print!("{}\n", strong);
            }
        }
        return Ok(());
    }

    //returns default value as safe, when it can't parse string
    pub fn safe_u32(strong: String, default: u32) -> u32 {
        return match strong.parse() {
            Ok(value) => value,
            Err(_e) => default,
        };
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
        pub fn new(args: &[String]) -> Config {
            return Config::from(args);
        }
        pub fn default() -> Config {
            let amount = 16;
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
                //println!("{}",&args[1]);
                amount = args[1].parse().expect("Number must be");
            }

            if args.len() > 2 {
                println!("{}", &args[2]);
                length = safe_u32(args[2].clone(), 4);
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
