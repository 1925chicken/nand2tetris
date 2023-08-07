use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::Bytes;
use std::path::Path;


#[derive(Eq,PartialEq)]
enum CommandType {
    ACommand,
    CCommand,
    LCommand,
}

pub struct Parser {
    input_file: File,
    output_file: File,
}

impl Parser {

    pub fn new(file_name: String) -> Self {
        let path = Path::new(&file_name);
        let display = path.display();
        let input_file = match File::open(&path) {
            Err(why) => panic!("couldn't open {} : {}", display, why),
            Ok(file) => file,
        };
        let new_file_name = file_name.split('.').next().unwrap().to_string();
        let output_file = match File::create("assembler.asm".to_owned()) {
            Err(why) => panic!("couldn't create file : {}", why),
            Ok(file) => file,
        };
        Self {
            input_file,
            output_file,
        }
    }

    fn create_file(&self, new_file_name: &String) -> File {
        match File::create(Path::new(&new_file_name)) {
            Err(why) => panic!("couldn't create file : {}", why),
            Ok(file) => file,
        }
    }

    fn has_more_command(&self, line: &str) -> bool {
        line.bytes().next() == Some(0)
    }

    fn advance(&self, line: &str) -> String {
        line.split("////")
            .next()
            .unwrap()
            .to_string()
            .trim()
            .to_owned()
    }

    fn command_type(&self, command: &str) -> CommandType {
        let initial = &command[..1];
        match initial {
            "@" => CommandType::ACommand,
            "(" => CommandType::LCommand,
            _ => CommandType::CCommand,
        }
    }

    fn symbol(&self,command:&str, command_type: CommandType) -> String {
        if command_type == CommandType::ACommand {
            command[1..].to_string()
        }else if command_type == CommandType::LCommand {
            command[1..command.len() - 1].to_string()
        }else {
            panic!("invalid Command!!");
        }
    }

}
