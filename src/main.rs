#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

use std::env;

#[macro_use]
extern crate ct;
extern crate colored;
extern crate serde_json;


use colored::*;

use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::show_banner;
use std::string::String;
use ct::ports::CTPorts;
use rocket::response::Content;
use rocket::http::ContentType;

fn main() -> Result<(), String> {
    show_banner();
    let app_args: Vec<String> = env::args().collect();
    let ct_file = CTFile::get_content()?;

    if app_args.len() > 0 && app_args[1] == "ports" {
        println!("Started ports web server at http://localhost:1500, CTRL+C to exit...");
        start_rocket();
        return Ok(())
    }

    let maybe_config = Config::new(app_args);
    match maybe_config {
        Ok(config) => run(&ct_file, config),
        Err(_) => {
            println!("{}", help(&ct_file));
            Ok(())
        }
    }


}

#[get("/scan")]
fn scan() -> String {
    serde_json::to_string(&CTPorts::all().unwrap()).unwrap()
}

#[get("/", format = "text/html")]
fn home_page() -> Content<String> {
    Content(ContentType::HTML, INDEX_HTML!().to_string())
}

fn start_rocket() {
    let config = rocket::config::Config::build(rocket::config::Environment::Production)
        .port(1500)
        .finalize().expect("Could not create config");

    rocket::custom(config, false)
        .mount("/", routes![scan, home_page])
        .launch();
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
