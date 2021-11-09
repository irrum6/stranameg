pub mod modes {
    use std::fs::read_to_string as fs_read;
    use std::fs::File;
    use std::io::{Error, Write};

    const OUTPUT_NAME: &str = "strings.textout";

    use crate::grammar::GermanNounList;
    use crate::{
        fill_german_nounlist, fill_list, fill_list2, ABCGenerator, Languages, ListGenerator,
        ListType, StringGenerator,
    };

    fn get_alphabet(lang: Languages) -> String {
        let result = match lang {
            Languages::Georgian => "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ",
            Languages::English => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
            _ => "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy",
        };
        return String::from(result);
    }

    pub fn mode1x(len: u32, amount: u32, _mode: u32, next: String) -> Result<(), Error> {
        let mut mode = _mode;
        let mut wtf = false;
        if mode > 99 {
            mode = mode / 10;
            wtf = true;
        }

        let mut sg = ABCGenerator::new("abc");

        if mode == 10 {
            let lang = Languages::from(&next);
            let alphabet = get_alphabet(lang);
            sg.set_alphabet(alphabet.as_ref());
        }
        if mode == 11 {
            sg.set_alphabet(next.as_ref());
        }
        if mode == 12 {
            let contents =
                fs_read(next.clone().trim()).expect("Something went wrong reading the file");
            sg.set_alphabet(contents.as_ref());
        }
        let mut output = File::create(OUTPUT_NAME)?;
        for _i in 0..amount {
            let strang = sg.get(len as usize);
            if wtf {
                writeln!(output, "{}", strang)?;
            } else {
                print!("{}:{}\n", strang, _i);
            }
        }
        return Ok(());
    }
    pub fn mode2x(len: u32, amount: u32, _mode: u32, next: String) -> Result<(), Error> {
        let mut mode = _mode;
        let mut wtf = false;
        if mode > 99 {
            mode = mode / 10;
            wtf = true;
        }
        let lst = ListType::Nouns;
        let lan = Languages::from(&next);
        let mut lsg = ListGenerator::new(lst, lan);
        if mode == 21 {
            fill_list(&mut lsg);
        }
        if mode == 22 {
            fill_list2(&mut lsg, next.clone());
        }
        let mut output = File::create(OUTPUT_NAME)?;
        for _i in 0..amount {
            let strang = lsg.get(len as usize);
            if wtf {
                writeln!(output, "{}", strang)?;
            } else {
                print!("{}:{}\n", strang, _i);
            }
        }
        return Ok(());
    }
    pub fn mode3x(amount: u32, _mode: u32, next: String) -> Result<(), Error> {
        let mut mode = _mode;
        let mut wtf = false;
        if mode > 99 {
            mode = mode / 10;
            wtf = true;
        }
        let lan = Languages::from(&next);
        let list_typ1 = ListType::Adjectives;
        let mut list_typ2 = ListType::Nouns;
        if mode == 32 {
            list_typ2 = ListType::Names;
        }
        let mut lsg1 = ListGenerator::new(list_typ1, lan.clone());
        let mut lsg2 = ListGenerator::new(list_typ2, lan.clone());
        if mode == 31 || mode == 32 {
            fill_list(&mut lsg1);
            fill_list(&mut lsg2);
        }
        if mode == 33 {
            let names: Vec<&str> = next.split(":").collect();
            let name0 = String::from(names[0]);
            let name1 = String::from(names[1]);
            fill_list2(&mut lsg1, name0);
            fill_list2(&mut lsg2, name1);
        }
        let mut nounlist: GermanNounList = GermanNounList::new();
        if lan.is_german() {
            fill_german_nounlist(&mut nounlist);
        }

        let mut output = File::create(OUTPUT_NAME)?;
        for _i in 0..amount {
            let strang = lsg1.get_single_word();
            let strang2 = lsg2.get_single_word();

            let mut strong = format!("{}_{}", strang, strang2);

            if lan.is_german() && mode == 31 {
                //noun adjective
                strong = nounlist.get_adapted(strang2, strang);
            }

            if wtf {
                writeln!(output, "{}", strong)?;
            } else {
                print!("{}:{}\n", strong, _i);
            }
        }
        return Ok(());
    }
}
