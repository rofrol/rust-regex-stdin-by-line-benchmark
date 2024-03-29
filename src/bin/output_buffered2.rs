use regex::Regex;
use std::io::prelude::*;
use std::io::{self, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regex = r#"^([^ ]+) ([^ ]+) ([^\[]+) +\[([^\]]*)\] "([A-Z]+) ([^"]*)" ([0-9]+) ([0-9]+) "([^"]*)" "([^"]*)"$"#;
    let regex = Regex::new(regex)?;
    let mut writer = BufWriter::new(io::stdout());

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if let Some(_) = regex.captures(&line) {
            writer.write_all(line.as_bytes())?;
        }
    }
    Ok(())
}
