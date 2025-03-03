use nom::error::ErrorKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error<I> {
    /// position of the error in the input data
    pub input: I,
    /// nom error code
    pub code: ErrorKind,
}

impl nom::error::ParseError<&str> for Error {
    fn from_error_kind(input: &str, kind: nom::error::ErrorKind) -> Self {
        todo!()
    }

    fn append(input: &str, kind: nom::error::ErrorKind, other: Self) -> Self {
        todo!()
    }
}
