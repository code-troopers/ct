#![feature(plugin, extern_prelude)]
#![plugin(rocket_codegen)]
extern crate rocket;


#[macro_use]
extern crate serde_derive;

use cli::Config;
use file_finder::CTFile;
use std::env;

pub mod cli;
pub mod extract;
pub mod file_finder;
pub mod ports;
pub mod man;
#[macro_use]
pub mod banner;
#[macro_use]
pub mod ports_html;

use rocket::response::Content;
use rocket::http::ContentType;
use ports::CTPorts;

pub fn find_command(config: &Config, ct_file: &CTFile) -> String{
    println!("{}", ct_file.content);
    let matching_line: String = ct_file.content.lines()
        .filter(|line| line.starts_with(&config.command))
        .last()
        //build a "fake" command with the one the user tries to execute
        .map_or(format!("{}={}", &config.command, &config.command), |v| { v.to_string() });
    matching_line
}

pub fn show_banner(){
    let show_banner = env::var("CTNOBANNER").unwrap_or("false".to_string());
    if show_banner == "false" {
        println!("{}", BANNER!());
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

pub fn start_rocket() {
    let config = rocket::config::Config::build(rocket::config::Environment::Production)
        .port(1500)
        .finalize().expect("Could not create config");

    rocket::custom(config, false)
        .mount("/", routes![scan, home_page])
        .launch();
}