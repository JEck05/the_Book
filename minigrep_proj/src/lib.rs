//! # minigrep_lib
//! `minigrep` is a grep clone that takes a query and a text based file
//! and searches for the query in the file; with added support for regex.
//!

use std::error::Error;
use std::fs;
use regex::Regex;
/// A complex data type, used to run a grep like search.  
/// 
/// This struct is used for [`minigrep::run()`](run()) see its documentation for more.
///
pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub is_regex: bool,
}
impl Config{
    /// Builds the Config Struct from an [iterable](Iterator) `collection` 
    ///
    /// Discards the first element of the `collection` (for env::args this is usually the exe)
    ///
    /// # Errors
    /// If the given [Iterator] cannot produce a `query` and `file_path`, or has invalid command flag, 
    /// then an Error is returned.
    ///
    /// # Examples
    /// 
    /// Basic Example using Command Line Arguments 
    /// ```rust
    /// use std::{env, process};
    /// use minigrep::Config;
    ///
    /// let vec1: Vec<String> = vec![
    ///                 "./minigrep.exe".to_string(),
    ///                 "P".to_string(),
    ///                 "homework.txt".to_string(),
    ///                 "--ignore-case".to_string()
    /// ];
    ///
    /// let config = Config::build(vec1.into_iter()).unwrap_or_else(|err| { 
    ///     eprintln!("Problem parsing Arguments: {err}");
    ///     process::exit(1)}
    /// );
    /// ```
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
            }else {
                return Err("Invalid Flag argument(See documentation for valid flags)");
            }
        }

        Ok(Config { query, file_path, ignore_case, is_regex })
    }
}
/// Searches the given file and prints the lines that match the query.
/// 
/// 
/// # Errors
/// This returns an error if the given file path does not exist.
/// 
/// If searching with regex and the query is an invalid regex, then 
/// an error will also be returned.
/// 
/// # Example
/// Basic Example with Command Line arguments 
/// ```rust
/// use std::{env, process};
/// use minigrep::Config;
/// 
/// let vec1: Vec<String> = vec![
///                 "./minigrep.exe".to_string(),
///                 "P".to_string(),
///                 "homework.txt".to_string(),
///                 "--ignore-case".to_string()
/// ];
/// 
/// let config = Config::build(vec1.into_iter()).unwrap();
/// 
/// // Note if homework.txt isn't a valid file this will throw an error 
/// if let Err(e) = minigrep::run(config) { 
///     eprintln!("Application Error: {e}"); 
/// }
/// ```
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
        let regex = Regex::new(&config.query)?;
        search(
            &contents, 
            |lines| regex.is_match(lines)
        )
    }else{
        search(
            &contents,
            |lines| lines.contains(&config.query)
        )
    };
    
    results.iter().for_each(|lines| println!("{lines}"));

    Ok(())
}
/// Helper function for [`run()`]
/// 
/// Searches a &str line by line and filters based
/// on the given closure
/// 
/// # Examples 
/// ```rust 
///     let query = "P";
///     let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.";
///     assert_eq!(
///         vec!["safe, fast, productive.", "Pick three."],
///         search(contents, |lines| lines.to_lowercase().contains(&query.to_lowercase()))
///     );
/// ```
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
// Keeping this for the same reason.
// Comparing this to the new way we implemented it, we can
// see how closures can be useful and prevent duplicate code
// While it should be noted that this function could still serve a 
// purpose if we wanted to individually deal with the regex::Error
#[allow(dead_code)]
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
        assert_eq!(vec!["Rust:", "Pick three."] , search(
            &contents, 
            |lines| Regex::new(query).unwrap().is_match(lines))
        );
    }
    
    #[test]
    #[should_panic]
    fn regex_invalid(){
        
        let query = "[A-Z]{,3}";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        search(&contents, |lines| Regex::new(query).unwrap().is_match(lines));
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