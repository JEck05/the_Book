use std::error::Error;
use std::fs;


pub struct Config{
    pub query: String,
    pub file_path: String,
}
impl Config{
    pub fn new(args: &[String]) -> Self{
        if args.len() < 3 {
            panic!("Not enough arguments ")
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
    pub fn build(args: &[String]) -> Result<Self, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();

        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = search(&config.query, &contents);
    
    for lines in results {
        println!("{lines}");
    }

    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for lines in contents.lines() {
        if lines.contains(query) {
            results.push(lines);
        }
    }
    results
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents =   "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}