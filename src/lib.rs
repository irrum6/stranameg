pub mod help;
pub mod config;
pub mod languages;
pub mod strgen;

mod fast_switch;
mod repl;

pub mod stringer {
    use std::fs::File;
    use std::io::{Error, Write};

    pub use super::help::help::print_help2 as print_help;

    pub use super::fast_switch::fast_switch;
    pub use super::repl::repl::run_repl;

    use super::config::config::Config;
    use super::strgen::string_generator_module::StringGenerator;

    pub fn run_generator(conf: &Config) -> Result<(), Error> {
        let mut output_file_name: &str = "strings.textout";
        let out2 = conf.get_output_filename();
        if out2.len() > 0 {
            output_file_name = out2;
        }
        let mut sg = StringGenerator::default();
        sg.setup(&conf);
        let mut output = File::create(output_file_name)?;

        for _i in 0..conf.get_amount() {
            let strang = sg.get();
            let mut strong = format!("{}:{}\n", strang, _i);
            //it runs if dont_write_indices is true
            if conf.get_write_indices() {
                strong = format!("{}\n", strang);
            }

            if conf.get_write_to_file() {
                writeln!(output, "{}", strong)?;
            } else {
                println!("{}\n", strong);
            }
        }
        return Ok(());
    }

    //returns default value as safe, when it can't parse string
    pub fn safe_u32(strong: String, default: u32) -> u32 {
        return match strong.parse() {
            Ok(value) => value,
            Err(_e) => default,
        };
    }
    //command parser functions
    pub fn get_value(strong: String, delimiter: &str) -> String {
        // return ;
        //going fishing
        let v = strong.split(delimiter).collect::<Vec<&str>>();
        // return String::from(v[1]);
        let value = String::from(v[1]);
        return value;
    }

    pub enum ListType{
        Adjectives,
        Nouns,
        Names
    }

}
