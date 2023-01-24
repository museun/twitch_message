use super::message::Message;
use crate::{typed_messages::TypedMessageMarker, Error};

/// A parse trait for parsing a `&mut &str` into some type.
pub trait Parse<'a>: Sized {
    /// Output of the parse method
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

/// # Representation of a possibly partially parse
///
/// If [`parse()`] is called with a string that may contain multiple messages, this type will point to the start of the next messages.
///
/// ```rust
/// # use twitch_message::{parse, ParseResult};
/// let mut raw_line = ":tmi.twitch.tv PING :12345\r\n:tmi.twitch.tv PING :12346\r\n:tmi.twitch.tv PING :12347\r\n";
///
/// let res = parse(raw_line)?;
/// assert_eq!(res.remaining, ":tmi.twitch.tv PING :12346\r\n:tmi.twitch.tv PING :12347\r\n");
/// raw_line = res.remaining;
///
/// let res = parse(raw_line)?;
/// assert_eq!(res.remaining, ":tmi.twitch.tv PING :12347\r\n");
/// raw_line = res.remaining;
///
/// let res = parse(raw_line)?;
/// assert_eq!(res.remaining, "");
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
///
/// # NOTE
/// If [`parse()`] did not contain a `\r\n` then [`parse()`] considers it as a *full* message, and [`ParseResult::remaining`] will be empty
#[derive(Debug, Clone)]
pub struct ParseResult<'a> {
    /// The remaining data, if any
    pub remaining: &'a str,
    /// The parsed messages
    pub message: Message<'a>,
}

/// Attempt to parse a message.
///
/// For the behavior of the value see [`ParseResult`]
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

/// Parses potentionally many messages from the input.
///
/// This returns an iterator of [`Message`]
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

/// A helper parse function for parsing a string as a specific typed message
///
/// For available messages, see the structs in [`messages`](crate::messages#structs)
pub fn parse_as<'a, T>(input: &'a str) -> Result<T, Error>
where
    T: TypedMessageMarker<'a>,
{
    let msg = crate::parse(input)?.message;
    msg.as_typed_message::<T>()
        .ok_or_else(|| Error::IncorrectMessageType {
            expected: T::kind(),
            got: msg.kind.as_str(),
        })
}
