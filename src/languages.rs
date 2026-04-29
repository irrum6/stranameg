pub mod languages {

    enum Gender {
        Feminine,
        Masculine,
        Neutral,
    }

    trait Grammar {
        fn get_indefinite_article(w: &str) -> String;
        fn get_adapted(&self, rand1: usize, rand2: usize) -> String;
        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String;
    }

    #[derive(Debug)]
    struct Dictionary {
        wordlist: Vec<String>,
        index: usize,
    }

    impl Dictionary {
        fn new() -> Dictionary {
            let wordlist: Vec<String> = Vec::new();
            return Dictionary { wordlist, index: 0 };
        }

        fn get_random_word(&self, rand: usize) -> String {
            let diclen = self.wordlist.len();

            if diclen == 0 {
                return String::new();
            }

            let index = rand % diclen;
            return self.wordlist[index].clone();
        }

        fn fill(&mut self, path: &str) -> Result<(), String> {
            use std::fs::File;
            use std::io::{BufRead, BufReader};

            let filename = String::from(path);

            let file_op2 = File::open(filename);

            if file_op2.is_err() {
                return Err(String::from("d.fill: error occured when reading file"));
            }

            let file = file_op2.unwrap();
            let mut buff = BufReader::new(file);

            //also there is buff.lines()
            let mut linestr = String::new();

            let mut counter = 0;

            loop {
                let res = buff.read_line(&mut linestr);
                //once line is read
                if res.is_err() {
                    println!("{:?}", res);
                    break;
                }

                if linestr.len() == 0 {
                    if counter == 0 {
                        println!("empty");
                    }
                    break;
                }

                let charz = linestr.split(",");

                for chaz in charz {
                    if chaz == "" {
                        continue;
                    }
                    self.wordlist.push(String::from(chaz.trim()));
                }
                counter += 1;
                linestr.truncate(0);
            }

            return Ok(());
        }
    }

    impl Iterator for Dictionary {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index < self.wordlist.len() {
                let word = self.wordlist[self.index].clone();
                self.index += 1;
                return Some(word);
            }
            //self.index = 0;
            return None;
        }
    }

    struct Language {
        alphabet: String,
        nouns: Dictionary,
        adjectives: Dictionary,
        names: Dictionary,
    }

    impl Language {
        fn new(abc: &str) -> Language {
            let alphabet = String::from(abc);
            let nouns = Dictionary::new();
            let adjectives = Dictionary::new();
            let names = Dictionary::new();

            return Language {
                alphabet,
                nouns,
                adjectives,
                names,
            };
        }

        fn get_alphabet(&self) -> &String {
            return &self.alphabet;
        }
    }

    pub struct GeorgianLanguage {
        language: Language,
    }

    impl GeorgianLanguage {
        pub fn new() -> GeorgianLanguage {
            let language = Language::new("აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ");
            return GeorgianLanguage { language };
        }
    }

    impl Grammar for GeorgianLanguage {
        fn get_indefinite_article(_w: &str) -> String {
            return String::new();
        }
        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            return format!("{}_{}", adj, noun);
        }
        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);

            return format!("{}_{}", adj, name);
        }
    }

    pub struct EnglishLanguage {
        language: Language,
    }

    impl EnglishLanguage {
        pub fn new() -> EnglishLanguage {
            let language = Language::new("abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy");
            return EnglishLanguage { language };
        }
    }

    impl Grammar for EnglishLanguage {
        fn get_indefinite_article(w: &str) -> String {
            //check if starts with consontant
            // h needs further adjustment
            let chars: Vec<char> = w.to_lowercase().chars().collect();
            let fch = chars[0];

            return match fch {
                'a' | 'e' | 'i' | 'o' | 'u' => String::from("an"),
                _ => String::from("a"),
            };
        }

        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            let article = EnglishLanguage::get_indefinite_article(&adj);

            return format!("{}_{}_{}", article, adj, noun);
        }

        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);

            return format!("{}_{}", adj, name);
        }
    }

    pub struct GermanLanguage {
        language: Language,
    }

    impl GermanLanguage {
        fn new() -> GermanLanguage {
            let language = Language::new("abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy");
            return GermanLanguage { language };
        }
        #[inline]
        fn get_gender(w: &str) -> Gender {
            return match w.to_lowercase().as_ref() {
                "die" => Gender::Feminine,
                "der" => Gender::Masculine,
                "das" => Gender::Neutral,
                _ => Gender::Neutral,
            };
        }

        fn get_suffix(w: &str) -> &str {
            //split and determine gender
            // split by space

            return match GermanLanguage::get_gender(&w) {
                Gender::Feminine => "e",
                Gender::Masculine => "er",
                Gender::Neutral => "es",
            };
        }
    }

    impl Grammar for GermanLanguage {
        fn get_indefinite_article(w: &str) -> String {
            //split and determine gender
            // split by space
            return match GermanLanguage::get_gender(&w) {
                Gender::Feminine => String::from("eine"),
                Gender::Masculine => String::from("ein"),
                Gender::Neutral => String::from("ein"),
            };
        }

        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            let split: Vec<&str> = noun.split(" ").collect();

            let def_article = split[0];
            let noun = split[1];

            let article = GermanLanguage::get_indefinite_article(def_article);
            let suffix = GermanLanguage::get_suffix(def_article);

            return format!("{} {}{} {}", article, adj, suffix, noun);
        }

        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);

            let suffix = "e";
            return format!("{}{} {}", adj, suffix, name);
        }
    }

    pub enum SupportedLanguages {
        German(GermanLanguage),
        Georgian(GeorgianLanguage),
        English(EnglishLanguage),
    }

    impl SupportedLanguages {
        pub fn from(s: &str) -> SupportedLanguages {
            return match s.to_lowercase().as_ref() {
                "en" => SupportedLanguages::English(EnglishLanguage::new()),
                "ka" => SupportedLanguages::Georgian(GeorgianLanguage::new()),
                "de" => SupportedLanguages::German(GermanLanguage::new()),
                _ => SupportedLanguages::English(EnglishLanguage::new()),
            };
        }
        pub fn get_alphabet(&self) -> &String {
            return match &self {
                SupportedLanguages::English(e) => e.language.get_alphabet(),
                SupportedLanguages::Georgian(e) => e.language.get_alphabet(),
                SupportedLanguages::German(e) => e.language.get_alphabet(),
            };
        }

        pub fn fill_nouns(&mut self, path: &str) -> Result<(), String> {
            return match self {
                SupportedLanguages::English(e) => e.language.nouns.fill(path),
                SupportedLanguages::Georgian(e) => e.language.nouns.fill(path),
                SupportedLanguages::German(e) => e.language.nouns.fill(path),
            };
        }

        pub fn fill_names(&mut self, path: &str) -> Result<(), String> {
            return match self {
                SupportedLanguages::English(e) => e.language.names.fill(path),
                SupportedLanguages::Georgian(e) => e.language.names.fill(path),
                SupportedLanguages::German(e) => e.language.names.fill(path),
            };
        }

        pub fn fill_adjectives(&mut self, path: &str) -> Result<(), String> {
            return match self {
                SupportedLanguages::English(e) => e.language.adjectives.fill(path),
                SupportedLanguages::Georgian(e) => e.language.adjectives.fill(path),
                SupportedLanguages::German(e) => e.language.adjectives.fill(path),
            };
        }

        //default file names
        pub fn get_list_name(&self, lstype: &str) -> String {
            let pfx = match lstype.to_lowercase().as_ref() {
                "noun" => "nouns",
                "adj" => "adjectives",
                "name" => "names",
                _ => "",
            };
            let sfx = "list";
            let middle = match self {
                SupportedLanguages::English(_e) => "en",
                SupportedLanguages::Georgian(_e) => "ka",
                SupportedLanguages::German(_e) => "de",
            };

            return format!("./lists/{}.{}.{}", pfx, middle, sfx);
        }

        pub fn get_adapted(&self, r1: usize, r2: usize) -> String {
            return match self {
                SupportedLanguages::English(e) => call_get_adapted(e, r1, r2),
                SupportedLanguages::Georgian(e) => call_get_adapted(e, r1, r2),
                SupportedLanguages::German(e) => call_get_adapted(e, r1, r2),
            };
        }

        pub fn get_adapted2(&self, r1: usize, r2: usize) -> String {
            return match self {
                SupportedLanguages::English(e) => call_get_adapted2(e, r1, r2),
                SupportedLanguages::Georgian(e) => call_get_adapted2(e, r1, r2),
                SupportedLanguages::German(e) => call_get_adapted2(e, r1, r2),
            };
        }
    }
    //
    fn call_get_adapted(lang: &impl Grammar, r1: usize, r2: usize) -> String {
        return lang.get_adapted(r1, r2);
    }

    fn call_get_adapted2(lang: &impl Grammar, r1: usize, r2: usize) -> String {
        return lang.get_adapted2(r1, r2);
    }
}
