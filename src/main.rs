
use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::process::*;

extern crate regex;

use regex::Regex;

fn main() {
    println!("====== {:?} =========", env::args());
    let app_args: Vec<String> = env::args().collect();

    let (query, mut rest_args) = app_args.split_at(2);

    println!("{:?}, {:?}", query, rest_args);
    println!("Searching for {}", query[1]);


    let mut f = File::open(".ctproject").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("{}",contents);

    let matching_line = contents.lines()
        .filter(|line| line.contains(&query[1]))
        .last().unwrap();


    println!("Matching line:\n{}", matching_line);
    let regex = Regex::new(r"^[^=]*=([^#]*)#?(.*)").unwrap();
    for capture in regex.captures_iter(matching_line){
        let command_with_args = &capture[1].replace("\"", "");
        let doc = &capture[2];
        println!("{}, {}", command_with_args, doc);
        let commands_vec: Vec<_> = command_with_args.split(" ").collect();
        let (command, mut args) = commands_vec.split_first().unwrap();

        let mut args_as_vect: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        args_as_vect.append(rest_args.to_vec().as_mut());

        println!("{:?} {:?}", args, args_as_vect);
        let mut s = Command::new(command)
            .args(args_as_vect)
            .spawn().unwrap();
        let output = s.wait_with_output();
        println!("{:?}", output.unwrap().stdout);

    }

}