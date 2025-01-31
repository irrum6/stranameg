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
        let result = sg.setup(&conf);
        if result.is_err(){

        }else{

        }
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
        let conf = Config::new(&sarraya);

        let lstype = ListType::Adjectives;
        let lan = Languages::Georgian;
        let mut sg = CoupledWords::new(lstype, lan);

        sg.setup(&conf).ok();
        // match sg.setup(&conf){
        //     Ok() =>{},
        //     Err(e)=>
        // }
        let strong = sg.get();

        println!("{}",strong);

        let kachars = "აბგდევზთიკლმნოპჟრსტუფქღყშჩცძწჭხჯჰ";
        for c in strong.chars() {
            let check = kachars.contains(c) || c == ' ' || c =='_';
            assert_eq!(check, true);
        }
    }
    
    #[test]
    fn command_parser() {
        let vargs = vec!["mode=rla", "len=12", "num=16", "next=alphabet"];
        let confetti = command_parser::get_config(vargs);
        let mut sg = LettterSequence::new("abc", 12);
        let _= sg.setup(&confetti);
        let strong = sg.get();
        let alphabet = "alphabet";
        println!("{}", &strong);
        for s in strong.chars() {
            assert_eq!(alphabet.contains(s), true);
        }
        assert_eq!(sg.get().len(), 12);
    }

}
