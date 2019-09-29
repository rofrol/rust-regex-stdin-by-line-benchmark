use regex::Regex;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let regex = r#"^([^ ]+) ([^ ]+) ([^\[]+) +\[([^\]]*)\] "([A-Z]+) ([^"]*)" ([0-9]+) ([0-9]+) "([^"]*)" "([^"]*)"$"#;
    let regex = Regex::new(regex)?;

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if let Some(_) = regex.captures(&line) {
            println!("{}", line);
        }
    }
    Ok(())
}
