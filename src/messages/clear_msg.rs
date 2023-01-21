use std::borrow::Cow;

use super::{Message, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ClearMsg<'a> {
    pub raw: Cow<'a, str>,
    pub channel: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub tags: Tags<'a>,
}

impl ClearMsg<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty() && value.data.is_some()
    }
}

impl<'a> TryFrom<Message<'a>> for ClearMsg<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw,
            channel: value.args.remove(0),
            message: value.data.unwrap(),
            tags: value.tags,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for ClearMsg<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw.clone(),
            channel: value.args[0].clone(),
            message: value.data.clone().unwrap(),
            tags: value.tags.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn clear_msg() {
        let input =
            "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARMSG #museun :Kappa\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<ClearMsg>(input),
            ClearMsg {
                raw,
                tags,
                channel: Cow::from("#museun"),
                message: Cow::from("Kappa")
            }
        );
    }
}
