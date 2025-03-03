use nom::error::ParseError;
use nom::{IResult, Input, Parser};

pub fn recognize_separated0<I, Separator, Item, Error>(
    item: Item,
    separator: Separator,
) -> impl FnMut(I) -> IResult<I, I>
where
    I: Input + Copy,
    Separator: Parser<I, Error = Error>,
    Item: Parser<I, Error = Error> + Copy,
    Error: ParseError<I>,
{
    let mut item = item;
    let mut next_item = (separator, item);
    move |start| {
        let Ok((mut input, _)) = item.parse(start) else {
            return Ok(start.take_split(0));
        };
        loop {
            let Ok((remaining, _)) = next_item.parse(input) else {
                return Ok(start.take_split(start.input_len() - input.input_len()));
            };
            input = remaining;
        }
    }
}

#[cfg(test)]
mod tests {
    use nom::bytes::complete::is_a;
    use nom::character::complete::{alphanumeric1, char};

    use super::*;

    #[test]
    fn can_recognize_empty() {
        let input = "";
        let result = recognize_separated0::<_, _, _, nom::error::Error<_>>(
            alphanumeric1,
            is_a(",; "),
        )(input);
        assert_eq!(Ok(("", "")), result);
    }

    #[test]
    fn can_recognize_comma_separated_elements() {
        let input = "all, comma, separated-elements";
        let result = recognize_separated0::<_, _, _, nom::error::Error<_>>(
            alphanumeric1,
            is_a(",; "),
        )(input);
        assert_eq!(Ok(("-elements", "all, comma, separated")), result);
    }

    #[test]
    fn can_run_example() {
        let input = "comma,separated,words other";
        let result =
            recognize_separated0::<_, _, _, nom::error::Error<_>>(alphanumeric1, char(','))(input);
        assert_eq!(Ok((" other", "comma,separated,words")), result);
    }
}
