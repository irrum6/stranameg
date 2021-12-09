pub mod parse {
    use std::fs::File;
    use std::io::ErrorKind;
    use std::io::{self, BufRead};
    use std::path::Path;

    use crate::{
        grammar::GermanGenders, grammar::GermanNoun, grammar::GermanNounList, Languages,
        ListGenerator, ListType,
    };

    //copied from rust site and modified
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        match File::open(filename) {
            Ok(file) => return Ok(io::BufReader::new(file).lines()),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("File not found"),
                _ => panic!("Error when opening file"),
            },
        };
    }
    fn get_file_name(_type: ListType, lang: Languages) -> String {
        let head = match _type {
            ListType::Nouns => "nouns",
            ListType::Adjectives => "adjectives",
            ListType::Names => "names",
        };
        let lang = lang.abbr();
        return format!("./lists/{}.{}.list", head, lang);
    }
    pub fn fill_german_nounlist(list: &mut GermanNounList) {
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
                        list.add(gnoun);
                    }
                }
            }
        }
    }

    pub fn fill(list: &mut ListGenerator) {
        let filename = get_file_name(list.get_list_type(), list.get_language());
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        if chaz == "" {
                            continue;
                        }
                        list.add_word(String::from(chaz.trim()))
                    }
                }
            }
        }
    }
    pub fn fill2(list: &mut ListGenerator, filename: String) {
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        if chaz == "" {
                            continue;
                        }
                        list.add_word(String::from(chaz.trim()))
                    }
                }
            }
        }
    }
}
