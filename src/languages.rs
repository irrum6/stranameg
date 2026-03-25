pub mod languages {

    //Grammar Genders
    #[derive(Clone, Debug)]
    pub enum Genders {
        Masculine,
        Feminine,
        Neuter,
        NoGender,
    }

    impl Genders {
        pub fn from(s: &str, lang: Languages) -> Genders {
            //lower the case

            return match lang {
                Languages::Georgian => Genders::NoGender,
                Languages::English => Genders::NoGender,
                Languages::German => match s.to_ascii_lowercase().as_ref() {
                    "die" => Genders::Feminine,
                    "der" => Genders::Masculine,
                    "das" => Genders::Neuter,
                    _ => Genders::Neuter,
                },
            };
        }
    }

    #[derive(Clone, Copy)]
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
                "en" | "En" | "eN" | "EN" => Languages::English,
                "ka" | "KA" | "Ka" | "kA" => Languages::Georgian,
                "de" | "dE" | "De" | "DE" => Languages::German,
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

    pub struct Grammar {
        language: Languages,
    }

    impl Grammar {
        pub fn new(lang: Languages) -> Grammar {
            return Grammar { language: lang };
        }

        pub fn get_definite_article(&self, gender: Genders) -> String {
            return match self.language {
                Languages::Georgian => String::new(),
                Languages::English => String::from("the"),
                Languages::German => match gender {
                    Genders::Feminine => String::from("Die"),
                    Genders::Masculine => String::from("Der"),
                    Genders::Neuter => String::from("Das"),
                    Genders::NoGender => String::new(),
                },
            };
        }

        pub fn get_indefinite_article(&self, gender: Genders) -> String {
            return match self.language {
                Languages::Georgian => String::new(),
                Languages::English => String::from("a"),
                Languages::German => match gender {
                    Genders::Feminine => String::from("eine"),
                    Genders::Masculine => String::from("ein"),
                    Genders::Neuter => String::from("ein"),
                    Genders::NoGender => String::new(),
                },
            };
        }

        pub fn get_prefix() -> String {
            return String::new();
        }

        pub fn get_suffix() -> String {
            //for Nominative
            return String::from("e");
        }

        pub fn get_adapted(lang: Languages, word: String, adjective: String) -> String {
            return match lang {
                Languages::English => format!("{}_{}", adjective, word),
                Languages::Georgian => format!("{}_{}", adjective, word),
                Languages::German => {
                    let split: Vec<&str> = word.split(" ").collect();
                    let article = split[0];
                    let noun = split[1];
                    //let gender = Genders::from(split[0], lang);
                    let suffix = Grammar::get_suffix();
                    return format!("{} {}{} {}", article, adjective, suffix, noun);
                }
            };
        }
    }
}
