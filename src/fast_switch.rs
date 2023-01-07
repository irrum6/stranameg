pub mod fast_switch{
    use crate::stringer::Config;
    use crate::stringer::Modes;
    pub fn get_fswitch_conf(onkstr:String)->Config{
        let mut conf = Config::default();
        let split: Vec<&str> = onkstr.split("-f").collect();
        // println!("{}",split[0]);
        let s: String = String::from(split[1]);
        let zero = s.chars().nth(0).unwrap();
        if 'n' == zero {
            // number
            // -fn16
            let split: Vec<&str> = s.split("n").collect();
            let num: u32 = split[1].parse().expect("number isnot?!");
            conf.set_amount(num);
        } else if 's' == zero {
            // length/size
            // -fs32
            let split: Vec<&str> = s.split("s").collect();
            let num: u32 = split[1].parse().expect("number isnot?!");
            conf.set_length(num);
        } else if 'm' == zero {
            // mode
            // -fmrla
            let split: Vec<&str> = s.split("m").collect();
            let mode = Modes::from(split[1]);
            conf.set_mode(mode);
        } else if 'l' == zero {
            // language
            // -flka
            let split: Vec<&str> = s.split("l").collect();
            let la = String::from(split[1]);
            conf.set_next(la);
        } else {
            // do nothing
        }
        return conf;
    }
}