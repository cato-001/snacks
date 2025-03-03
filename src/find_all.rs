use nom::{error::ParseError, FindSubstring, IResult, Parser};

pub fn find_all<'a, Needle, I, Output, Error>(
    needle: Needle,
    item: impl Parser<I, Output = Output, Error = Error> + 'a,
) -> impl FnMut(I) -> IResult<I, Vec<Output>> + 'a
where
    Needle: Copy + 'a,
    I: FindSubstring<Needle> + nom::Input + Copy + 'a,
    Error: ParseError<I>,
{
    let mut parser = find_all_into(needle, item);
    move |input| {
        let mut buffer = Vec::new();
        let (input, _) = parser(input, &mut buffer)?;
        Ok((input, buffer))
    }
}

pub fn find_all_into<'a, Needle, Input, Output, Error>(
    needle: Needle,
    item: impl Parser<Input, Output = Output, Error = Error> + 'a,
) -> impl FnMut(Input, &mut Vec<Output>) -> IResult<Input, ()> + 'a
where
    Needle: Copy + 'a,
    Input: FindSubstring<Needle> + nom::Input + Copy + 'a,
    Error: ParseError<Input>,
{
    let mut item = item;
    move |mut start, buffer| loop {
        let Some(index) = start.find_substring(needle) else {
            return Ok((start, ()));
        };
        let input = start.take_from(index);
        let Ok((input, value)) = item.parse(input) else {
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
