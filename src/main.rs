use std::env;

extern crate ct;
extern crate colored;

use colored::*;

use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::show_banner;
use std::string::String;

fn main() -> Result<(), String> {
    show_banner();
    let app_args: Vec<String> = env::args().collect();
    let ct_file = CTFile::get_content()?;

    let maybe_config = Config::new(app_args);
    match maybe_config {
        Ok(config) => run(&ct_file, config),
        Err(_) => {
            println!("{}", help(&ct_file));
            Ok(())
        }
    }


}

fn help(ct_file: &CTFile) -> String{
    let mut help : Vec<String> = Vec::new();
    for (alias, command )in RunCommand::all(&ct_file.content, None){
        help.push(format!("{} {} {} {}", alias.blue(), command.command.green(), command.args.join(" ").green(), command.doc.red()));
    }
    help.join("\n")
}

fn run(ct_file: &CTFile, config: Config) -> Result<(), String>{

    let all_commands = RunCommand::all(&ct_file.content, Some(&config));
    let command = all_commands.get(&config.command);
    match command {
        Some(run_command) => run_command.run(&ct_file),
        None => { println!("{}", help(&ct_file)); return Ok(()) }
    }

    if config.command.len() == 0{
        return Err("No argument given !".to_string())
    }

    Ok(())
}
