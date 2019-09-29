use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::os::unix::io::FromRawFd;

fn main() -> io::Result<()> {
    let regex = r#"^([^ ]+) ([^ ]+) ([^\[]+) +\[([^\]]*)\] "([A-Z]+) ([^"]*)" ([0-9]+) ([0-9]+) "([^"]*)" "([^"]*)"$"#;
    let regex = Regex::new(regex).unwrap();
    let stdout = unsafe { File::from_raw_fd(1) };
    let mut writer = BufWriter::new(stdout);

    for line in std::io::stdin().lock().lines() {
        let line = line?;
        if let Some(_) = regex.captures(&line) {
            writer.write_all(line.as_bytes())?;
        }
    }
    Ok(())
}
