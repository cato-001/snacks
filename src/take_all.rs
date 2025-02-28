use nom::error::ParseError;
use nom::{IResult, Parser};

pub fn take_all<'a, Input: Copy, Output, Error: ParseError<Input>>(
    prefix: impl Parser<Input, Error = Error>,
    item: impl Parser<Input, Output = Output, Error = Error>,
) -> impl FnMut(Input) -> IResult<Input, Vec<Output>> {
    let mut parser = take_all_into(prefix, item);
    move |input| {
        let mut buffer = Vec::new();
        let (input, _) = parser(input, &mut buffer)?;
        Ok((input, buffer))
    }
}

pub fn take_all_into<'a, Input: Copy, Output, Error: ParseError<Input>>(
    prefix: impl Parser<Input, Error = Error>,
    item: impl Parser<Input, Output = Output, Error = Error>,
) -> impl FnMut(Input, &mut Vec<Output>) -> IResult<Input, ()> {
    let mut prefix = prefix;
    let mut item = item;
    move |mut start, buffer| loop {
        let input = prefix.parse(start).map(|(input, _)| input).unwrap_or(start);
        let Ok((input, value)) = item.parse(input) else {
            return Ok((start, ()));
        };
        buffer.push(value);
        start = input;
    }
}

#[cfg(test)]
mod tests {
    use nom::bytes::complete::is_not;
    use nom::bytes::is_a;
    use nom::character::complete::one_of;
    use nom::sequence::preceded;

    use super::*;

    #[test]
    fn can_parse_empty() {
        let input = "";
        let result = take_all::<_, _, nom::error::Error<_>>(is_a("abc"), is_a("def"))(input);
        assert_eq!(Ok(("", Vec::new())), result);
    }

    #[test]
    fn can_take_one() {
        let input = "abcdefghi";
        let result = take_all::<_, _, nom::error::Error<_>>(is_a("abc"), is_a("def"))(input);
        assert_eq!(Ok(("ghi", vec!["def"])), result);
    }

    #[test]
    fn can_take_tags() {
        let input = "word1 word2 #tag1 #tag2 word3 #tag3 word4";
        let result = take_all::<_, _, nom::error::Error<_>>(
            is_not("#"),
            preceded(one_of("#"), is_not(" ")),
        )(input);
        assert_eq!(Ok((" word4", vec!["tag1", "tag2", "tag3"])), result);
    }

    #[test]
    fn can_take_tags_newline() {
        let input = "word1 word2 #tag1\n#tag2 word3\n#tag3\nword4";
        let result = take_all::<_, _, nom::error::Error<_>>(
            is_not("#"),
            preceded(one_of("#"), is_not(" \n")),
        )(input);
        assert_eq!(Ok(("\nword4", vec!["tag1", "tag2", "tag3"])), result);
    }

    #[test]
    fn can_take_from_start() {
        let input = "#tag0 word1 word2 #tag1\n#tag2 word3\n#tag3\nword4";
        let result = take_all::<_, _, nom::error::Error<_>>(
            is_not("#"),
            preceded(one_of("#"), is_not(" \n")),
        )(input);
        assert_eq!(
            Ok(("\nword4", vec!["tag0", "tag1", "tag2", "tag3"])),
            result
        );
    }

    #[test]
    fn can_run_example() {
        let input = "An example #sentence with #cool tags!";
        let result = take_all::<_, _, nom::error::Error<_>>(
            is_not("#"),
            preceded(one_of("#"), is_not(" ")),
        )(input);
        assert_eq!(Ok((" tags!", vec!["sentence", "cool"])), result);
    }

    #[test]
    fn can_run_example_with_buffer() {
        let mut buffer = Vec::new();
        let input = "An example #sentence with #cool tags!";
        let result = take_all_into::<_, _, nom::error::Error<_>>(
            is_not("#"),
            preceded(one_of("#"), is_not(" ")),
        )(input, &mut buffer);
        assert_eq!(Ok((" tags!", ())), result);
        assert_eq!(vec!["sentence", "cool"], buffer);
    }
}
