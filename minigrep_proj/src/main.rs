use std::{env, process};

use minigrep::Config;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing Arguments: {err}");
        process::exit(1)}
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error {e}");
        process::exit(1);
    }

}
