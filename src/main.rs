
use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::process::*;

extern crate regex;
extern crate ct;

use ct::cli::Config;

use regex::Regex;

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
    let regex = Regex::new(r"^[^=]*=([^#]*)#?(.*)").unwrap();
    for capture in regex.captures_iter(&matching_line){
        let command_with_args = &capture[1].replace("\"", "");
        let doc = &capture[2];
        println!("Command with args  {} and doc {}", command_with_args, doc);
        let commands_vec: Vec<_> = command_with_args.split(" ").collect();
        let (command, args) = commands_vec.split_first().unwrap();

        let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        args_as_vect.append(&mut config.args.clone());

        println!("Args : {:?} {:?}", args, args_as_vect);
        let mut s = Command::new(command)
            .args(args_as_vect)
            .spawn().unwrap();
        //result printed to stdout / stderr as expected as io are shared
        let _output = s.wait_with_output();
        // println!(">> {:?}", output.unwrap().stdout);
    }
}
