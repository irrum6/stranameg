pub mod languages {
    use std::io::Error;

    enum Gender {
        Feminine,
        Masculine,
        Neutral,
    }
    #[derive(Clone, Copy, Debug)]
    enum WordType {
        Noun,
        Name,
        Adjective,
    }
    #[derive(Clone, Debug)]
    struct Word {
        word: String,
        wordtype: WordType,
    }

    trait Grammar {
        fn get_definite_article(w: &Word) -> String;
        fn get_indefinite_article(w: &Word) -> String;
        fn get_adapted(&self, rand1: usize, rand2: usize) -> String;
        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String;
    }

    #[derive(Debug)]
    struct Dictionary {
        wordlist: Vec<Word>,
        index: usize,
    }

    impl Dictionary {
        fn new() -> Dictionary {
            let wordlist: Vec<Word> = Vec::new();
            return Dictionary { wordlist, index: 0 };
        }

        fn add_word(&mut self, w: Word) {
            self.wordlist.push(w);
        }

        fn get_random_word(&self, rand: usize) -> Word {
            let diclen = self.wordlist.len();

            //println!("{:?}", &self.wordlist);

            if diclen == 0 {
                return Word {
                    word: String::new(),
                    wordtype: WordType::Name,
                };
            }

            let index = rand % diclen;
            return self.wordlist[index].clone();
        }

        fn fill(&mut self, path: &str, wt: WordType) -> Result<(), Error> {
            use std::fs::File;
            use std::io::{BufRead, BufReader};

            let filename = format!("./lists/{}", path);

            let file_op2 = File::open(filename);

            if file_op2.is_err() {
                println!("fill:err, failing silently");
                //println!("{}",&path);
                return Ok(());
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

                //println!("{}", &linestr);
                //println!("{}", &counter);

                if linestr.len() == 0 {
                    println!("empty line");
                    break;
                }

                let charz = linestr.split(",");

                for chaz in charz {
                    if chaz == "" {
                        println!("{}", &chaz);
                        continue;
                    }
                    self.add_word(Word {
                        word: String::from(chaz),
                        wordtype: wt,
                    });
                    print!("{} ", &chaz);
                }
                //println!("{:?}",self.wordlist);
                //after successfull addition increase counter
                counter += 1;

                linestr.truncate(0);
            }

            return Ok(());
        }
    }

    impl Iterator for Dictionary {
        type Item = Word;

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

        fn fill_nouns(&mut self, path: &str) -> Result<(), Error> {
            //let path = "gela";
            return self.nouns.fill(path, WordType::Noun);
        }

        fn fill_names(&mut self, path: &str) -> Result<(), Error> {
            return self.names.fill(path, WordType::Name);
        }

        fn fill_adjectives(&mut self, path: &str) -> Result<(), Error> {
            return self.adjectives.fill(path, WordType::Adjective);
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
        fn get_definite_article(w: &Word) -> String {
            return String::new();
        }

        fn get_indefinite_article(w: &Word) -> String {
            return String::new();
        }
        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            return format!("{}_{}", adj.word, noun.word);
        }
        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);

            return format!("{}_{}", adj.word, name.word);
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
        fn get_definite_article(w: &Word) -> String {
            return String::from("the");
        }

        fn get_indefinite_article(w: &Word) -> String {
            //check if starts with consontant
            return String::from("a");
        }

        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            return format!("{}_{}", adj.word, noun.word);
        }

        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);

            return format!("{}_{}", adj.word, name.word);
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
        fn get_gender(w: &Word) -> Gender {
            let s: Vec<&str> = w.word.split(" ").collect();
            return match s[0].to_lowercase().as_ref() {
                "die" => Gender::Feminine,
                "der" => Gender::Masculine,
                "das" => Gender::Neutral,
                _ => Gender::Neutral,
            };
        }

        fn abbr(&self) -> String {
            return String::from("De");
        }

        fn get_suffix() -> String {
            //for Nominative
            return String::from("e");
        }
    }

    impl Grammar for GermanLanguage {
        fn get_definite_article(w: &Word) -> String {
            //split and determine
            let s: Vec<&str> = w.word.split(" ").collect();
            return String::from(s[0]);
        }

        fn get_indefinite_article(w: &Word) -> String {
            //split and determine gender
            // split by space

            let gender = GermanLanguage::get_gender(w);

            return match gender {
                Gender::Feminine => String::from("eine"),
                Gender::Masculine => String::from("ein"),
                Gender::Neutral => String::from("ein"),
            };
        }

        fn get_adapted(&self, rand1: usize, rand2: usize) -> String {
            let adj = self.language.adjectives.get_random_word(rand1);
            let noun = self.language.nouns.get_random_word(rand2);

            let split: Vec<&str> = noun.word.split(" ").collect();
            let article = GermanLanguage::get_indefinite_article(&noun);
            let noun = split[1];
            //let gender = Genders::from(split[0], lang);
            let suffix = GermanLanguage::get_suffix();
            return format!("{} {}{} {}", article, adj.word, suffix, noun);
        }

        fn get_adapted2(&self, rand1: usize, rand2: usize) -> String {

            let adj = self.language.adjectives.get_random_word(rand1);
            let name = self.language.names.get_random_word(rand2);
            //println!("{:?}",&name);

            let suffix = String::from("e");
            return format!("{}{} {}", adj.word, suffix, name.word);
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
                "en" => {
                    let mut en = EnglishLanguage::new();
                    SupportedLanguages::English(en)
                }

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

        pub fn fill_nouns(&mut self, path: &str) -> Result<(), Error> {
            return match self {
                SupportedLanguages::English(e) => e.language.fill_nouns(path),
                SupportedLanguages::Georgian(e) => e.language.fill_nouns(path),
                SupportedLanguages::German(e) => e.language.fill_nouns(path),
            };
        }

        pub fn fill_names(&mut self, path: &str) -> Result<(), Error> {
            return match self {
                SupportedLanguages::English(e) => e.language.fill_names(path),
                SupportedLanguages::Georgian(e) => e.language.fill_names(path),
                SupportedLanguages::German(e) => e.language.fill_names(path),
            };
        }

        pub fn fill_adjectives(&mut self, path: &str) -> Result<(), Error> {
            return match self {
                SupportedLanguages::English(e) => e.language.fill_adjectives(path),
                SupportedLanguages::Georgian(e) => e.language.fill_adjectives(path),
                SupportedLanguages::German(e) => e.language.fill_adjectives(path),
            };
        }
        //default file names
        pub fn get_default_noun_list_name(&self) -> &str {
            match self {
                SupportedLanguages::English(e) => "nouns.en.list",
                SupportedLanguages::Georgian(e) => "nouns.ka.list",
                SupportedLanguages::German(e) => "nouns.de.list",
            }
        }

        pub fn get_default_adjective_list_name(&self) -> &str {
            match self {
                SupportedLanguages::English(e) => "adjectives.en.list",
                SupportedLanguages::Georgian(e) => "adjectives.ka.list",
                SupportedLanguages::German(e) => "adjectives.de.list",
            }
        }

        pub fn get_default_name_list_name(&self) -> &str {
            match self {
                SupportedLanguages::English(e) => "names.en.list",
                SupportedLanguages::Georgian(e) => "names.ka.list",
                SupportedLanguages::German(e) => "names.de.list",
            }
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
