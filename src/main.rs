use std::env;

extern crate ct;

use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::find_command;

fn main() -> Result<(), String> {
    let app_args: Vec<String> = env::args().collect();
    let config = Config::new(app_args);

    let ct_file = CTFile::get_content()?;

    if config.command.len() == 0{
        println!("{}", ct_file.content);
        return Err("No argument given !".to_string())
    }

    let command = find_command(&config, &ct_file);

    let run_command = RunCommand::new(&command, config)?;
    run_command.run(&ct_file);
    Ok(())
}
