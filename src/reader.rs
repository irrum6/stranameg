pub mod reader {
    use std::fs::File;
    use std::io::{self, BufRead, ErrorKind};
    use std::path::Path;

    //copied from rust site and modified
    pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        match File::open(filename) {
            Ok(file) => return Ok(io::BufReader::new(file).lines()),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("File not found"),
                _ => panic!("Error when opening file"),
            },
        };
    }
}
