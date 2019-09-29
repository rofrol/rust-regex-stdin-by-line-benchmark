use regex::Regex;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let regex = r#"^([^ ]+) ([^ ]+) ([^\[]+) +\[([^\]]*)\] "([A-Z]+) ([^"]*)" ([0-9]+) ([0-9]+) "([^"]*)" "([^"]*)"$"#;
    let regex = Regex::new(regex).unwrap();

    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if let Some(_) = regex.captures(&line) {
            println!("{}", line);
        }
    }
    Ok(())
}
