pub mod help;
pub mod reader;

pub mod stringer {

    pub use super::help::help::print_help2 as print_help;
    // pub use super::strgen::strgen::*;
    pub use super::reader::reader::read_lines;

}
