use crate::{Error, Message};

pub trait Parse<'a>: Sized {
    type Output;
    fn parse(input: &mut &'a str) -> Self::Output;
}

impl<'a, T> Parse<'a> for T
where
    T: std::str::FromStr + Sized,
{
    type Output = Result<T, T::Err>;

    fn parse(input: &mut &'a str) -> Self::Output {
        str::parse(input)
    }
}

#[derive(Debug, Clone)]
pub struct ParseResult<'a> {
    pub remaining: &'a str,
    pub message: Message<'a>,
}

pub fn parse(mut input: &str) -> Result<ParseResult<'_>, Error> {
    if let Some((mut head, tail)) = input.split_once("\r\n") {
        return Message::parse(&mut head).map(|msg| ParseResult {
            remaining: tail,
            message: msg,
        });
    }

    let input = &mut input;
    Message::parse(input).map(|msg| ParseResult {
        remaining: input,
        message: msg,
    })
}

pub fn parse_many(mut input: &str) -> impl Iterator<Item = Result<Message<'_>, Error>> + '_ {
    std::iter::from_fn(move || {
        if matches!(input, "" | "\r\n" | "\n") {
            return None;
        }

        match parse(input) {
            Ok(ParseResult { remaining, message }) => {
                input = remaining;
                Some(Ok(message))
            }
            Err(err) => Some(Err(err)),
        }
    })
}
