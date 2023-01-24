pub mod string_generator_module {

    use std::fs::read_to_string;
    use std::io::Error;

    use crate::stringer::read_lines;
    use crate::stringer::{Config, GermanNounList, Languages, ListType, Modes, RNGWheel, RNG};

    pub trait StringGenerator {
        fn get(&mut self) -> String;
        fn setup(&mut self, conf: &Config) -> Result<(), Error>;
    }

    pub struct LettterSequence {
        alphabet: Vec<char>,
        held_string: String,
        length: usize,
        rng: RNG,
    }

    impl LettterSequence {
        pub fn new(s: &str, length: usize) -> LettterSequence {
            let held_string = String::new();
            let alphabet: Vec<char> = s.chars().collect();
            let mut rng = RNG::new();
            rng.seed();
            return LettterSequence {
                held_string,
                alphabet,
                length,
                rng,
            };
        }
        pub fn set_alphabet(&mut self, s: &str) {
            let abc: Vec<char> = s.trim().chars().collect();
            self.alphabet = abc;
        }
        pub fn set_length(&mut self, n: usize) {
            self.length = n;
        }
        fn setup_rlaf(&mut self, conf: &Config) -> Result<(), Error> {
            // let contents = read_to_string(conf.get_next())?;
            // we sorta need to handle error there
            let contents: String = read_to_string(conf.get_next()).unwrap_or_else(|e| {
                println!("Error reading, reverting to latin");
                Languages::English.get_alphabet()
            });
            self.set_alphabet(contents.as_ref());
            return Ok(());
        }
    }
    impl StringGenerator for LettterSequence {
        fn get(&mut self) -> String {
            let len = self.alphabet.len();
            self.held_string = String::new();
            for _i in 0..self.length {
                let num = self.rng.get();
                let index = num as usize % len;
                self.held_string.push(self.alphabet[index]);
            }
            return self.held_string.clone();
        }
        fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            match conf.get_mode() {
                Modes::RandomLettersFromCustomAlphabet => {
                    if conf.get_next().is_empty() {
                        println!("Alphabet paremeter empty, defaulting to latin");
                        self.set_alphabet("abcdefghijklmnopqrstuvwxyz");
                    } else {
                        self.set_alphabet(conf.get_next().as_ref());
                    }
                }
                Modes::RandomLettersFromAlphabetFile => {
                    self.setup_rlaf(conf)?;
                }
                Modes::RandomLetters => {
                    let lang = Languages::from(conf.get_next().as_ref());
                    let alphabet = lang.get_alphabet();
                    self.set_alphabet(alphabet.as_ref());
                }
                _ => {}
            }
            self.set_length(conf.get_length() as usize);
            return Ok(());
        }
    }

    pub struct RandomWord {
        list: Vec<String>,
        list_type: ListType,
        language: Languages,
        rng: RNG,
    }

    impl RandomWord {
        pub fn new(list_type: ListType, language: Languages) -> RandomWord {
            let list: Vec<String> = Vec::new();
            let mut rng = RNG::new();
            rng.seed();
            return RandomWord {
                list,
                list_type,
                language,
                rng,
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
        pub fn fill(&mut self, s: &str) -> Result<(), Error> {
            let filename = if s == "" {
                self.get_file_name()
            } else {
                String::from(s)
            };
            let lines = read_lines(filename)?;
            for line in lines {
                let ip = line?;
                let chazar = ip.split(",");
                for chaz in chazar {
                    if chaz == "" {
                        continue;
                    }
                    self.add_word(String::from(chaz.trim()))
                }
            }
            return Ok(());
        }
    }
    impl StringGenerator for RandomWord {
        fn get(&mut self) -> String {
            let diclen = self.list.len();
            let index = self.rng.get() as usize % diclen;
            return self.list[index].clone();
        }
        fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            match conf.get_mode() {
                Modes::RandomWord => self.fill(""),
                Modes::RandomWordFromListFile => self.fill(conf.get_next().as_ref()),
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
        pub fn new(second_type: ListType, language: Languages) -> CoupledWords {
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
        fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            match conf.get_mode() {
                Modes::CoupledWordsNouns | Modes::CoupledWordsNames => {
                    self.adjectives.fill("")?;
                    self.type_list.fill("")?;
                    Ok(())
                }
                Modes::CoupledWordsListFiles => {
                    let nxt = conf.get_next();
                    let names: Vec<&str> = nxt.split(":").collect();
                    self.adjectives.fill(names[0])?;
                    self.type_list.fill(names[1])?;
                    Ok(())
                }
                _ => Ok(()),
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

            let strong = format!("{} {} {} {} {}", adj1, noun1, verb, adj2, noun2);

            return strong;
        }
        fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            //propagates error
            self.adjectives.fill("")?;
            self.nouns.fill("")?;
            self.verbs.fill("")?;
            Ok(())
        }
    }
}
