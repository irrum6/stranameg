pub mod grammar {

    #[derive(Clone, Debug)]
    pub enum GermanGenders {
        Masculine,
        Feminine,
        Neuter,
    }
    impl GermanGenders {
        pub fn from(s: &str) -> GermanGenders {
            //lower the case
            let s1 = s.to_ascii_lowercase();

            let gg = match s1.as_ref() {
                "die" => GermanGenders::Feminine,
                "der" => GermanGenders::Masculine,
                "das" => GermanGenders::Neuter,
                _ => GermanGenders::Neuter,
            };
            return gg;
        }
    }
    #[derive(Clone, Debug)]
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
            //
            self.list.push(noun);
        }

        pub fn get(&mut self,num:usize) -> String {
            let diclen = self.list.len();
            let index = num as usize % diclen;
            return self.list[index].noun.clone();
        }

        pub fn get_adapted(&mut self, noun: String, adjective: String) -> String {
            if self.list.len() == 0 {
                return String::from("empty");
            }
            //first find word in list
            for word in self.list.iter() {
                //println!("{} {}", word.noun,noun);
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
            use std::fs::File;
            use std::io::{BufRead, BufReader};

            let filename = "./lists/nouns.de.dic";

            let file_op2 = File::open(filename);

            if file_op2.is_err() {
                return ();
            }

            let file = file_op2.unwrap();
            let mut buff = BufReader::new(file);
            //also there is buff.lines()
            let mut linestr = String::new();

            loop {
                let res = buff.read_line(&mut linestr);
                //once line is read
                if res.is_err() {
                    break;
                }

                let charz = linestr.split(",");

                for _char in charz {
                    if _char == "" {
                        continue;
                    }
                    let split: Vec<&str> = _char.trim().split(" ").collect();

                    if split[0].trim() == "" || split[1].trim() == "" {
                        continue;
                    }

                    let gender = GermanGenders::from(split[0]);
                    let noun = String::from(split[1]);
                    let gnoun = GermanNoun::new(noun, gender);
                    self.add(gnoun);
                }

                if linestr.len() == 0 {
                    break;
                }
                linestr.truncate(0);
            }
        }
    }
}
