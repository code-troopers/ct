pub struct Config {
    pub command: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(args: Vec<String>) -> Config {
        let (query, rest_args) = args.split_at(2);
        Config { command: query[1].to_owned(), args: rest_args.to_vec() }
    }
}