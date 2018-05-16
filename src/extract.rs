extern crate regex;

use std::process::*;
use cli::Config;
use self::regex::Regex;
use file_finder::CTFile;

pub struct RunCommand {
    pub command: String,
    pub args: Vec<String>
}

impl RunCommand{
    pub fn new(matching_line: &str, config: Config) -> Result<RunCommand, String>{
        let regex = Regex::new(r"^[^=]*=([^#]*)#?(.*)").unwrap();
        for capture in regex.captures_iter(matching_line){
            let command_with_args = &capture[1].replace("\"", "").replace("'", "");
            let _doc = &capture[2];
            let commands_vec: Vec<_> = command_with_args.split(" ").collect();
            let (command, args) = commands_vec.split_first().unwrap();

            let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            args_as_vect.append(&mut config.args.clone());
            args_as_vect = args_as_vect.into_iter().filter(|a| { a.len() > 0 }).collect();

            return Ok(RunCommand{command: command.to_string(), args: args_as_vect})
        }
        Err("Could not find any command".to_owned())
    }

    pub fn run(&self, ct_file: &CTFile){
        println!(">> {:?}, {:?}", &self.command, &self.args);
        let s = Command::new(&self.command)
            .args(&self.args)
            .current_dir(ct_file.path.clone())
            .spawn().unwrap();
        //result printed to stdout / stderr as expected as io are shared
        let _output = s.wait_with_output();
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_should_extract_single_quoted_command(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect());
        let run_command = RunCommand::new("command='run'", config).unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_extract_double_quoted_command(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect());
        let run_command = RunCommand::new("command=\"run\"", config).unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_extract_not_quoted_command(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect());
        let run_command = RunCommand::new("command=run", config).unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_append_args_to_run_command_if_no_args_in_run_command(){
        let config = Config::new(vec!["ct", "command", "arg1", "arg2"].into_iter().map(ToString::to_string).collect());
        let run_command = RunCommand::new("command=run", config).unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "arg1 arg2");
    }

    #[test]
    fn it_should_append_args_to_run_command_if_args_in_run_command(){
        let config = Config::new(vec!["ct", "command", "arg1", "arg2"].into_iter().map(ToString::to_string).collect());
        let run_command = RunCommand::new("command=run tests", config).unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "tests arg1 arg2");
    }

    #[test]
    #[should_panic]
    fn it_should_error_if_line_does_not_match_pattern(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect());
        let _run_command = RunCommand::new("command", config).unwrap();
    }
}