pub mod command_parser {
    use crate::stringer::{safe_u32,Config,Modes};

    pub fn get_config(vargs: Vec<&str>) -> Config {
        let a = [String::new(), String::from("02")];
        //confetti
        let mut conf = Config::new(&a);

        for str in vargs {
            if str.contains("mode") {
                let strong = str.to_string();
                let mo = Modes::from(get_value(strong, "=").as_ref());
                conf.set_mode(mo);
            }
            if str.contains("num") {
                let strong = str.to_string();
                let ammount: u32 = get_value(strong, "=").parse().expect("number");
                conf.set_amount(ammount);
            }
            if str.contains("len") {
                let strong = str.to_string();
                let length: u32 = safe_u32(get_value(strong, "="), 4);
                conf.set_length(length);
            }
            if str.contains("next") {
                let strong = str.to_string();
                conf.set_next(get_value(strong, "="));
            }
            if str.contains("wtf") {
                let strong = str.to_string();
                conf.set_write_to_file(get_value(strong, "=") == "1");
            }
            if str.contains("dwi") {
                let strong = str.to_string();
                conf.set_write_indices(get_value(strong, "=") == "1");
            }
        }
        return conf;
    }
    fn get_value(strong: String, delimiter: &str) -> String {
        // return ;
        let v: Vec<&str> = strong.split(delimiter).collect();
        // return String::from(v[1]);
        let value = String::from(v[1]);
        return value;
    }
}
