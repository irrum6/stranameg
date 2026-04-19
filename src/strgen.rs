pub mod string_generator_module {

    use std::fs::read_to_string;
    use std::io::Error;

    use crate::stringer::{Config, Modes, SupportedLanguages, RNG};

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

        fn get_default_alphabet() -> String {
            return String::from("abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy");
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
                println!("Error reading, reverting to default");
                self.set_alphabet(LettterSequence::get_default_alphabet().as_ref());
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
                    let lang = SupportedLanguages::from(conf.get_next().as_ref());
                    let alphabet = lang.get_alphabet();
                    self.set_alphabet(alphabet.as_ref());
                }
                _ => {}
            }
            self.set_length(conf.get_length() as usize);
            return Ok(());
        }
    }

    pub struct CoupledWords {
        lang: SupportedLanguages,
        rng: RNG,
        mode: Modes,
    }

    impl CoupledWords {
        pub fn new(lang: SupportedLanguages) -> CoupledWords {
            let list: Vec<String> = Vec::new();
            let mut rng = RNG::new();
            rng.seed();
            let mode = Modes::CoupledWordsNouns;
            return CoupledWords { lang, rng, mode };
        }
    }

    impl StringGenerator for CoupledWords {
        fn get(&mut self) -> String {
            let rand1 = self.rng.get() as usize;

            let rand2 = self.rng.get() as usize;

            return match self.mode {
                Modes::CoupledWordsNouns => self.lang.get_adapted(rand1, rand2),
                Modes::CoupledWordsNames => self.lang.get_adapted(rand1, rand2),
                Modes::CoupledWordsListFiles => self.lang.get_adapted(rand1, rand2),
                _ => self.lang.get_adapted(rand1, rand2),
            };
        }

        fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            //set mode
            self.mode = conf.get_mode();

            match conf.get_mode() {
                Modes::CoupledWordsNouns => {
                    let path1 = String::from(self.lang.get_default_adjective_list_name());
                    let path2 = String::from(self.lang.get_default_noun_list_name());

                    self.lang.fill_adjectives(path1.as_ref());
                    self.lang.fill_nouns(path2.as_ref());
                    Ok(())
                }
                Modes::CoupledWordsNames => {
                    let path1 = String::from(self.lang.get_default_adjective_list_name());
                    let path2 = String::from(self.lang.get_default_name_list_name());
                    self.lang.fill_adjectives(path1.as_ref());
                    self.lang.fill_names(path2.as_ref());
                    Ok(())
                }
                Modes::CoupledWordsListFiles => {
                    let nxt = conf.get_next();
                    let names: Vec<&str> = nxt.split(":").collect();

                    self.lang.fill_adjectives(names[0]);
                    self.lang.fill_names(names[1]);
                    Ok(())
                }
                _ => Ok(()),
            }
        }
    }
}
