use std::fmt;
use std::io::{BufRead, BufReader, Error};
use std::num::ParseIntError;

#[derive(Debug)]
enum ParseError {
    IoError(Error),
    ParseIntError(ParseIntError),
    UnmatchedBracket(char, char),
    UnexpectedEndOfLine,
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::IoError(err) => fmt::Display::fmt(err, f),
            ParseError::ParseIntError(err) => fmt::Display::fmt(err, f),
            ParseError::UnmatchedBracket(found, expected) => {
                write!(f, "Expected {} found {}.", expected, found)
            }
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
        // read_line puts a newline at the end
        self.line_buf.pop();
        self.line_buf.push(' ');

        struct Searcher<'a> {
            mem: memchr::Memchr<'a>,
            buf: &'a [u8],
            /// This is always either zero or the character after a space.
            /// This means that slices starting at i are always valid utf-8 if buf is.
            i: usize,
        }
        impl<'a> Searcher<'a> {
            #[inline]
            pub fn next_word(&mut self) -> Result<&'a [u8], ParseError> {
                loop {
                    let ni = self.mem.next().ok_or(ParseError::UnexpectedEndOfLine)?;
                    let i = self.i;
                    self.i = ni + 1;
                    if ni - i > 1 {
                        // This is valid utf-8, since ni is the index of a space.
                        return Ok(&self.buf[i..ni]);
                    }
                }
            }
            #[inline]
            pub fn next_delimitered(&mut self, start: u8, end: u8) -> Result<&'a [u8], ParseError> {
                loop {
                    let ni = self.mem.next().ok_or(ParseError::UnexpectedEndOfLine)?;
                    if self.buf[ni - 1] == end {
                        let mut i = self.i;

                        while self.buf[i] == b' ' {
                            i += 1;
                        }
                        if self.buf[i] != start {
                            return Err(ParseError::UnmatchedBracket(
                                self.buf[i] as char,
                                start as char,
                            ));
                        }

                        self.i = ni + 1;
                        // Since start and end are valid ascii characters,
                        // this is valid utf-8.
                        return Ok(&self.buf[i + 1..ni - 1]);
                    }
                }
            }
        }

        let mut searcher = Searcher {
            mem: memchr::Memchr::new(b' ', self.line_buf.as_bytes()),
            buf: self.line_buf.as_bytes(),
            i: 0,
        };

        let first = searcher.next_word()?;
        let second = searcher.next_word()?;
        let third = searcher.next_word()?;
        let brackets = searcher.next_delimitered(b'[', b']')?;
        let quotes_1 = searcher.next_delimitered(b'"', b'"')?;
        let int1 = searcher.next_word()?;
        let int2 = searcher.next_word()?;
        let quotes_2 = searcher.next_delimitered(b'"', b'"')?;
        let quotes_3 = searcher.next_delimitered(b'"', b'"')?;

        /*
        Ok(Some(LogEntry {
            first: std::str::from_utf8(first)?,
            second: std::str::from_utf8(second)?,
            third: std::str::from_utf8(third)?,
            brackets: std::str::from_utf8(brackets)?,
            quotes_1: std::str::from_utf8(quotes_1)?,
            int1: std::str::from_utf8(int1)?.parse()?,
            int2: std::str::from_utf8(int2)?.parse()?,
            quotes_2: std::str::from_utf8(quotes_2)?,
            quotes_3: std::str::from_utf8(quotes_3)?,
        }))
         */

        // Turns out the utf-8 check is pretty expensive.
        // I added comments above that explains why the slices are always valid utf-8.

        unsafe {
            Ok(Some(LogEntry {
                first: std::str::from_utf8_unchecked(first),
                second: std::str::from_utf8_unchecked(second),
                third: std::str::from_utf8_unchecked(third),
                brackets: std::str::from_utf8_unchecked(brackets),
                quotes_1: std::str::from_utf8_unchecked(quotes_1),
                int1: std::str::from_utf8_unchecked(int1).parse()?,
                int2: std::str::from_utf8_unchecked(int2).parse()?,
                quotes_2: std::str::from_utf8_unchecked(quotes_2),
                quotes_3: std::str::from_utf8_unchecked(quotes_3),
            }))
        }
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
        assert_eq!(entry.first, expected.first);
        assert_eq!(entry.second, expected.second);
        assert_eq!(entry.third, expected.third);
        assert_eq!(entry.brackets, expected.brackets);
        assert_eq!(entry.quotes_1, expected.quotes_1);
        assert_eq!(entry.int1, expected.int1);
        assert_eq!(entry.int2, expected.int2);
        assert_eq!(entry.quotes_2, expected.quotes_2);
        assert_eq!(entry.quotes_3, expected.quotes_3);
    }

    Ok(())
}
