pub mod help;
pub mod reader;
pub mod rng;

pub mod grammar;
pub mod languages;
pub mod modes;

pub mod command_parser;

pub mod stringer {
    use std::fs::read_to_string as fs_read;
    use std::fs::File;
    use std::io::{Error, Write};

    pub use super::command_parser::command_parser::get_config_from_commands;
    pub use super::grammar::grammar::GermanNounList;
    pub use super::help::help::print_help2 as print_help;
    pub use super::languages::languages::Languages;
    pub use super::modes::modes::Modes;
    pub use super::reader::reader::read_lines;
    pub use super::rng::rng::{RNGWheel, RNG};

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
    pub trait StringGenerator {
        fn get(&mut self) -> String;
        fn setup(&mut self, conf: Config);
    }
    pub struct LettterSequence {
        alphabet: Vec<char>,
        held_string: String,
        length: usize,
    }

    impl LettterSequence {
        pub fn new(s: &str, length: usize) -> LettterSequence {
            let held_string = String::new();
            let alphabet: Vec<char> = s.chars().collect();
            return LettterSequence {
                held_string,
                alphabet,
                length,
            };
        }
        pub fn set_alphabet(&mut self, s: &str) {
            let abc: Vec<char> = s.chars().collect();
            self.alphabet = abc;
        }
        pub fn set_length(&mut self, n: usize) {
            self.length = n;
        }
    }
    impl StringGenerator for LettterSequence {
        fn get(&mut self) -> String {
            let rng = RNGWheel::new(self.length);
            let len = self.alphabet.len();
            self.held_string = String::new();
            for num in rng {
                let index = num as usize % len;
                self.held_string.push(self.alphabet[index]);
            }
            return self.held_string.clone();
        }
        fn setup(&mut self, conf: Config) {
            match conf.mode {
                Modes::RandomLettersFromCustomAlphabet => {
                    self.set_alphabet(conf.next.as_ref());
                }
                Modes::RandomLettersFromAlphabetFile => {
                    let contents =
                        fs_read(conf.next).expect("Something went wrong reading the file");
                    self.set_alphabet(contents.as_ref());
                }
                Modes::RandomLetters => {
                    let lang = Languages::from(conf.next.as_ref());
                    let alphabet = lang.get_alphabet();
                    self.set_alphabet(alphabet.as_ref());
                }
                _ => {}
            }
            self.set_length(conf.length as usize);
        }
    }
    pub struct RandomWord {
        list: Vec<String>,
        list_type: ListType,
        language: Languages,
    }

    impl RandomWord {
        pub fn new(list_type: ListType, language: Languages) -> RandomWord {
            let list: Vec<String> = Vec::new();
            return RandomWord {
                list,
                list_type,
                language,
            };
        }
        pub fn add_word(&mut self, s: String) {
            self.list.push(s);
        }
        pub fn get_language(&self) -> Languages {
            return self.language.clone();
        }
        pub fn get_list_type(&self) -> ListType {
            return self.list_type.clone();
        }
        pub fn get_file_name(&self) -> String {
            let list_type = &self.get_list_type();
            let lang = &self.get_language();
            let head = match list_type {
                ListType::Nouns => "nouns",
                ListType::Adjectives => "adjectives",
                ListType::Verbs => "verbs",
                ListType::Names => "names",
            };
            let lang = lang.abbr();
            return format!("./lists/{}.{}.list", head, lang);
        }
        pub fn get_list_len(&self) -> usize {
            return self.list.len();
        }
        pub fn fill(&mut self, s: &str) {
            let filename = if s == "" {
                self.get_file_name()
            } else {
                String::from(s)
            };
            if let Ok(lines) = read_lines(filename) {
                for line in lines {
                    if let Ok(ip) = line {
                        let chazar = ip.split(",");
                        for chaz in chazar {
                            if chaz == "" {
                                continue;
                            }
                            self.add_word(String::from(chaz.trim()))
                        }
                    }
                }
            }
        }
    }
    impl StringGenerator for RandomWord {
        fn get(&mut self) -> String {
            let mut rng = RNG::new();
            rng.seed();
            let diclen = self.list.len();
            let index = rng.get() as usize % diclen;
            return self.list[index].clone();
        }
        fn setup(&mut self, conf: Config) {
            match conf.mode {
                Modes::RandomWord => self.fill(""),
                Modes::RandomWordFromListFile => self.fill(conf.next.as_ref()),
                _ => self.fill(""),
            }
        }
    }
    pub struct CoupledWords {
        adjectives: RandomWord,
        second_type: ListType,
        language: Languages,
        type_list: RandomWord,
    }
    impl CoupledWords {
        fn new(second_type: ListType, language: Languages) -> CoupledWords {
            let adjectives = RandomWord::new(ListType::Adjectives, language.clone());
            let type_list = RandomWord::new(second_type.clone(), language.clone());
            return CoupledWords {
                adjectives,
                second_type,
                language,
                type_list,
            };
        }
    }
    impl StringGenerator for CoupledWords {
        fn get(&mut self) -> String {
            let mut nounlist: GermanNounList = GermanNounList::new();
            if self.language.is_german() {
                nounlist.fill();
            }
            let adj = self.adjectives.get();
            let s2 = self.type_list.get();

            let mut strong = format!("{}_{}", adj, s2);

            if self.language.is_german() && self.second_type.is_noun() {
                //noun adjective
                strong = nounlist.get_adapted(s2, adj);
            }
            return strong;
        }
        fn setup(&mut self, conf: Config) {
            match conf.mode {
                Modes::CoupledWordsNouns | Modes::CoupledWordsNames => {
                    self.adjectives.fill("");
                    self.type_list.fill("");
                }
                Modes::CoupledWordsListFiles => {
                    let names: Vec<&str> = conf.next.split(":").collect();
                    self.adjectives.fill(names[0]);
                    self.type_list.fill(names[1]);
                }
                _ => {}
            }
        }
    }

    pub struct SimpleSentences {
        adjectives: RandomWord,
        nouns: RandomWord,
        verbs: RandomWord,
        language: Languages,
    }

    impl SimpleSentences {
        pub fn new(language: Languages) -> SimpleSentences {
            let adjectives = RandomWord::new(ListType::Adjectives, language.clone());
            let nouns = RandomWord::new(ListType::Nouns, language.clone());
            let verbs = RandomWord::new(ListType::Verbs, language.clone());
            return SimpleSentences {
                adjectives,
                nouns,
                verbs,
                language,
            };
        }
    }

    impl StringGenerator for SimpleSentences {
        fn get(&mut self) -> String {
            let adj1 = self.adjectives.get();
            let adj2 = self.adjectives.get();

            let noun1 = self.nouns.get();
            let noun2 = self.nouns.get();
            
            let verb = self.verbs.get();

            let strong = format!("{} {} {} {} {}", adj1, noun1, verb, adj2 , noun2);

            return strong;
        }
        fn setup(&mut self, conf: Config) {
            self.adjectives.fill("");
            self.nouns.fill("");
            self.verbs.fill("");
        }
    }

    pub fn stringer(conf: Config) -> Box<dyn StringGenerator> {
        let result_box: Box<dyn StringGenerator> = match conf.mode {
            Modes::RandomLetters => Box::new(LettterSequence::new("abc", 16)),
            Modes::RandomWord => Box::new(RandomWord::new(
                ListType::Nouns,
                Languages::from(conf.next.as_ref()),
            )),
            Modes::RandomWordFromListFile => Box::new(RandomWord::new(
                ListType::Nouns,
                Languages::from(conf.next.as_ref()),
            )),
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
    pub fn run_generator(conf: Config) -> Result<(), Error> {
        const OUTPUT_NAME: &str = "strings.textout";
        let mut sg = stringer(conf.clone());
        sg.setup(conf.clone());
        let mut output = File::create(OUTPUT_NAME)?;
        for _i in 0..conf.amount {
            let strang = sg.get();
            if conf.write_to_file {
                writeln!(output, "{}", strang)?;
            } else {
                print!("{}:{}\n", strang, _i);
            }
        }
        return Ok(());
    }
    #[derive(Clone)]
    pub struct Config {
        mode: Modes,
        length: u32,
        amount: u32,
        write_to_file: bool,
        next: String,
    }
    impl Config {
        pub fn new(args: &[String]) -> Config {
            return Config::from(args);
        }
        pub fn from(args: &[String]) -> Config {
            let mut amount = 16;
            let mut mode = Modes::RandomLetters;
            let mut write_to_file = false;

            let mut next = String::new();

            let mut length: u32 = 12;

            if args.len() > 1 {
                //println!("{}",&args[1]);
                amount = args[1].parse().expect("Number must be");
            }

            if args.len() > 2 {
                println!("{}", &args[2]);
                length = args[2].parse().expect("Number must be");
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
            return Config {
                mode,
                length,
                amount,
                write_to_file,
                next,
            };
        }
        pub fn set_mode(&mut self, mode: Modes) {
            self.mode = mode;
        }
        pub fn set_length(&mut self, length: u32) {
            self.length = length;
        }
        pub fn set_amount(&mut self, amount: u32) {
            self.amount = amount;
        }
        pub fn set_write_to_file(&mut self, wtf: bool) {
            self.write_to_file = wtf;
        }
        pub fn set_next(&mut self, next: String) {
            self.next = next;
        }
    }
}
