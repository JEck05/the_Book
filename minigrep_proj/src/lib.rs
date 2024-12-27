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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str>{
        // Moves the iterator pass the first argument(for standard args this is the .exe path)
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("Didnt get a query string")
        };

        let file_path = match args.next(){
            Some(arg) => arg,
            None => return Err("Didnt get a File Path")
        };

        let mut ignore_case = false;

        let mut is_regex = false;

        if let Some(value) = args.next(){
            if value.contains(&String::from("--is-regex")) {
                is_regex= true;
            }else if value.contains(&String::from("--ignore-case")){
                ignore_case = true;
            }
        }

        Ok(Config { query, file_path, ignore_case, is_regex })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results =if config.ignore_case{
        search(
            &contents,
            |lines| {
                lines
                    .to_lowercase()
                    .contains(&config.query.to_lowercase())
            }
        )
    }else if config.is_regex {
        search_regex(&config.query, &contents)?
    }else{
        search(
            &contents,
            |lines| lines.contains(&config.query)
        )
    };
    
    results.iter().for_each(|lines| println!("{lines}"));

    Ok(())
}
fn search<P>(contents: &str, predicate: P) -> Vec<&str>
where
    P: Fn(&str) -> bool
{
    contents
        .lines()
        .filter(|lines| predicate(lines))
        .collect()
}
// keeping this because it's an example of why closures are cool
// this has become redundant by letting the user specify what it wants
// to filter by
#[allow(dead_code)]
fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    contents
        .lines()
        .filter(|lines| lines.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
fn search_regex<'a>(query: &str, contents: &'a str) -> Result<Vec<&'a str>, regex::Error>{
    // This throws the regex::Error if it is invalid regex
    let regex = Regex::new(query)?;
    Ok(contents
        .lines()
        .filter(|lines| regex.is_match(lines))
        .collect()
    )
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

        assert_eq!(vec!["safe, fast, productive."], search(contents, |lines| lines.contains(query)));
    }
    #[test]
    fn regex(){
        let query = "[A-Z].*";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["Rust:", "Pick three."] , search_regex(query, contents).unwrap());
    }
    #[test]
    #[should_panic]
    fn regex_invalid(){

        let query = "[A-Z]{,1}";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        match search_regex(query, contents){
            Ok(results) => { eprint!("Valid Regex String: "); assert_eq!(vec!["Rust:", "Pick three."] , results) },
            Err(err) => { eprint!("Invalid Regex {err}"); panic!() }
        };
    }
    #[test]
    fn case_insensitive(){
        let query = "P";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
            search(contents, |lines| lines.to_lowercase().contains(&query.to_lowercase()))
        );
    }

}