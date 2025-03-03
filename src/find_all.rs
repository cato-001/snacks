use nom::{error::ParseError, FindSubstring, IResult, Parser};

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
