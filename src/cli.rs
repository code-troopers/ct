use crate::errors::CTErrors;

pub struct Config {
    pub command: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Result<Config, CTErrors>{
        if args.len() == 1 { //no-args
            return Err(CTErrors::CLI)
        }
        let (query, rest_args) = args.split_at(2);
        Ok(Config { command: query[1].to_owned(), args: rest_args.to_vec() })
    }
}