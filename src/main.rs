use std::env;

extern crate ct;

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
    return match maybe_config {
        Ok(config) => run(&ct_file, config),
        Err(_) => Err(help(&ct_file))
    }

}

fn help(ct_file: &CTFile) -> String{
    let all_commands = RunCommand::all(&ct_file.content, None);
    format!("{:?}", all_commands)
}

fn run(ct_file: &CTFile, config: Config) -> Result<(), String>{

    let all_commands = RunCommand::all(&ct_file.content, Some(&config));
    let command = all_commands.get(&config.command);
    match command {
        Some(run_command) => run_command.run(&ct_file),
        None => return Err(help(&ct_file))
    }

    if config.command.len() == 0{
        return Err("No argument given !".to_string())
    }

    Ok(())
}
