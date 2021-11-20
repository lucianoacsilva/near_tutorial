use std::str::Utf8Error;
use super::method::HTTP_method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HTTP_method,
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    // GET /search?name=abc&sort=1 HTTP/1.1

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        let request = str::from_utf8(buffer)?;

        unimplemented!();
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    let mut iter = request.chars();


    for (i, c) in request.chars().enumerate() {
        if c == ' ' {
            return Some((&request[..i], &request[i + 1..]));
        }    
    }

    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request!",
            Self::InvalidEncoding => "Invalid Encoding!",
            Self::InvalidProtocol => "Invalid Protocol!",
            Self::InvalidMethod => "Invalid Method!"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}
