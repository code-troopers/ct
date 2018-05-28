extern crate clap;
extern crate colored;
extern crate ct;
extern crate serde_json;
extern crate linked_hash_map;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use colored::*;
use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::man::CTMan;
use ct::show_banner;
use ct::start_rocket;
use std::env;
use std::string::String;
use std::collections::HashMap;
use linked_hash_map::LinkedHashMap;


fn main() -> Result<(), String> {
    show_banner();
    let app_args: Vec<String> = env::args().collect();
    //see #16, do not block people from launching port from anywhere.
    let ct_file = CTFile::get_content().ok();

    let maybe_config = Config::new(app_args);
    match maybe_config {
        Ok(config) => run(ct_file, config),
        Err(_) => {
            println!("{}", help(ct_file));
            Ok(())
        }
    }
}

fn start_port_listening() {
    println!("👂 Started ports web server at http://localhost:1500, CTRL+C to exit...");
    start_rocket();
}

fn show_man(app_args: &Vec<String>, ct_file: Option<CTFile>) {
    if let Some(ct_file) = ct_file {
        if let Some(ct_man) = CTMan::all(&ct_file) {
            if app_args.len() > 2 {
                if let Some(man) = ct_man.get(&app_args[2..].join(" ")) {
                    man.print();
                }
            } else {
                ct_man.values().for_each(CTMan::print);
            }
        }
    }
}


fn help(ct_file: Option<CTFile>) -> String{
    let mut help : Vec<String> = Vec::new();
    help.push("Default commands :".green().to_string());
    help.push(format!("\t• {} runs a server on http://localhost:1500 to see other used ports 👂", "ports".blue()));
    help.push(format!("\t• {} provide manual from content {{name}} of README.md 📖", "man {name}".blue()));
    help.push(String::from(""));
    if let Some(file) = ct_file {
        help.push(format!("Declared aliases found in {} :", file.path).green().to_string());
        for (alias, command) in RunCommand::all(&file.content, None) {
            help.push(format!("\t• {} runs {} {} {}", alias.blue(), command.command.green(), command.args.join(" ").green(), command.doc.red()));
        }
    }
    help.join("\n")
}

fn run(ct_file: Option<CTFile>, config: Config) -> Result<(), String>{
    let args: Vec<App> = vec![SubCommand::with_name("man")
                                  .about("provide manual from content {{name}} of README.md 📖")
                                  .arg(Arg::with_name("name").help("extract content")),
                              SubCommand::with_name("ports")
                                  .about("runs a server on http://localhost:1500 to see other used ports 👂")];
    let all_commands = match ct_file {
        Some(ref ct_file) => RunCommand::all(&ct_file.content, Some(&config)),
        None => LinkedHashMap::new()
    };
    let commands_from_ctproject: Vec<App> = all_commands.iter()
        .map(|a| {
            SubCommand::with_name(&a.0).about(a.1.doc.as_str())
        }).collect();
    let app = App::new("ct - CLI helper")
        .version("0.1.1")
        .author("Code-Troopers <contact@code-troopers.com>")
        .about("Help you to handle your project easily")
        .subcommands(args.clone())
        .subcommands(commands_from_ctproject);
    let matches = app.get_matches();
    println!("{:?}", matches.subcommand);
    if let Some(command) = matches.subcommand {
        if args.iter().map(|a| a.get_name().to_string()).filter(|c| c == &command.name).count() > 0{
            println!("{:?}", command);
            match command.name.as_ref() {
                "ports" => start_port_listening(),
                "man" => show_man(&Vec::new(), ct_file),
                _ => println!(""),
            }

            return Ok(());
        }
        if let Some(ct_file) = ct_file {
            let command = all_commands.get(&command.name);
            match command {
                Some(run_command) => run_command.run(&ct_file),
                None => {
                    println!("{}", help(Some(ct_file)));
                    return Ok(())
                }
            }
        }
    }

    if config.command.len() == 0{
        return Err("⚠️ No argument given !".to_string())
    }

    Ok(())
}
