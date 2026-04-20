pub mod string_generator_module {

    use std::fs::read_to_string;
    use std::io::Error;

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
            let alphabet =
                "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()";
            self.set_alphabet(alphabet);
            self.set_length(length);
        }

        pub fn set_pass_generator84(&mut self, length: usize) {
            //84 unique symbol set
            let alphabet = "abcdefghijklmnopqrstuvwxyz0123456789aaiioouuyABCDEFGHIJKLMNOPQRSTUVWXYZ!@#$%^&*()[]{};:,.<>?|";
            self.set_alphabet(alphabet);
            self.set_length(length);
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

        pub fn set_language(&mut self, language: SupportedLanguages) {
            self.set_alphabet(language.get_alphabet());
            self.lang = language;
        }

        fn setup_rlaf(&mut self, conf: &Config) -> Result<(), Error> {
            // let contents = read_to_string(conf.get_next())?;
            // we sorta need to handle error there
            let read_text = read_to_string(conf.get_next());

            if read_text.is_err() {
                println!("Error reading, reverting to default");
                self.set_alphabet(StringGenerator::get_default_alphabet().as_ref());
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

        //get couped words string
        fn get_cws(&mut self) -> String {
            let rand1 = self.rng.get() as usize;

            let rand2 = self.rng.get() as usize;

            return match self.mode {
                Modes::CoupledWordsNouns => self.lang.get_adapted(rand1, rand2),
                Modes::CoupledWordsNames => self.lang.get_adapted(rand1, rand2),
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

        pub fn get(&mut self)->String{
            return match self.mode {
                Modes::CoupledWordsNouns => self.get_cws(),
                Modes::CoupledWordsNames => self.get_cws(),
                Modes::CoupledWordsListFiles => self.get_cws(),
                _ => self.get_rls()
            }
        }

        pub fn setup(&mut self, conf: &Config) -> Result<(), Error> {
            //first assign mode
            self.mode = conf.get_mode();

            return match self.mode {
                Modes::Password => {
                    let length = conf.get_length() as usize;
                    self.set_pass_generator(length);
                    Ok(())
                }
                Modes::Password84 => {
                    let length = conf.get_length() as usize;
                    self.set_pass_generator84(length);
                    Ok(())
                }
                Modes::RandomLetters => {
                    let lang = SupportedLanguages::from(conf.get_next().as_ref());
                    self.set_language(lang);
                    self.set_length(conf.get_length() as usize);
                    Ok(())
                }
                Modes::RandomLettersFromCustomAlphabet => {
                    if conf.get_next().is_empty() {
                        println!("Alphabet paremeter empty, defaulting to latin");
                        self.set_alphabet("abcdefghijklmnopqrstuvwxyz");
                    } else {
                        self.set_alphabet(conf.get_next().as_ref());
                    }
                    self.set_length(conf.get_length() as usize);
                    Ok(())
                }
                Modes::RandomLettersFromAlphabetFile => {
                    self.set_length(conf.get_length() as usize);
                    let res = self.setup_rlaf(conf);
                    Ok(())
                }
                Modes::CoupledWordsNouns => {
                    let lang = SupportedLanguages::from(&conf.get_next());
                    self.set_language(lang);

                    let path1 = String::from(self.lang.get_default_adjective_list_name());
                    let path2 = String::from(self.lang.get_default_noun_list_name());

                    let r = self.lang.fill_adjectives(path1.as_ref());
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    let r = self.lang.fill_nouns(path2.as_ref());
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    Ok(())
                }
                Modes::CoupledWordsNames => {
                    let lang = SupportedLanguages::from(&conf.get_next());
                    self.set_language(lang);


                    let path1 = String::from(self.lang.get_default_adjective_list_name());
                    let path2 = String::from(self.lang.get_default_name_list_name());

                    let r =self.lang.fill_adjectives(path1.as_ref());
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    let r =self.lang.fill_names(path2.as_ref());
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    Ok(())
                }
                Modes::CoupledWordsListFiles => {
                    
                    let nxt = conf.get_next();
                    let names: Vec<&str> = nxt.split(":").collect();

                    let r =self.lang.fill_adjectives(names[0]);
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    let r = self.lang.fill_nouns(names[1]);
                    if r.is_err(){
                        println!("{:?}",r);
                        return Ok(());
                    }
                    Ok(())
                }
            };
        }
    }
}
