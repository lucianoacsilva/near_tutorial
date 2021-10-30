use super::method::HTTP_method;
use std::convert::TryFrom;

pub struct Request {
    path: String,
    query_string: Option<String>,
    method: HTTP_method
}

impl Request {
    fn from_byte_array(buffer: &[u8]) -> Result<Self, String> {
        unimplemented!();
    }
}

impl TryFrom<&[u8]> for Request {

    type Error = String;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
