use minigrep::config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new()?;
    minigrep::run(&config)?;
    Ok(())
}
