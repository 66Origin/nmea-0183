use nom::Err;

#[derive(Debug, PartialEq)]
pub enum Error<'a> {
    ParseError(Err<(&'a str, nom::error::ErrorKind)>),
}

impl<'a> std::convert::From<nom::Err<(&'a str, nom::error::ErrorKind)>> for Error<'a> {
    fn from(err: nom::Err<(&'a str, nom::error::ErrorKind)>) -> Error<'a> {
        Error::ParseError(err)
    }
}
