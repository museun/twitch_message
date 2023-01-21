use std::borrow::Cow;

use super::{Message, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ClearChat<'a> {
    pub raw: Cow<'a, str>,
    pub channel: Cow<'a, str>,
    pub target: ClearChatTarget<'a>,
    pub tags: Tags<'a>,
}

impl ClearChat<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClearChatTarget<'a> {
    All,
    User(Cow<'a, str>),
}

impl<'a> TryFrom<Message<'a>> for ClearChat<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw,
            channel: value.args.remove(0),
            target: value
                .data
                .map_or(ClearChatTarget::All, ClearChatTarget::User),
            tags: value.tags,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for ClearChat<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw.clone(),
            channel: value.args[0].clone(),
            target: value
                .data
                .clone()
                .map_or(ClearChatTarget::All, ClearChatTarget::User),
            tags: value.tags.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn clear_chat_all() {
        let input =
            "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARCHAT #museun\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<ClearChat>(input),
            ClearChat {
                raw,
                tags,
                channel: Cow::from("#museun"),
                target: ClearChatTarget::All
            }
        );
    }

    #[test]
    fn clear_chat() {
        let input = "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARCHAT #museun :shaken_bot\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<ClearChat>(input),
            ClearChat {
                raw,
                tags,
                channel: Cow::from("#museun"),
                target: ClearChatTarget::User(Cow::from("shaken_bot"))
            }
        );
    }
}
