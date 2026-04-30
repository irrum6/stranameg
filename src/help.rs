pub mod help {
    pub fn print_help() {
        print!("help");
    }
    pub fn print_help2() {
        use std::fs::read_to_string;
        let read_hs = read_to_string("HELP.md");

        match read_hs{
            Ok(help_string)=>println!("{}", help_string),
            Err(_e)=>println!("Something went wrong while reading HELP.md file")
        }
    }
}
