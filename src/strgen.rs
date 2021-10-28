pub mod strgen {
    use crate::RNGWheel;
    use crate::RNG;
    #[derive(Clone)]
    pub enum ListType {
        Nouns,
        Adjectives,
        Names,
    }
    #[derive(Clone)]
    pub enum Languages {
        English,
        Georgian,
    }
    pub trait StringGenerator {
        fn get(&mut self, l: usize) -> String;
    }
    pub struct AlphaBetStringGenerator {
        alphabet: Vec<char>,
        held_string: String,
    }

    impl AlphaBetStringGenerator {
        pub fn new(alpha: Vec<char>) -> AlphaBetStringGenerator {
            let x = String::new();
            return AlphaBetStringGenerator {
                held_string: x,
                alphabet: alpha,
            };
        }
    }
    impl StringGenerator for AlphaBetStringGenerator {
        fn get(&mut self, l: usize) -> String {
            let rng = RNGWheel::new(l);
            let len = self.alphabet.len();
            self.held_string = String::new();
            for num in rng {
                let index = num as usize % len;
                self.held_string.push(self.alphabet[index]);
            }
            return self.held_string.clone();
        }
    }
    pub struct ListStringGenerator {
        list: Vec<String>,
        lstype: ListType,
        language: Languages,
    }

    impl ListStringGenerator {
        pub fn new(lstype: ListType, language: Languages) -> ListStringGenerator {
            let mut list: Vec<String> = Vec::new();
            list.push(String::from("gela"));
            return ListStringGenerator {
                list,
                lstype,
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
            return self.lstype.clone();
        }
        pub fn get_single_word(&self) -> String {
            let mut rng = RNG::new();
            rng.seed();
            let diclen = self.list.len();
            let index = rng.get() as usize % diclen;
            return self.list[index].clone();
        }
    }
    impl StringGenerator for ListStringGenerator {
        fn get(&mut self, len: usize) -> String {
            let mut rng = RNG::new();
            rng.seed();
            let diclen = self.list.len();
            let mut stringlen = 0;
            let mut string_list: Vec<String> = Vec::new();
            while len > stringlen {
                let index = rng.get() as usize % diclen;
                let strong = self.list[index].clone();
                stringlen += strong.len();
                string_list.push(strong);
            }
            return string_list.join(" ");
        }
    }
}
