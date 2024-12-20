use minigrep_vbomfim::config;
use std::error::Error;
/// Documentation for the minigrep application.
///
/// This application searches for a pattern in a file and prints the lines that contain it.
fn main() -> Result<(), Box<dyn Error>> {
    let config = config::Config::new()?;
    minigrep_vbomfim::run(&config)?;
    Ok(())
}
