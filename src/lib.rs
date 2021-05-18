#![feature(plugin)]
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate lazy_static;
extern crate colored;

#[macro_use]
extern crate serde_derive;
extern crate itertools;


use colored::*;
use file_finder::CTFile;
use man::CTMan;
use ports::CTPorts;
use std::env;

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


use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode, HeaderMap};
use hyper::header::CONTENT_TYPE;


pub fn show_banner(){
    let show_banner = env::var("CTNOBANNER").unwrap_or_else(|_|"false".to_string());
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
                           why.to_string()),
        Ok(file) => file,
    };

    match file.write_all("run='your run command'\nbuild='your build command'\ntest='your test command'".as_bytes()) {
        Err(why) => {
            panic!("❌ couldn't write to {}: {}", display,
                   why.to_string())
        },
        Ok(_) => println!("{} successfully wrote to {}", "✔︎".green(), display),
    }
}


pub fn start_port_listening() {
    if CTPorts::available() {
        println!("👂 Started ports web server at http://localhost:1500, CTRL+C to exit...");
        start_hyper();
    }else{
        println!("🙉 Unable to start port server, please make sure lsof is available");
    }
}


pub fn show_man(man_entry: Option<&str>, help: bool, ct_file: Option<CTFile>) {
    if let Some(ct_file) = ct_file {
        if help{
            CTMan::help(&ct_file);
            return
        }
        if let Some(ct_man) = CTMan::all(&ct_file) {
            if man_entry.is_some() {
                if let Some(man) = ct_man.get(man_entry.unwrap()) {
                    man.print();
                }
            } else {
                ct_man.values().for_each(|m| m.print());
            }
        }
    }
}

fn scan() -> String {
    serde_json::to_string(&CTPorts::all().unwrap()).unwrap()
}


fn home_page() -> &'static str {
    INDEX_HTML!()
}

type BoxFut = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn echo(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            let mut map = HeaderMap::new();

            map.insert(CONTENT_TYPE, "text/html;charset=utf-8".parse().unwrap());

            *response.headers_mut() = map;
            *response.body_mut() = Body::from(home_page());
        }
        (&Method::GET, "/scan") => {
            *response.body_mut() = Body::from(scan());
        }
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Box::new(future::ok(response))
}

pub fn start_hyper() {
    let addr = ([0, 0, 0, 0], 1500).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(echo))
        .map_err(|e| eprintln!("server error: {}", e));


    hyper::rt::run(server);
}
