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
    fn get_file_name(list: &ListGenerator) -> String {
        let head = match list.get_list_type() {
            ListType::Nouns => "nouns",
            ListType::Adjectives => "adjectives",
            ListType::Names => "names",
            _ => "",
        };
        let lang = match list.get_language() {
            Languages::English => "en",
            Languages::Georgian => "ge",
            _ => "",
        };
        return format!("./lists/{}.{}.list", head, lang);
    }

    pub fn fill(list: &mut ListGenerator) {
        let mut ls: Vec<String> = Vec::new();
        let filename = get_file_name(list);
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
}
