extern crate regex;

use std::process::*;
use cli::Config;
use self::regex::Regex;

pub struct RunCommand {
    pub command: String,
    pub args: Vec<String>
}

impl RunCommand{
    pub fn new(matching_line: &str, config: Config) -> Result<RunCommand, String>{
        let regex = Regex::new(r"^[^=]*=([^#]*)#?(.*)").unwrap();
        for capture in regex.captures_iter(matching_line){
            let command_with_args = &capture[1].replace("\"", "").replace("'", "");
            let doc = &capture[2];
            println!("Command with args  {} and doc {}", command_with_args, doc);
            let commands_vec: Vec<_> = command_with_args.split(" ").collect();
            let (command, args) = commands_vec.split_first().unwrap();

            let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            args_as_vect.append(&mut config.args.clone());

            println!("Args : {:?} {:?}", args, args_as_vect);
            return Ok(RunCommand{command: command.to_string(), args: args_as_vect})
        }
        Err("Could not find any command".to_owned())
    }

    pub fn run(&self){
        let s = Command::new(&self.command)
            .args(&self.args)
            .spawn().unwrap();
        //result printed to stdout / stderr as expected as io are shared
        let _output = s.wait_with_output();
    }
}
