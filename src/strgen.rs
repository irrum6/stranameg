pub mod string_generator_module {

    use std::fs::read_to_string;

    use crate::stringer::{Config, EnglishLanguage, Modes, SupportedLanguages, RNG};

    pub struct StringGenerator {
        alphabet: Vec<char>,
        length: usize,
        rng: RNG,
        lang: SupportedLanguages,
        mode: Modes,
    }

    impl StringGenerator {
        pub fn default() -> StringGenerator {
            //let alphabet: Vec<char> = StringGenerator::get_default_alphabet().chars().collect();
            let mut rng = RNG::new();
            rng.seed();
            let mode = Modes::RandomLetters;
            let length = 16;
            let lang = SupportedLanguages::English(EnglishLanguage::new());
            let alphabet = lang.get_alphabet().chars().collect();
            return StringGenerator {
                alphabet,
                length,
                rng,
                lang,
                mode,
            };
        }

        pub fn set_pass_generator(&mut self, length: usize) {
            //72 unique symbol set
            let alpha =
                "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()";
            self.alphabet = alpha.chars().collect();
            self.length = length;
        }

        pub fn set_pass_generator84(&mut self, length: usize) {
            //84 unique symbol set
            let alpha = "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()[]{};:,.<>?|";
            self.alphabet = alpha.chars().collect();
            self.length = length;
        }

        pub fn set_length(&mut self, n: usize) {
            self.length = n;
        }

        pub fn set_language(&mut self, language: SupportedLanguages) {
            self.alphabet = language.get_alphabet().chars().collect();
            self.lang = language;
        }

        fn setup_rlaf(&mut self, conf: &Config) {
            // we sorta need to handle error there
            let read_text = read_to_string(conf.get_next());

            match read_text {
                Ok(alphabet)=>{
                    let alpha = alphabet.trim();
                    self.alphabet = alpha.chars().collect();
                }
                Err(e)=>{
                    println!("Error reading file");
                }
            }
            //alpha.retain(char::is_alphanumeric);
        }

        //get couped words string
        fn get_cws(&mut self) -> String {
            let rand1 = self.rng.get() as usize;

            let rand2 = self.rng.get() as usize;

            return match self.mode {
                Modes::CoupledWordsNouns => self.lang.get_adapted(rand1, rand2),
                Modes::CoupledWordsNames => self.lang.get_adapted2(rand1, rand2),
                Modes::CoupledWordsListFiles => self.lang.get_adapted(rand1, rand2),
                _ => self.lang.get_adapted(rand1, rand2),
            };
        }
        //get random letter string
        fn get_rls(&mut self) -> String {
            let len = self.alphabet.len();
            let mut held_string = String::new();
            for _i in 0..self.length {
                let num = self.rng.get();
                let index = num as usize % len;
                held_string.push(self.alphabet[index]);
            }
            return held_string.clone();
        }

        pub fn get(&mut self) -> String {
            return match self.mode {
                Modes::CoupledWordsNouns => self.get_cws(),
                Modes::CoupledWordsNames => self.get_cws(),
                Modes::CoupledWordsListFiles => self.get_cws(),
                _ => self.get_rls(),
            };
        }

        pub fn setup(&mut self, conf: &Config) {
            //first assign mode
            self.mode = conf.get_mode();

            match self.mode {
                Modes::Password => {
                    let length = conf.get_length() as usize;
                    self.set_pass_generator(length);
                }
                Modes::Password84 => {
                    let length = conf.get_length() as usize;
                    self.set_pass_generator84(length);
                }
                Modes::RandomLetters => {
                    let lang = SupportedLanguages::from(conf.get_next().as_ref());
                    self.set_language(lang);
                    self.set_length(conf.get_length() as usize);
                }
                Modes::RandomLettersFromCustomAlphabet => {
                    if conf.get_next().is_empty() {
                        println!("Alphabet paremeter empty, using default");
                    } else {
                        self.alphabet = conf.get_next().chars().collect();
                    }
                    self.set_length(conf.get_length() as usize);
                }
                Modes::RandomLettersFromAlphabetFile => {
                    self.set_length(conf.get_length() as usize);
                    self.setup_rlaf(conf);
                }
                Modes::CoupledWordsNouns => {
                    let lang = SupportedLanguages::from(&conf.get_next());
                    self.set_language(lang);

                    let path1 = String::from(self.lang.get_list_name("adj"));
                    let path2 = String::from(self.lang.get_list_name("noun"));

                    let r = self.lang.fill_adjectives(path1.as_ref());
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                    let r = self.lang.fill_nouns(path2.as_ref());
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                }
                Modes::CoupledWordsNames => {
                    let lang = SupportedLanguages::from(&conf.get_next());
                    self.set_language(lang);

                    let path1 = String::from(self.lang.get_list_name("adj"));
                    let path2 = String::from(self.lang.get_list_name("name"));

                    let r = self.lang.fill_adjectives(path1.as_ref());
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                    let r = self.lang.fill_names(path2.as_ref());
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                }
                Modes::CoupledWordsListFiles => {
                    let nxt = conf.get_next();
                    let fnames: Vec<&str> = nxt.split(":").collect();

                    println!("{:?}", &fnames);

                    let r = self.lang.fill_adjectives(fnames[0]);
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                    let r = self.lang.fill_nouns(fnames[1]);
                    if r.is_err() {
                        println!("{:?}", r);
                    }
                }
            };
        }
    }
}
