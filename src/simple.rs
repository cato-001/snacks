use nom::error::ErrorKind;
use nom::{AsChar, IResult, Input};

pub fn alphanumdot0(input: &str) -> IResult<&str, &str> {
    input.split_at_position_complete(|char| !char.is_alphanum() || char != '.')
}

pub fn alphanumdot1(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(
        |char| !char.is_alphanum() || char != '.',
        ErrorKind::AlphaNumeric,
    )
}
