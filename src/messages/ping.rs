use std::borrow::Cow;

use super::Message;

/// Tests the presence of a connection. A [PING](Self) message results in a [PONG](crate::encode::Pong) reply.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Ping<'a> {
    /// Token associated with the ping that the resulting [`Pong`](crate::encode::Pong) should reflect
    pub token: Cow<'a, str>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
}

impl Ping<'_> {
    const fn validate(value: &Message<'_>) -> bool {
        value.data.is_some()
    }
}

impl<'a> TryFrom<Message<'a>> for Ping<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            token: value.data.unwrap(),
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Ping<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            token: value.data.clone().unwrap(),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn ping() {
        let input = "PING :1234567890\r\n";

        let raw = test_util::raw(input);
        assert_eq!(
            test_util::parse_as::<Ping>(input),
            Ping {
                token: Cow::from("1234567890"),
                raw,
            }
        );
    }
}
