mod parser;
mod symbolTable;
use std::env;

fn main() {
    let filenames = env::args().collect::<Vec<String>>();
    let filename = &filenames[1]; 
    let parser = parser::Parser::new(filename.to_owned());
  //  let file_to_write = parser.create_file(filename);
    let symbol_table = symbolTable::SymbolTable::new(); 
    loop {
    }
}
