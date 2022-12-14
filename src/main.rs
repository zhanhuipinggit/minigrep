
use std::env;
use std::process;

extern crate minigrep;
use minigrep::Config;

fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) =  minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }



}
