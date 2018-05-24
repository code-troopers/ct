extern crate regex;
extern crate linked_hash_map;
use self::regex::Regex;
use self::linked_hash_map::*;
use std::process::*;
use cli::Config;
use file_finder::CTFile;

#[derive(Debug)]
pub struct RunCommand {
    pub command: String,
    pub args: Vec<String>,
    pub doc: String,
}

impl RunCommand{
    pub fn all<'a>(file_content: &'a str, config: Option<&Config>) -> LinkedHashMap<String, RunCommand>{
        let regex = Regex::new(r"(?m)^\s*([^=]*)=([^#\n]*)(#\s*(.*)\s*)?$").unwrap();
        let mut commands: LinkedHashMap<String, RunCommand> = LinkedHashMap::new();
        for capture in regex.captures_iter(file_content){
            let alias = &capture[1];
            let command_with_args = &capture[2].replace("\"", "").replace("'", "");

            let doc = capture.get(4).map(|m| m.as_str()).map(ToString::to_string).unwrap_or(String::from(""));
            let commands_vec: Vec<_> = command_with_args.split(" ").collect();
            let (command, args) = commands_vec.split_first().unwrap();

            let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
            if config.is_some() {
                args_as_vect.append(&mut config.unwrap().args.clone());
            }
            args_as_vect = args_as_vect.into_iter().filter(|a| { a.len() > 0 }).collect();

            commands.insert(alias.to_string(), RunCommand{command: command.to_string(), args: args_as_vect, doc});;
        }
        commands
    }

    pub fn run(&self, ct_file: &CTFile){
//        println!(">> {:?}, {:?}", &self.command, &self.args);
        let mut sh_sub_command = Vec::new();
        sh_sub_command.push(self.command.to_string());
        sh_sub_command.push(String::from(" "));
        sh_sub_command.push(self.args.join(" ")); // no need to escape "', it is properly handled
        let s = Command::new("sh")
            .arg("-c")
            .arg(sh_sub_command.join(""))
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
    fn it_should_match_three_commands_without_comment(){
        let map = RunCommand::all(r"command=run tests
        command2=run app
        command3=push commits
        ", None);
        assert_eq!(map.len(), 3);
        assert_eq!(map.contains_key("command"), true);
        assert_eq!(map.contains_key("command2"), true);
        assert_eq!(map.contains_key("command3"), true);
    }

    #[test]
    fn it_should_match_command_with_leading_spaces(){
        let map = RunCommand::all("   command=run tests", None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key("command"), true);
    }

    #[test]
    fn it_should_match_command_with_doc(){
        let map = RunCommand::all("command=run tests # this run tests", None);
        assert_eq!(map.len(), 1);
        let run_command = map.get("command").unwrap();
        assert_eq!(run_command.command, "run");
        assert_eq!(run_command.args.join(""), "tests");
        assert_eq!(run_command.doc, "this run tests");

    }

    #[test]
    fn it_should_match_command_with_leading_tab(){
        let map = RunCommand::all("\tcommand=run tests", None);
        assert_eq!(map.len(), 1);
        assert_eq!(map.contains_key("command"), true);
    }


    #[test]
    #[should_panic]
    fn it_should_error_if_line_does_not_match_pattern(){
        let config = Config::new(vec!["ct", "command"].into_iter().map(ToString::to_string).collect()).unwrap();
        let map = RunCommand::all("command", Some(&config));
        let _run_command = map.get("command").unwrap();
    }
}