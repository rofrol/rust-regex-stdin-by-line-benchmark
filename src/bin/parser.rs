use memchr::memchr;
use std::fmt;
use std::io::{BufRead, BufReader, Error};
use std::num::ParseIntError;

#[derive(Debug)]
enum ParseError {
    IoError(Error),
    ParseIntError(ParseIntError),
    UnexpectedEndOfLine,
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IoError(err) => fmt::Display::fmt(err, f),
            ParseError::ParseIntError(err) => fmt::Display::fmt(err, f),
            ParseError::UnexpectedEndOfLine => write!(f, "Unexpected end of line."),
        }
    }
}
impl From<Error> for ParseError {
    fn from(err: Error) -> ParseError {
        ParseError::IoError(err)
    }
}
impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::ParseIntError(err)
    }
}

struct Parser<R: BufRead> {
    input: R,
    line_buf: String,
}

#[derive(Debug, PartialEq, Eq)]
struct LogEntry<'a> {
    first: &'a str,
    second: &'a str,
    third: &'a str,
    brackets: &'a str,
    quotes_1: &'a str,
    int1: u32,
    int2: u32,
    quotes_2: &'a str,
    quotes_3: &'a str,
}

#[inline]
fn next_char(buf: &[u8], b: u8, i: &mut usize) -> Result<usize, ParseError> {
    match memchr(b, &buf[*i..]) {
        Some(n) => {
            *i += n + 1;
            Ok(*i)
        }
        None => Err(ParseError::UnexpectedEndOfLine),
    }
}

impl<R: BufRead> Parser<R> {
    pub fn new(input: R) -> Self {
        Self {
            input,
            line_buf: String::new(),
        }
    }
    pub fn next_entry(&mut self) -> Result<Option<LogEntry>, ParseError> {
        self.line_buf.clear();
        if self.input.read_line(&mut self.line_buf)? == 0 {
            return Ok(None);
        }

        let mut i = 0;
        let first_space = next_char(self.line_buf.as_bytes(), b' ', &mut i)?;
        let second_space = next_char(self.line_buf.as_bytes(), b' ', &mut i)?;
        let third_space = next_char(self.line_buf.as_bytes(), b' ', &mut i)?;
        let start_paren = next_char(self.line_buf.as_bytes(), b'[', &mut i)?;
        let end_paren = next_char(self.line_buf.as_bytes(), b']', &mut i)?;
        let quote_1_s = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;
        let quote_1_e = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;
        i += 1;
        let i1_end = next_char(self.line_buf.as_bytes(), b' ', &mut i)?;
        let i2_end = next_char(self.line_buf.as_bytes(), b' ', &mut i)?;
        let quote_2_s = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;
        let quote_2_e = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;
        let quote_3_s = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;
        let quote_3_e = next_char(self.line_buf.as_bytes(), b'"', &mut i)?;

        Ok(Some(LogEntry {
            first: &self.line_buf[..first_space - 1],
            second: &self.line_buf[first_space..second_space - 1],
            third: &self.line_buf[second_space..third_space - 1],
            brackets: &self.line_buf[start_paren..end_paren - 1],
            quotes_1: &self.line_buf[quote_1_s..quote_1_e - 1],
            int1: self.line_buf[quote_1_e + 1..i1_end - 1].parse()?,
            int2: self.line_buf[i1_end..i2_end - 1].parse()?,
            quotes_2: &self.line_buf[quote_2_s..quote_2_e - 1],
            quotes_3: &self.line_buf[quote_3_s..quote_3_e - 1],
        }))
    }
}

fn main() -> Result<(), ParseError> {
    let stdin = std::io::stdin();
    let mut parser = Parser::new(BufReader::new(stdin.lock()));

    let expected = LogEntry {
        first: "54.213.178.139",
        second: "-",
        third: "-",
        brackets: "24/Sep/2019:03:28:55 +0200",
        quotes_1: "GET / HTTP/1.1",
        int1: 301,
        int2: 415,
        quotes_2: "-",
        quotes_3: "Go-http-client/1.1",
    };

    while let Some(entry) = parser.next_entry()? {
        assert_eq!(expected, entry);
        println!("{}", parser.line_buf);
    }

    Ok(())
}
