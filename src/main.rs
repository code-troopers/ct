use std::env;

extern crate ct;

use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::find_command;

fn main() {
    let app_args: Vec<String> = env::args().collect();
    let config = Config::new(app_args);

    let ct_file = CTFile::get_content();
    let command = find_command(&config, &ct_file);


    let run_command = RunCommand::new(&command, config).unwrap();
    run_command.run();
}
