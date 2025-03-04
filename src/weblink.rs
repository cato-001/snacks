use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::recognize;
use nom::error::ErrorKind;
use nom::{AsChar, IResult, Input, Parser};

use crate::recognize_separated::recognize_separated0;

/// Takes a weblink from the input.
///
/// ```rust
/// use snacks::weblink;
///
/// let input = "https://github.com/cato-001/snacks.git other";
/// let result = weblink(input);
/// assert_eq!(
///     Ok((" other", "https://github.com/cato-001/snacks.git")),
///     result
/// );
/// ```
pub fn weblink(input: &str) -> IResult<&str, &str> {
    recognize((
        alt((tag("https://"), tag("http://"))),
        recognize_separated0(link_char, char('/')),
    ))
    .parse(input)
}

pub fn link_char(input: &str) -> IResult<&str, &str> {
    input.split_at_position1_complete(
        |char| !(char.is_alphanum() || matches!(char, '-' | '_' | '.')),
        ErrorKind::AlphaNumeric,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_parse_empty() {
        let input = "";
        let result = weblink(input);
        assert!(result.is_err());
    }

    #[test]
    fn can_parse_github_link() {
        let input = "https://github.com/cato-001/snacks.git other";
        let result = weblink(input);
        assert_eq!(
            Ok((" other", "https://github.com/cato-001/snacks.git")),
            result
        );
    }
}
