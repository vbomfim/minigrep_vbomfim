use clap::Parser;
use std::env;

pub const IGNORE_CASE: &str = "IGNORE_CASE";

#[derive(Parser)]
#[command(name = "minigrep")]
#[command(about = "A simple grep clone in Rust", long_about = None)]
pub struct Config {
    /// The pattern to search for
    query: String,
    /// The path to the file to read
    file_path: String,
    /// Ignore case sensitivity
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    ignore_case: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        //        let args_str: Vec<String> = std::env::args().collect();
        //        let args: Vec<&str> = args_str.iter().map(|s| s.as_str()).collect();
        Self::build_from(std::env::args())
    }

    pub fn build_from<I: Iterator<Item = String>>(args: I) -> Result<Config, &'static str> {
        // args: &[&str]
        let mut config = Config::parse_from(args);
        config.ignore_case = Self::determine_ignore_case(config.ignore_case);
        Ok(config)
    }

    fn determine_ignore_case(cli_ignore_case: bool) -> bool {
        cli_ignore_case
            || env::var_os(IGNORE_CASE)
                .map(|v| v.to_str().unwrap_or("").to_lowercase() == "true")
                .unwrap_or(false)
    }

    pub fn query(&self) -> &str {
        &self.query
    }
    pub fn file_path(&self) -> &str {
        &self.file_path
    }
    pub fn ignore_case(&self) -> bool {
        self.ignore_case
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    fn clear_env_var() {
        env::remove_var(IGNORE_CASE);
    }

    #[test]
    #[serial]
    fn test_build_with_ignore_case_flag() {
        clear_env_var();
        let config =
            Config::build_from(&["minigrep", "pattern", "file.txt", "--ignore-case"]).unwrap();
        assert!(config.ignore_case);
    }

    #[test]
    #[serial]
    fn test_build_with_ignore_case_env_var() {
        env::set_var(IGNORE_CASE, "true");
        let config = Config::build_from(&["minigrep", "pattern", "file.txt"]).unwrap();
        assert!(config.ignore_case);
        clear_env_var();
    }

    #[test]
    #[serial]
    fn test_build_without_ignore_case() {
        clear_env_var();
        let config = Config::build_from(&["minigrep", "pattern", "file.txt"]).unwrap();
        assert!(!config.ignore_case);
    }
}
