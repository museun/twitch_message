use std::borrow::Cow;

use super::Message;

/// This command is a reply to the [PING](crate::encode::Ping) command
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Pong<'a> {
    /// Token associated with the [`Ping`](crate::encode::Ping) that this pong should reflect
    pub token: Cow<'a, str>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
}

impl Pong<'_> {
    const fn validate(value: &Message<'_>) -> bool {
        value.data.is_some()
    }
}

impl<'a> TryFrom<Message<'a>> for Pong<'a> {
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

impl<'a, 'b> TryFrom<&'b Message<'a>> for Pong<'a> {
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
    fn pong() {
        let input = "PONG :1234567890\r\n";
        let raw = test_util::raw(input);

        assert_eq!(
            test_util::parse_as::<Pong>(input),
            Pong {
                token: Cow::from("1234567890"),
                raw,
            }
        );
    }
}
