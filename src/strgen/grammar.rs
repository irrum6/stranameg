pub mod grammar {
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
            //first find word in list
            if 0 == self.list.len() {
                return String::from("empty");
            }
            for word in self.list.iter() {
                if word.noun == noun {
                    let suffix = match word.gender {
                        GermanGenders::Masculine => "e",
                        GermanGenders::Feminine => "e",
                        GermanGenders::Neuter => "e",
                    };
                    let prefix = word.get_prefix();
                    let strong = format!("{} {}{} {}", prefix, adjective, suffix, noun);
                    return strong;
                }
            }
            return String::new();
        }
    }
}
