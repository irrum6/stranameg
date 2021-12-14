pub mod grammar {
    use crate::read_lines;

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
        pub fn get_suffix(&self) -> String {
            let s = match self.gender {
                GermanGenders::Masculine => "e",
                GermanGenders::Feminine => "e",
                GermanGenders::Neuter => "e",
            };
            return String::from(s);
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
            if 0 == self.list.len() {
                return String::from("empty");
            }
            //first find word in list
            for word in self.list.iter() {
                if word.noun == noun {
                    let suffix = word.get_suffix();
                    let prefix = word.get_prefix();
                    let strong = format!("{} {}{} {}", prefix, adjective, suffix, noun);
                    return strong;
                }
            }
            return String::new();
        }
        pub fn fill(&mut self) {
            let filename = "./lists/genders.de.dic";
            if let Ok(lines) = read_lines(filename) {
                for line in lines {
                    if let Ok(ip) = line {
                        let chazar = ip.split(",");
                        for chaz in chazar {
                            if chaz == "" {
                                continue;
                            }
                            let spl: Vec<&str> = chaz.trim().split(" ").collect();
                            let gender = GermanGenders::from(spl[0]);
                            let noun = String::from(spl[1]);
                            let gnoun = GermanNoun::new(noun, gender);
                            self.add(gnoun);
                        }
                    }
                }
            }
        }
    }
}