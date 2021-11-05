pub mod parse {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    use crate::{Languages, ListGenerator, ListType};

    //copied from rust site, if you don't undestand this don't worry , so did I.
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    fn get_file_name(_type: ListType, lang: Languages) -> String {
        let head = match _type {
            ListType::Nouns => "nouns",
            ListType::Adjectives => "adjectives",
            ListType::Names => "names",
        };
        let lang = match lang {
            Languages::English => "en",
            Languages::Georgian => "ge",
        };
        return format!("./lists/{}.{}.list", head, lang);
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
                        list.add_word(String::from(chaz))
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
                        list.add_word(String::from(chaz))
                    }
                }
            }
        }
    }
}
