## minigrep_jeck

This crate is a published version of my personal implementation of [The Rust Programming Languages](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) Chapter 12 Project. 
It is meant to be a simple [grep](https://www.gnu.org/software/grep/manual/grep.html) clone that works on any valid UTF-8 file.

It should be noted that I extended the basic I/O project to include Regex searches, using the [regexv1.11.1](https://crates.io/crates/regex/1.11.1) crate

### **Documentation**
[Library documentation with examples](https://docs.rs/minigrep_jeck/0.1.0).

### **Usage**
To bring this crate into scope, either add `minigrep_jeck@0.1.1` to your dependencies in `cargo.toml`, or run `cargo install minigrep_jeck@0.1.1`.

Here is an example that creates a new Rust project, adds a dependency on minigrep_jeck@0.1.1, creates the source code and a poem to search with a query.

```rust
use std::{env, process};
use minigrep_jeck::Config;
// If you were calling this from the command line it would look like
// cargo run -- "P" "homework.txt" --ignore-case
let vec1: Vec<String> = vec![
                "./minigrep.exe".to_string(),
                "P".to_string(),
                "homework.txt".to_string(),
                "--ignore-case".to_string()
];

let config = Config::build(vec1.into_iter()).unwrap_or_else(|err| { 
    eprintln!("Problem parsing Arguments: {err}");
    process::exit(1)}
);

if let Err(e) = minigrep_jeck::run(config) {
    eprintln!("Application Error: {e}"); 
}
```
