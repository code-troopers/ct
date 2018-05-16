extern crate regex;

use std::process::*;
use cli::Config;
use self::regex::Regex;
use file_finder::CTFile;
use std::collections::HashMap;

#[derive(Debug)]
pub struct RunCommand {
    pub command: String,
    pub args: Vec<String>,
    pub doc: String,
}

impl RunCommand{
//    pub fn new<'a>(matching_line: &'a str, config: &Config) -> Result<RunCommand, String>{
//        let all = RunCommand::all(matching_line, config);
//        let maybe_command = all.get(matching_line);
//        match maybe_command{
//            Some(command) => Ok(command),
//            None => Err("Command not found".to_string())
//        }
//    }

    pub fn all<'a>(file_content: &'a str, config: Option<&Config>) -> HashMap<String, RunCommand>{
        let regex = Regex::new(r"([^=]*)=([^#]*)#?(.*)\n?").unwrap();
        let mut commands: HashMap<String, RunCommand> = HashMap::new();
        for capture in regex.captures_iter(file_content){
        //    println!("> {}", &capture[0]);
            let alias = &capture[1];
            let command_with_args = &capture[2].replace("\"", "").replace("'", "");
            let doc = capture[3].to_string();
            let commands_vec: Vec<_> = command_with_args.split(" ").collect();
            let (command, args) = commands_vec.split_first().unwrap();

            let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            if config.is_some() {
                args_as_vect.append(&mut config.unwrap().args.clone());
            }
            args_as_vect = args_as_vect.into_iter().filter(|a| { a.len() > 0 }).collect();

            commands.insert(alias.to_string(), RunCommand{command: command.to_string(), args: args_as_vect, doc});;
        }
        //let mut map = HashMap::new();
//        map.insert(String::from("toto"), String::from("yaaa"));
//        map
        commands
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
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command='run'", Some(&config));
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_extract_double_quoted_command(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command=\"run\"", Some(&config));
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_extract_not_quoted_command(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command=run", Some(&config));
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "");
    }

    #[test]
    fn it_should_append_args_to_run_command_if_no_args_in_run_command(){
        let config = Config::new(vec!["ct", "command", "arg1", "arg2"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command=run", Some(&config));
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "arg1 arg2");
    }

    #[test]
    fn it_should_append_args_to_run_command_if_args_in_run_command(){
        let config = Config::new(vec!["ct", "command", "arg1", "arg2"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command=run tests", Some(&config));
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(" "), "tests arg1 arg2");
    }

    #[test]
    #[should_panic]
    fn it_should_error_if_line_does_not_match_pattern(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command", Some(&config));
        let _run_command = map.get("command").unwrap();
    }
}