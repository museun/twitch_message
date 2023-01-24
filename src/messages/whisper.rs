use std::borrow::Cow;

use crate::{Badge, Color, Emote};

use super::{Message, Prefix, Tags, UserType};

/// Sent when a `WHISPER` message is directed specifically to the connected user.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Whisper<'a> {
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// The user that’s sending the whisper message.
    pub from_user: Cow<'a, str>,
    /// The user that’s receiving the whisper message.
    pub to_user: Cow<'a, str>,
    /// The text of the whisper
    pub data: Cow<'a, str>,
    /// Metadata attached to the message
    pub tags: Tags<'a>,
}

impl<'a> Whisper<'a> {
    /// Global Badges attached to the [`Self::from_user`].
    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    /// The color of the user’s name. This may be [`None`] if it is never set.
    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    /// The user’s display name
    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    /// Emotes in the message.
    pub fn emotes<'t: 'a>(&'t self) -> impl Iterator<Item = Emote<'a>> + 't {
        Emote::from_tags(&self.tags, &self.data)
    }

    /// An ID that uniquely identifies the whisper message.
    pub fn msg_id(&self) -> Option<&str> {
        self.tags.get("message-id")
    }

    /// An ID that uniquely identifies the whisper thread
    pub fn thread_id(&self) -> Option<&str> {
        self.tags.get("thread-id")
    }

    /// The ID of the user sending the whisper message.
    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    /// User has turbo
    pub fn is_turbo(&self) -> bool {
        self.tags.bool("turbo")
    }

    /// The type of user sending the whisper message.
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
