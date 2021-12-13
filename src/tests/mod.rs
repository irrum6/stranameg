#[cfg(test)]
pub mod tests {
    use crate::{run_generator, Config, LettterSequence, StringGenerator};
    #[test]
    pub fn check_len() {
        let length = 12;
        let len = String::from("12");
        let ammount = String::from("16");
        let mode = String::from("rls");
        let lang = String::from("en");
        let sarraya: [String; 5] = [String::new(), ammount, len, mode, lang];
        let conf = Config::new(&sarraya);

        let mut sg = LettterSequence::new("abc", length);
        sg.setup(conf);
        assert_eq!(sg.get().len(), length);
    }
    // #[test]
    // pub fn is_georgian() {
    //     //check if generated string is actually georgian alphabet
    //     let lst = ListType::Nouns;
    //     let lan = Languages::Georgian;
    //     let mut lsg = ListGenerator::new(lst, lan);
    //     fill_list(&mut lsg);
    //     let len = 32;
    //     let strong = lsg.get(len as usize);

    //     let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰ";
    //     for c in strong.chars() {
    //         let check = kachars.contains(c) || c == ' ';
    //         assert_eq!(check, true);
    //     }
    // }
}
