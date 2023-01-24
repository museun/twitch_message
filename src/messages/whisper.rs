use std::borrow::Cow;

use crate::{Badge, Color, Emote};

use super::{Message, Prefix, Tags, UserType};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Whisper<'a> {
    pub raw: Cow<'a, str>,
    pub from_user: Cow<'a, str>,
    pub to_user: Cow<'a, str>,
    pub data: Cow<'a, str>,
    pub tags: Tags<'a>,
}

impl<'a> Whisper<'a> {
    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    pub fn emotes<'t: 'a>(&'t self) -> impl Iterator<Item = Emote<'a>> + 't {
        Emote::from_tags(&self.tags, &self.data)
    }

    pub fn msg_id(&self) -> Option<&str> {
        self.tags.get("message-id")
    }

    pub fn thread_id(&self) -> Option<&str> {
        self.tags.get("thread-id")
    }

    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    pub fn is_turbo(&self) -> bool {
        self.tags.bool("turbo")
    }

    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }
}

impl Whisper<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
            && value.data.is_some()
            && matches!(value.prefix, Prefix::User { .. })
    }
}

impl<'a> TryFrom<Message<'a>> for Whisper<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw,
            from_user: value.args.remove(0),
            to_user: match value.prefix {
                Prefix::User { name, .. } => name,
                _ => unreachable!(),
            },
            data: value.data.unwrap(),
            tags: value.tags,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Whisper<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw.clone(),
            from_user: value.args[0].clone(),
            to_user: match value.prefix.clone() {
                Prefix::User { name, .. } => name,
                _ => unreachable!(),
            },
            data: value.data.clone().unwrap(),
            tags: value.tags.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn whisper() {
        let input =
            "@room-id=12345678;tmi-sent-ts=1642715695392 :museun!museun@museun.tmi.twitch.tv WHISPER shaken_bot :this is a test\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);

        assert_eq!(
            test_util::parse_as::<Whisper>(input),
            Whisper {
                raw,
                tags,
                to_user: Cow::from("museun"),
                from_user: Cow::from("shaken_bot"),
                data: Cow::from("this is a test")
            }
        );
    }
}
