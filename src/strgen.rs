pub mod string_generator_module {

    use std::fs::read_to_string;
    use std::io::Error;

    
    use crate::stringer::{Config, GermanNounList, Languages, ListType, Modes, RNG};

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
        pub fn pass_generator(length: usize) -> LettterSequence {
            //72 unique symbol set
            let alphabet =
                "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()";
            return LettterSequence::new(alphabet, length);
        }

        pub fn pass_generator84(length: usize) -> LettterSequence {
            //84 unique symbol set
            let alphabet = "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()[]{};:,.<>?|";
            return LettterSequence::new(alphabet, length);
        }

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
            let read_text = read_to_string(conf.get_next());

            if read_text.is_err() {
                println!("Error reading, reverting to latin");
                self.set_alphabet(Languages::English.get_alphabet().as_ref());
                return Ok(());
            }

            //remove spaces and line returns
            //trim_matches?

            let alphabet = read_text.unwrap();

            let mut alpha = String::from(alphabet.trim());
            alpha.retain(char::is_alphanumeric);

            self.set_alphabet(alpha.as_ref());

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

    pub struct WordList {
        list: Vec<String>,
        list_type: ListType,
        language: Languages,
        rng: RNG,
    }

    impl WordList {
        pub fn new(list_type: ListType, language: Languages) -> WordList {
            let list: Vec<String> = Vec::new();
            let mut rng = RNG::new();
            rng.seed();
            return WordList {
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

            use std::fs::File;            
            use std::io::{BufRead,BufReader};

            let file_op2 = File::open(filename);

            if file_op2.is_ok() {
                let file = file_op2.unwrap();
                let mut buff = BufReader::new(file);

                //also there is buff.lines()
                let mut linestr = String::new();

                loop {
                    let res = buff.read_line(&mut linestr);
                    if res.is_ok() {
                        //once line is read
                        let charz = linestr.split(",");

                        for chaz in charz {
                            if chaz == "" {
                                continue;
                            }
                            self.add_word(String::from(chaz.trim()))
                            
                        }

                    } else {
                        break;
                    }
                    if linestr.len() == 0 {
                        break;
                    }
                    linestr.truncate(0);
                }
            }
            
            return Ok(());
        }

        fn get(&mut self) -> String {
            let diclen = self.list.len();
            let index = self.rng.get() as usize % diclen;
            return self.list[index].clone();
        }
    }

    pub struct CoupledWords {
        adjectives: WordList,
        second_type: ListType,
        language: Languages,
        type_list: WordList,
    }
    impl CoupledWords {
        pub fn new(second_type: ListType, language: Languages) -> CoupledWords {
            let adjectives = WordList::new(ListType::Adjectives, language.clone());
            let type_list = WordList::new(second_type.clone(), language.clone());
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
}
