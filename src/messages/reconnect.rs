use std::borrow::Cow;

use super::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Reconnect<'a> {
    pub raw: Cow<'a, str>,
}

impl<'a> TryFrom<Message<'a>> for Reconnect<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self { raw: value.raw })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Reconnect<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn reconnect() {
        let input = ":tmi.twitch.tv RECONNECT\r\n";
        let raw = test_util::raw(input);
        assert_eq!(test_util::parse_as::<Reconnect>(input), Reconnect { raw });
    }
}
