//! # Minigrep
//! Searches for a line containing a query string.

use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // skip over the first one
        args.next();

        let query = match args.next() {
            None => {
                return Err("Didn't get a query string");
            }
            Some(v) => v,
        };

        let filename = match args.next() {
            None => {
                return Err("Didn't get a filename");
            }
            Some(v) => v,
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    for line in search(&config.query, &contents, config.case_sensitive) {
        println!("{}", line);
    }

    return Ok(());
}

/// Searches for a line in the (query) in the given string (contents). It also accepts
/// whether we want it to be a case sensitive search or not.
///
/// # Examples
///
/// ```
/// let query = "potato";
/// let contents = "A potato flew around\n my room before you came";
///
/// let answer = minigrep::search(query, contents, true);
///
///
/// assert_eq!(vec!["A potato flew around"], answer);
/// ```
///
/// # Panics
/// scenarios in which it panics
///
/// # Errors
/// describing the kinds of errors that might occur and what conditions
///
/// # Safety
/// if it is unsafe to call, there should be a section explaining why the function is unsafe
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    // bind the lifetime of the return vector to the one in the contents str
    // this is needed because we are returning a vec of references
    let lower_query = String::from(query).to_lowercase();

    let query = if case_sensitive { query } else { &lower_query };

    return contents
        .lines()
        .filter(|line| {
            return if case_sensitive {
                line.contains(query)
            } else {
                line.to_lowercase().contains(query)
            };
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents, true)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "dUcT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive.", "Duct tape."],
            search(query, contents, false)
        );
    }
}
