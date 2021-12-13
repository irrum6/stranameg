pub mod grammar {
    use crate::read_lines;

    #[derive(Clone)]
    pub enum GermanGenders {
        Masculine,
        Feminine,
        Neuter,
    }
    impl GermanGenders {
        pub fn from(s: &str) -> GermanGenders {
            let gg = match s {
                "Die" => GermanGenders::Feminine,
                "die" => GermanGenders::Feminine,
                "Der" => GermanGenders::Masculine,
                "der" => GermanGenders::Masculine,
                "Das" => GermanGenders::Neuter,
                "das" => GermanGenders::Neuter,
                _ => GermanGenders::Neuter,
            };
            return gg;
        }
    }
    #[derive(Clone)]
    pub struct GermanNoun {
        noun: String,
        gender: GermanGenders,
    }
    impl GermanNoun {
        pub fn new(noun: String, gender: GermanGenders) -> GermanNoun {
            return GermanNoun { noun, gender };
        }
        pub fn get_prefix(&self) -> String {
            let gender = match self.gender {
                GermanGenders::Masculine => "Der",
                GermanGenders::Feminine => "Die",
                GermanGenders::Neuter => "Das",
            };
            return String::from(gender);
        }
        pub fn get_suffix(&self) -> String {
            let s = match self.gender {
                GermanGenders::Masculine => "e",
                GermanGenders::Feminine => "e",
                GermanGenders::Neuter => "e",
            };
            return String::from(s);
        }
    }
    pub struct GermanNounList {
        list: Vec<GermanNoun>,
    }
    impl GermanNounList {
        pub fn new() -> GermanNounList {
            let list: Vec<GermanNoun> = Vec::new();
            return GermanNounList { list };
        }
        pub fn add(&mut self, noun: GermanNoun) {
            self.list.push(noun);
        }
        pub fn get_adapted(&mut self, noun: String, adjective: String) -> String {
            if 0 == self.list.len() {
                return String::from("empty");
            }
            //first find word in list
            for word in self.list.iter() {
                if word.noun == noun {
                    let suffix = word.get_suffix();
                    let prefix = word.get_prefix();
                    let strong = format!("{} {}{} {}", prefix, adjective, suffix, noun);
                    return strong;
                }
            }
            return String::new();
        }
        pub fn fill(&mut self) {
            let filename = "./lists/genders.de.dic";
            if let Ok(lines) = read_lines(filename) {
                for line in lines {
                    if let Ok(ip) = line {
                        let chazar = ip.split(",");
                        for chaz in chazar {
                            if chaz == "" {
                                continue;
                            }
                            let spl: Vec<&str> = chaz.trim().split(" ").collect();
                            let gender = GermanGenders::from(spl[0]);
                            let noun = String::from(spl[1]);
                            let gnoun = GermanNoun::new(noun, gender);
                            self.add(gnoun);
                        }
                    }
                }
            }
        }
    }
}

pub mod strgen {
    use std::fs::read_to_string as fs_read;
    use std::fs::File;
    use std::io::{Error, Write};

    use crate::read_lines;
    use crate::RNGWheel;
    use crate::RNG;

    #[derive(Clone)]
    enum Modes {
        RandomLetters,
        RandomLettersFromCustomAlphabet,
        RandomLettersFromAlphabetFile,
        RandomWord,
        RandomWordFromListFile,
        CoupledWordsNouns,
        CoupledWordsNames,
        CoupledWordsListFiles,
    }
    impl Modes {
        pub fn from(s: &str) -> Modes {
            return match s {
                "rls" | "10" => Modes::RandomLetters,
                "rla" | "11" => Modes::RandomLettersFromCustomAlphabet,
                "rlaf" | "12" => Modes::RandomLettersFromAlphabetFile,
                "raw" | "21" => Modes::RandomWord,
                "rawl" | "22" => Modes::RandomWordFromListFile,
                "cow" | "31" => Modes::CoupledWordsNouns,
                "cowe" | "32" => Modes::CoupledWordsNames,
                "cowf" | "33" => Modes::CoupledWordsListFiles,
                _ => Modes::RandomLetters,
            };
        }
    }

    use super::grammar::GermanNounList;

    #[derive(Clone)]
    pub enum ListType {
        Nouns,
        Adjectives,
        Names,
    }
    impl ListType {
        pub fn is_noun(&self) -> bool {
            return matches!(*self, ListType::Nouns);
        }
    }
    #[derive(Clone)]
    pub enum Languages {
        English,
        Georgian,
        German,
    }
    impl Languages {
        pub fn abbr(&self) -> String {
            let result = match *self {
                Languages::English => "en",
                Languages::Georgian => "ka",
                Languages::German => "de",
            };
            return String::from(result);
        }
        pub fn is_german(&self) -> bool {
            return matches!(*self, Languages::German);
        }
        pub fn from(s: &str) -> Languages {
            return match s {
                "en" => Languages::English,
                "ka" => Languages::Georgian,
                "de" => Languages::German,
                _ => Languages::English,
            };
        }
        pub fn get_alphabet(&self) -> String {
            let result = match *self {
                Languages::Georgian => "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ",
                Languages::English => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
                _ => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
            };
            return String::from(result);
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
                ListType::Names => "names",
            };
            let lang = lang.abbr();
            return format!("./lists/{}.{}.list", head, lang);
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
                _ => {}
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

    fn stringer(conf: Config) -> Box<dyn StringGenerator> {
        let result_box: Box<dyn StringGenerator> = match conf.mode {
            Modes::RandomLetters => Box::new(LettterSequence::new("abc", 16)),
            Modes::RandomWord => Box::new(RandomWord::new(ListType::Nouns, conf.lang.clone())),
            Modes::CoupledWordsNouns => {
                Box::new(CoupledWords::new(ListType::Nouns, conf.lang.clone()))
            }
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
        lang: Languages,
        length: u32,
        amount: u32,
        write_to_file: bool,
        next: String,
    }
    impl Config {
        pub fn new(args: &[String]) -> Config {
            let mut amount = 16;
            let lang = Languages::English;
            let mut mode = Modes::RandomLetters;
            let mut write_to_file = false;

            let mut next = String::new();

            let mut length: u32 = 12;

            if args.len() > 1 {
                amount = args[1].parse().expect("Number must be");
            }

            if args.len() > 2 {
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
                lang,
                length,
                amount,
                write_to_file,
                next,
            };
        }
    }
}
