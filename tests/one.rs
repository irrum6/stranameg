#[cfg(test)]
pub mod tests {
    // use stranameg::stringer::languages::languages::Languages;
    use stranameg::stringer::{Config, Languages, ListType, command_parser};

    use stranameg::strgen::string_generator_module::*;
    #[test]
    fn unnamed() {
        use stranameg::stringer::print_help;
        print_help();
        assert_eq!(1, 1);
    }
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
        sg.setup(&conf);
        assert_eq!(sg.get().len(), length);
    }
    #[test]
    pub fn is_georgian() {
        //check if generated string is actually georgian alphabet
        let len = String::from("12");
        let ammount = String::from("16");
        let mode = String::from("raw");
        let lang = String::from("ka");
        let sarraya: [String; 5] = [String::new(), ammount, len, mode, lang];
        let conf = Config::new(&sarraya);

        let lst = ListType::Nouns;
        let lan = Languages::Georgian;
        let mut sg = RandomWord::new(lst, lan);
        sg.setup(&conf);
        let strong = sg.get();

        let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰ";
        for c in strong.chars() {
            let check = kachars.contains(c) || c == ' ';
            assert_eq!(check, true);
        }
    }
    #[test]
    pub fn full_list() {
        let ammount = String::from("16");
        let sarraya: [String; 2] = [String::new(), ammount];
        let conf = Config::new(&sarraya);
        let english = Languages::English;
        let nouns = ListType::Nouns;
        let mut sg = RandomWord::new(nouns, english);
        // stringer
        sg.setup(&conf);
        //get_list_len
        assert_ne!(sg.get_list_len(), 0);
    }
    #[test]
    fn command_parser() {
        let vargs = vec!["mode=rla", "len=12", "num=16", "next=alphabet"];
        let confetti = command_parser::get_config(vargs);
        let mut sg = LettterSequence::new("abc", 12);
        sg.setup(&confetti);
        let strong = sg.get();
        let alphabet = "alphabet";
        println!("{}", &strong);
        for s in strong.chars() {
            assert_eq!(alphabet.contains(s), true);
        }
        // assert_eq!(sg.get().len(), 12);
    }
}
