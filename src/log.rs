use std::env;

lazy_static! {
    static ref DEBUG_ENABLED: String = env::var("CTDEBUG").unwrap_or("false".to_string());
}

pub fn debug_log<F>(func: F)
    where F: Fn() -> String{
    if *DEBUG_ENABLED == "true" {
        println!("{}", func());
    }
}