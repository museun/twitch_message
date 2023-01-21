use std::borrow::Cow;

use super::{Message, Prefix, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct PrivMsg<'a> {
    pub channel: Cow<'a, str>,
    pub sender: Cow<'a, str>,
    pub tags: Tags<'a>,
    pub data: Cow<'a, str>,
    pub raw: Cow<'a, str>,
}

impl PrivMsg<'_> {
    fn validate(value: &Message<'_>) -> bool {
        matches!(value.prefix, Prefix::User { .. })
            && value.data.is_some()
            && !value.args.is_empty()
    }
}

impl<'a> TryFrom<Message<'a>> for PrivMsg<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            channel: value.args.remove(0),
            sender: match value.prefix {
                Prefix::User { name, .. } => name,
                _ => unreachable!(),
            },
            tags: value.tags,
            data: value.data.unwrap(),
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for PrivMsg<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            channel: value.args[0].clone(),
            sender: match value.prefix.clone() {
                Prefix::User { name, .. } => name,
                _ => unreachable!(),
            },
            tags: value.tags.clone(),
            data: value.data.clone().unwrap(),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn privmsg() {
        let input = "@badge-info=;\
        badges=broadcaster/1,premium/1;\
        color=#008000;\
        display-name=museun;\
        emotes=;\
        first-msg=0;\
        flags=;\
        id=43113387-1686-42d9-9067-3b94eabf6eed;\
        mod=0;\
        returning-chatter=0;\
        room-id=23196011;\
        subscriber=0;\
        tmi-sent-ts=1674286550222;\
        turbo=0;\
        user-id=23196011;\
        user-type= :museun!museun@museun.tmi.twitch.tv PRIVMSG #museun :testing\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<PrivMsg>(input),
            PrivMsg {
                raw,
                tags,
                channel: Cow::from("#museun"),
                sender: Cow::from("museun"),
                data: Cow::from("testing")
            }
        );
    }
}
