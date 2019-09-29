use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, BufWriter};
use std::os::unix::io::{FromRawFd, IntoRawFd};

fn main() -> io::Result<()> {
    let regex = r#"^([^ ]+) ([^ ]+) ([^\[]+) +\[([^\]]*)\] "([A-Z]+) ([^"]*)" ([0-9]+) ([0-9]+) "([^"]*)" "([^"]*)"$"#;
    let regex = Regex::new(regex).unwrap();

    let stdin = unsafe { File::from_raw_fd(0) };
    let stdout = unsafe { File::from_raw_fd(1) };
    let (mut reader, mut writer) = (BufReader::new(stdin), BufWriter::new(stdout));

    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Err(_) | Ok(0) => break,
            Ok(_) => {
                if let Some(_) = regex.captures(&line) {
                    writer.write_all(line.as_bytes()).unwrap();
                }
                line.clear();
            }
        }
    }

    // drop without closing the fds
    let _ = reader.into_inner().into_raw_fd();
    let _ = writer.into_inner().unwrap().into_raw_fd();
    Ok(())
}
