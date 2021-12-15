pub mod help {
    pub fn print_help() {
        println!("help");
    }
    pub fn print_help2() {
        use std::fs::read_to_string;
        let help_string = read_to_string("HELP.md").expect("Something went wrong reading the file");
        println!("{}", help_string);
    }
}
