use anyhow::anyhow;
use std::fmt::Debug;

use nom::Parser;

pub trait ParseAnyhow<I> {
    type Output;
    fn parse_anyhow(&mut self, input: I) -> anyhow::Result<Self::Output>;
}

impl<'a, 'b, T, I> ParseAnyhow<I> for T
where
    I: 'a,
    T::Output: 'b,
    T: Parser<I>,
    T::Error: Debug,
{
    type Output = T::Output;

    fn parse_anyhow(&mut self, input: I) -> anyhow::Result<Self::Output> {
        self.parse(input)
            .map_err(|err| anyhow!("{}", err))
            .map(|(_, res)| res)
    }
}
