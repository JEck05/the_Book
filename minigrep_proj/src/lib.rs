use std::error::Error;
use std::fs;
use regex::Regex;

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub is_regex: bool,
}
impl Config{
    pub fn build(args: &[String]) -> Result<Self, &'static str>{
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();

        let file_path = args[2].clone();

        let ignore_case = args.contains(&String::from("--ignore-case"));

        let is_regex = args.contains(&String::from("--is-regex"));

        Ok(Config { query, file_path, ignore_case, is_regex })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else if config.is_regex {
        search_regex(&config.query, &contents)
    }else{
        search(&config.query, &contents)
    };
    
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
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    let query = query.to_lowercase();

    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query) {
            results.push(lines);
        }
    }
    results
}
pub fn search_regex<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    let regex = Regex::new(query).unwrap();
    for lines in contents.lines() {
        if regex.is_match(lines) {
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
    #[test]
    fn regex(){
        let query = "[A-Z].*";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:", "Pick three."] , search_regex(query, contents));
    }
    #[test]
    fn case_insensitive(){
        let query = "P";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search_case_insensitive(query, contents));
    }
}