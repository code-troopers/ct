use std::env;
use std::fs::File;
use std::io::prelude::*;

extern crate ct;

use ct::cli::Config;
use ct::extract::RunCommand;

fn main() {
    println!("====== {:?} =========", env::args());
    let app_args: Vec<String> = env::args().collect();
    let config = Config::new(app_args);

    println!("{:?}, {:?}", config.command, config.args);

    let mut f = File::open(".ctproject").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let matching_line: String = contents.lines()
        .filter(|line| line.contains(&config.command))
        .last()
        //build a "fake" command with the one the user tries to execute
        .map_or(format!("{}={}", &config.command, &config.command), |v| { v.to_string() });


    println!("Matching line:\n{}", matching_line);
    let run_command = RunCommand::new(&matching_line, config).unwrap();
    run_command.run();
}
