## minigrep_jeck

This crate is a published version of my personal implementation of [The Rust Programming Languages](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) Chapter 12 Project. 
It is meant to be a simple [grep](https://www.gnu.org/software/grep/manual/grep.html) clone that works on any valid UTF-8 file.

It should be noted that I extended the basic I/O project to include Regex searches, using the [regexv1.11.1](https://crates.io/crates/regex/1.11.1) crate

### **Documentation**
[Library documentation with examples](https://docs.rs/minigrep_jeck/0.1.0).

### **Usage**
To bring this crate into scope, either add `minigrep_jeck@0.1.1` to your dependencies in `cargo.toml`, or run `cargo add minigrep_jeck`.

Here is an example that creates a new Rust project, adds minigrep_jeck, and creates the source code to search a poem with a query.

First, create a new directory and create a new cargo instance:
```bash
$ mkdir minigrep_jeck_example
$ cd minigrep_jeck_example
$ cargo init
```
Second, add `minigrep_jeck` to dependancies
```bash
$ cargo add minigrep_jeck
```
Third, create a new text document in current directory, this is the file we are going to search.
```bash
$ touch shakespeare.txt
```
Next, Input into the file what you want to search for a query, in this case I used `Shakespeare's Sonnet 18`

In shakespeare.txt:
```text
Shall I compare thee to a summer’s day?
Thou art more lovely and more temperate.
Rough winds do shake the darling buds of May,
And summer’s lease hath all too short a date.
Sometime too hot the eye of heaven shines,
And often is his gold complexion dimmed;
And every fair from fair sometime declines,
By chance, or nature’s changing course, untrimmed;
But thy eternal summer shall not fade,
Nor lose possession of that fair thou ow’st,
Nor shall death brag thou wand’rest in his shade,
When in eternal lines to Time thou grow’st.
So long as men can breathe, or eyes can see,
So long lives this, and this gives life to thee.
```

Then, go to `src/main.txt`. Replace the contents with:

```rust
use std::{env, process};
use minigrep_jeck::Config;

fn main(){
    let config = Config::build(env::args()).unwrap_or_else(
        |err|{ 
            eprintln!("Problem parsing Arguments: {err}");
            process::exit(1)
        }
    );

    if let Err(e) = minigrep_jeck::run(config) {
        eprintln!("Application Error: {e}");
        process::exit(1);
    }
}
```
Finally, run it with `cargo run -- "thou" "shakespeare.txt"`:
```
$ cargo run -- "thou" "shakespeare.txt"
   Compiling memchr v2.7.4
   Compiling regex-syntax v0.8.5
   Compiling aho-corasick v1.1.3                                                                                                                                                   
   Compiling regex-automata v0.4.9                                                                                                                                                 
   Compiling regex v1.11.1                                                                                                                                                         
   Compiling minigrep_jeck v0.1.0                                                                                                                                                  
   Compiling minigrep_jeck_example v0.1.0 (C:\Users\James\RustroverProjects\minigrep_jeck_example)                                                                                 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 7.43s                                                                                                            
     Running `target\debug\minigrep_jeck_example.exe thou shakespeare.txt`
Nor lose possession of that fair thou ow’st,
Nor shall death brag thou wand’rest in his shade,
When in eternal lines to Time thou grow’st.
```
This returns lines that only match "`thou`", this is case-sensitive. To run it case-insensitive run with the flag "`--ignore-case`".

Also you could use a regex string with the flag "`--is-regex`".

