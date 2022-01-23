use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // the use of the eprintln!() macro redirects output to std errror stream instead of stdout
        eprintln!("[ ERROR ] Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // here we can handle the error that MIGHT get returned from the run function
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
