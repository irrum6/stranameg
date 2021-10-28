pub mod parse {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    use crate::{Coupler, Languages, ListGenerator, ListType};

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
            _ => "",
        };
        let lang = match lang {
            Languages::English => "en",
            Languages::Georgian => "ge",
            _ => "",
        };
        return format!("./lists/{}.{}.list", head, lang);
    }

    pub fn fill(list: &mut ListGenerator) {
        let mut ls: Vec<String> = Vec::new();
        let filename = get_file_name(list.get_list_type(), list.get_language());
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        list.add_list(ls);
    }
    pub fn fill2(list: &mut ListGenerator, filename: String) {
        let mut ls: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        list.add_list(ls);
    }

    pub fn fill_coupled(cw: &mut Coupler) {
        let mut ls: Vec<String> = Vec::new();
        let type1 = ListType::Adjectives;
        let type2 = ListType::Nouns;
        let filename = get_file_name(type1, cw.get_language());
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        cw.add_list(ls, 0);
        let mut ls: Vec<String> = Vec::new();
        let filename = get_file_name(type2, cw.get_language());
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        cw.add_list(ls, 2);
    }
    pub fn fill_coupled2(cw: &mut Coupler, filename: String, filename2: String) {
        let mut ls: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        cw.add_list(ls, 0);
        let mut ls: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename2) {
            for line in lines {
                if let Ok(ip) = line {
                    let chazar = ip.split(",");
                    for chaz in chazar {
                        ls.push(String::from(chaz));
                    }
                }
            }
        }
        cw.add_list(ls, 2);
    }
}