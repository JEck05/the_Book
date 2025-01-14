use std::{env, process};
use minigrep_jeck as minigrep;
use minigrep::Config;

fn main() {

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing Arguments: {err}");
        process::exit(1)}
    );

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}
