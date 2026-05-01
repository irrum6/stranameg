#[cfg(test)]
pub mod tests {

    use stranameg::config::config::{CommandParser, Config};
    use stranameg::languages::languages::{GeorgianLanguage, SupportedLanguages};
    use stranameg::strgen::string_generator_module::StringGenerator;

    #[test]
    pub fn check_len() {
        let length = 12;
        let len = String::from("12");
        let ammount = String::from("16");
        let mode = String::from("rls");
        let lang = String::from("en");
        let sarraya: [String; 5] = [String::new(), ammount, len, mode, lang];
        let conf = Config::from(&sarraya);

        let mut sg = StringGenerator::default();
        sg.setup(&conf);
        assert_eq!(sg.get().len(), length);
    }
    #[test]
    pub fn is_georgian() {
        //check if generated string is actually georgian alphabet
        let len = String::from("12");
        let ammount = String::from("16");
        let mode = String::from("cow");
        let lang = String::from("ka");
        let sarraya: [String; 5] = [String::new(), ammount, len, mode, lang];
        let conf = Config::from(&sarraya);

        let lan = SupportedLanguages::Georgian(GeorgianLanguage::new());
        let mut sg = StringGenerator::default();

        sg.set_language(lan);

        sg.setup(&conf);

        let strong = sg.get();

        println!("{}", strong);

        let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰ";
        for c in strong.chars() {
            let check = kachars.contains(c) || c == ' ' || c == '_';
            assert_eq!(check, true);
        }
    }

    #[test]
    fn command_parser() {
        let vargs = vec!["mode=rla", "len=12", "num=16", "next=alphabet"];
        let confetti = CommandParser::parse_config(vargs);
        let mut sg = StringGenerator::default();
        let _ = sg.setup(&confetti);
        let strong = sg.get();
        let alphabet = "alphabet";
        println!("{}", &strong);
        for s in strong.chars() {
            assert_eq!(alphabet.contains(s), true);
        }
        assert_eq!(sg.get().len(), 12);
    }
}
