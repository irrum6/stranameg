pub mod help;
pub mod rng;

pub mod languages;
pub mod config;

pub mod strgen;

pub mod fast_switch;
pub mod repl;

pub mod stringer {
    use std::fs::File;
    use std::io::{Error, Write};

    pub use super::help::help::print_help2 as print_help;
    pub use super::languages::languages::{
        EnglishLanguage, GeorgianLanguage, GermanLanguage, SupportedLanguages,
    };
    pub use super::config::config::{Config,Modes};

    pub use super::rng::rng::RNG;

    use super::strgen::string_generator_module::*;

    pub use super::fast_switch::fast_switch;
    pub use super::repl::repl::run_repl;

    pub fn run_generator(conf: &Config) -> Result<(), Error> {
        const OUTPUT_NAME: &str = "strings.textout";
        let mut sg = StringGenerator::default();
        sg.setup(&conf);
        let mut output = File::create(OUTPUT_NAME)?;

        for _i in 0..conf.get_amount() {
            let strang = sg.get();
            if conf.get_write_to_file() {
                writeln!(output, "{}", strang)?;
            } else {
                let mut strong = format!("{}:{}\n", strang, _i);
                if conf.get_write_indices() {
                    strong = format!("{}\n", strang);
                }
                print!("{}\n", strong);
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
        let v= strong.split(delimiter).collect::<Vec<&str>>();
        // return String::from(v[1]);
        let value = String::from(v[1]);
        return value;
    }
}
