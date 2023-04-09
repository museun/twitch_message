use std::borrow::Cow;

use crate::{parse_badges, Badge, Color};

use super::{EmoteSetIdRef, Message, Tags, UserType};

/// State received after joining a channel or sending a [`Privmsg`](crate::encode::Privmsg)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct UserState<'a> {
    /// Metadata attached to the message
    pub tags: Tags<'a>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// The channel this event happened on
    pub channel: Cow<'a, str>,
}

impl<'a> UserState<'a> {
    /// Contains metadata related to the chat badges in the [`badges`](Self::badges) tag.
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    /// Badges attached to a user in a channel.
    /// Badges attached to a user in a channel.
    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    /// The color of the user’s name in the chat room. This may be [`None`] if it is never set.
    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    /// The user’s display name
    pub fn display_name(&self) -> Option<&super::DisplayNameRef> {
        self.tags.get("display-name").map(Into::into)
    }

    /// An ID that uniquely identifies the message.
    pub fn msg_id(&self) -> Option<&super::MsgIdRef> {
        self.tags.get("id").map(Into::into)
    }

    /// The user is a moderator in the channel
    pub fn is_moderator(&self) -> bool {
        self.tags.bool("mod")
    }

    /// The user is a subscriber of the channel
    pub fn is_subscriber(&self) -> bool {
        self.tags.bool("subscriber")
    }

    /// The user has turbo.
    pub fn is_turbo(&self) -> bool {
        self.tags.bool("turbo")
    }

    /// The user’s type.
    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }

    /// A comma-delimited list of IDs that identify the emote sets that the user has access to. To access the emotes in the set, use the [Get Emote Sets](https://dev.twitch.tv/docs/api/reference#get-emote-sets) API.
    pub fn emote_sets(&self) -> impl Iterator<Item = &EmoteSetIdRef> {
        self.tags
            .get("emote-sets")
            .into_iter()
            .flat_map(|s| s.split(','))
            .map(Into::into)
    }
}

impl<'a> TryFrom<Message<'a>> for UserState<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
            channel: value.args.remove(0),
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for UserState<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags.clone(),
            raw: value.raw.clone(),
            channel: value
                .args
                .get(0)
                .cloned()
                .expect("channel attached to message"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn user_state() {
        let input = "@badge-info=;\
        badges=broadcaster/1,premium/1;\
        color=#008000;\
        display-name=museun;\
        emote-sets=0,19194,300374282,300597048,301337952,460515209,537206155,564265402,592920959,610186276;\
        mod=0;\
        subscriber=0;\
        user-type= :tmi.twitch.tv USERSTATE #museun\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            crate::test_util::parse_as::<UserState>(input),
            UserState {
                raw,
                tags,
                channel: Cow::from("#museun")
            }
        );
    }
}
