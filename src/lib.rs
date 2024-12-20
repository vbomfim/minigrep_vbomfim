//! # Minigrep
//! `minigrep` is a simple command-line tool written in Rust that searches for a pattern in a file and prints the lines that contain it.
//! ## Usage
//! `minigrep <pattern> <file>`
//! ## Example
//! `minigrep "pattern" file.txt`
//! ## Features
//! - Case-sensitive search
//! - Case-insensitive search
//! - Search in a file
//!
use std::error::Error;
use std::fs;

pub mod config;

pub fn run(configuration: &config::Config) -> Result<(), Box<dyn Error>> {
    // Read content from file
    let contents: String = fs::read_to_string(&configuration.file_path())?;

    // Filter lines
    let results = if configuration.ignore_case() {
        search_case_insensitive(&configuration.query(), &contents)
    } else {
        search(&configuration.query(), &contents)
    };

    // Print filtered lines
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
