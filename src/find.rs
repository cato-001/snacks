use nom::error::ErrorKind;
use nom::{error::ParseError, FindSubstring, IResult, Parser};

/// Run the parser at each found substring,
/// until the parser completes successfully, then return the result.
///
/// ```rust
/// use snacks::find_first;
/// use nom::character::complete::{alphanumeric1, char};
/// use nom::sequence::delimited;
///
/// let input = "This is a {text} with some {special} {words}!";
/// let result = find_first::<_, _, _, nom::error::Error<_>>(
///     "{",
///     delimited(char('{'), alphanumeric1, char('}')),
/// )(input);
/// assert_eq!(Ok((" with some {special} {words}!", "text")), result);
/// ```
pub fn find_first<I, N, P, Error>(needle: N, item: P) -> impl FnMut(I) -> IResult<I, P::Output>
where
    I: FindSubstring<N> + nom::Input + Copy,
    N: nom::Input + Copy,
    P: Parser<I, Error = Error>,
    Error: ParseError<I>,
{
    let mut item = item;
    move |mut start| loop {
        let Some(index) = start.find_substring(needle) else {
            return Err(nom::Err::Error(nom::error::Error::new(
                start,
                ErrorKind::Fail,
            )));
        };
        let input = start.take_from(index);
        let Ok((input, value)) = item.parse(input) else {
            start = input.take_from(needle.input_len());
            continue;
        };
        return Ok((input, value));
    }
}

/// Run the parser at each found substring.
///
/// ```rust
/// use snacks::find_all;
/// use nom::character::complete::{alphanumeric1, char};
/// use nom::sequence::delimited;
///
/// let input = "This is a {text} with some {special} {words}!";
/// let result = find_all::<_, _, _, nom::error::Error<_>>(
///     "{",
///     delimited(char('{'), alphanumeric1, char('}')),
/// )(input);
/// assert_eq!(Ok(("!", vec!["text", "special", "words"])), result);
/// ```
pub fn find_all<I, N, P, Error>(needle: N, item: P) -> impl FnMut(I) -> IResult<I, Vec<P::Output>>
where
    I: FindSubstring<N> + nom::Input + Copy,
    N: nom::Input + Copy,
    P: Parser<I, Error = Error>,
    Error: ParseError<I>,
{
    let mut parser = find_all_into(needle, item);
    move |input| {
        let mut buffer = Vec::new();
        let (input, _) = parser(input, &mut buffer)?;
        Ok((input, buffer))
    }
}

/// Run the parser at each found substring,
/// and push the results into the provided vector.
///
/// ```rust
/// use snacks::find_all_into;
/// use nom::character::complete::{alphanumeric1, char};
/// use nom::sequence::delimited;
///
/// let input = "This is a {text} with some {special} {words}!";
///
/// let mut buffer = Vec::new();
/// let result = find_all_into::<_, _, _, nom::error::Error<_>>(
///     "{",
///     delimited(char('{'), alphanumeric1, char('}')),
/// )(input, &mut buffer);
///
/// assert_eq!(vec!["text", "special", "words"], buffer);
/// ```
pub fn find_all_into<I, N, P, Error>(
    needle: N,
    item: P,
) -> impl FnMut(I, &mut Vec<P::Output>) -> IResult<I, ()>
where
    I: FindSubstring<N> + nom::Input + Copy,
    N: nom::Input + Copy,
    P: Parser<I, Error = Error>,
    Error: ParseError<I>,
{
    let mut item = item;
    move |mut start, buffer| loop {
        let Some(index) = start.find_substring(needle) else {
            return Ok((start, ()));
        };
        let input = start.take_from(index);
        let Ok((input, value)) = item.parse(input) else {
            start = input.take_from(needle.input_len());
            continue;
        };
        buffer.push(value);
        start = input;
    }
}

#[cfg(test)]
mod tests {
    use nom::character::complete::{alphanumeric1, char};
    use nom::sequence::delimited;

    use super::*;

    #[test]
    fn can_find_in_empty() {
        let input = "";
        let result = find_all::<_, _, _, nom::error::Error<_>>(
            "http",
            delimited(char('{'), alphanumeric1, char('}')),
        )(input);
        assert_eq!(Ok((input, Vec::new())), result);
    }

    #[test]
    fn can_collect_value() {
        let input = "This is a {text} with some {special} {words}!";
        let result = find_all::<_, _, _, nom::error::Error<_>>(
            "{",
            delimited(char('{'), alphanumeric1, char('}')),
        )(input);
        assert_eq!(Ok(("!", vec!["text", "special", "words"])), result);
    }
}
