#![feature(plugin, extern_prelude)]
extern crate colored;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;


use colored::*;
use file_finder::CTFile;
use man::CTMan;
use ports::CTPorts;
use rocket::http::ContentType;
use rocket::response::Content;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod cli;
pub mod extract;
pub mod file_finder;
pub mod ports;
pub mod man;
pub mod log;
#[macro_use]
pub mod banner;
#[macro_use]
pub mod ports_html;

pub fn show_banner(){
    let show_banner = env::var("CTNOBANNER").unwrap_or("false".to_string());
    if show_banner == "false" {
        println!("{}", BANNER!());
    }
}

pub fn init_project(){
    let path = Path::new(".ctproject");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    if path.exists(){
        println!("⚠️ .ctproject already exists");
        return;
    }
    let mut file = match File::create(&path) {
        Err(why) => panic!("❌ couldn't create {}: {}",
                           display,
                           why.description()),
        Ok(file) => file,
    };

    match file.write_all("run='your run command'\nbuild='your build command'\ntest='your test command'".as_bytes()) {
        Err(why) => {
            panic!("❌ couldn't write to {}: {}", display,
                   why.description())
        },
        Ok(_) => println!("{} successfully wrote to {}", "✔︎".green(), display),
    }
}


pub fn start_port_listening() {
    println!("👂 Started ports web server at http://localhost:1500, CTRL+C to exit...");
    start_rocket();
}

pub fn show_man(man_entry: Option<&str>, ct_file: Option<CTFile>) {
    if let Some(ct_file) = ct_file {
        if let Some(ct_man) = CTMan::all(&ct_file) {
            if man_entry.is_some() {
                if let Some(man) = ct_man.get(man_entry.unwrap()) {
                    man.print();
                }
            } else {
                ct_man.values().for_each(CTMan::print);
            }
        }
    }
}


pub fn start_rocket() {
}