pub mod modes {
    use std::fs::read_to_string as fs_read;

    use crate::{
        fill_list, fill_list2, ABCGenerator, Languages, ListGenerator, ListType, StringGenerator,
    };

    fn lang_mapper(s: &String) -> Languages {
        let result = match s.as_ref() {
            "11" => Languages::Georgian,
            "12" => Languages::English,
            _ => Languages::English,
        };
        return result;
    }

    pub fn mode1x(len: u32, amount: u32, mode: u32, next: String) {
        let latinchars = "abcdefghijklmnopqrstuvwxyzaaaaeeeiiiooouuy";
        let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰააააეეეიიიოოოუუ";

        let mut sg = ABCGenerator::new(latinchars);

        if mode == 10 && next == "11" {
            sg.set_alphabet(kachars);
        }
        if mode == 11 {
            sg.set_alphabet(next.as_ref());
        }
        if mode == 12 {
            let contents =
                fs_read(next.clone().trim()).expect("Something went wrong reading the file");
            sg.set_alphabet(contents.as_ref());
        }
        for _i in 0..amount {
            let strang = sg.get(len as usize);
            print!("{}:{}\n", strang, _i);
        }
    }
    pub fn mode2x(len: u32, amount: u32, mode: u32, next: String) {
        let lst = ListType::Nouns;
        let lan = lang_mapper(&next);
        let mut lsg = ListGenerator::new(lst, lan);
        if mode == 21 {
            fill_list(&mut lsg);
        }
        if mode == 22 {
            fill_list2(&mut lsg, next.clone());
        }
        for _i in 0..amount {
            let strang = lsg.get(len as usize);
            print!("{}:{}\n", strang, _i);
        }
        return;
    }
    pub fn mode3x(amount: u32, mode: u32, next: String) {
        let lan = lang_mapper(&next);
        let list_typ1 = ListType::Adjectives;
        let mut list_typ2 = ListType::Nouns;
        if mode == 32 {
            list_typ2 = ListType::Names;
        }
        let mut lsg1 = ListGenerator::new(list_typ1, lan.clone());
        let mut lsg2 = ListGenerator::new(list_typ2, lan);
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
        for _i in 0..amount {
            let strang = lsg1.get_single_word();
            let strang2 = lsg2.get_single_word();
            let strong = format!("{}_{}", strang, strang2);
            print!("{}:{}\n", strong, _i);
        }
        return;
    }
}
