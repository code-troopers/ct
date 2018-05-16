use std::env;

extern crate ct;

use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::find_command;
use ct::show_banner;
use std::string::String;

fn main() -> Result<(), String> {
    show_banner();
    let app_args: Vec<String> = env::args().collect();
    let config = Config::new(app_args);

    let ct_file = CTFile::get_content()?;

    let all_commands = RunCommand::all(&ct_file.content, &config);
    let command = all_commands.get(&config.command);
    match command {
        Some(run_command) => run_command.run(&ct_file),
        None => return Err(format!("{:?}", all_commands))
    }

    if config.command.len() == 0{
//        println!("{}", all_commands.iter().map(|s| s.command ).collect::Vec<String>());
        return Err("No argument given !".to_string())
    }

//    let command = find_command(&config, &ct_file);

//    let run_command = RunCommand::new(&command, config)?;
//    run_command.run(&ct_file);
    Ok(())
}
