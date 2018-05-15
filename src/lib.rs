use cli::Config;
use file_finder::CTFile;

pub mod cli;
pub mod extract;
pub mod file_finder;

pub fn find_command(config: &Config, ct_file: &CTFile) -> String{
    let matching_line: String = ct_file.content.lines()
        .filter(|line| line.contains(&config.command))
        .last()
        //build a "fake" command with the one the user tries to execute
        .map_or(format!("{}={}", &config.command, &config.command), |v| { v.to_string() });
    matching_line
}