mod parser;
mod symbolTable;
use std::env;
use std::fs::File;
fn main() {
    let filenames = env::args().collect::<Vec<String>>();
    let filename = filenames[1].clone();

    let mut file = match File::open(&filename) {
        Err(why) => panic!("could not read {} : {}", filename, why),
        Ok(file) => file,
    };
}
