use std::borrow::Cow;

use super::{Message, Prefix, Tags, UserType};
use crate::{builders::PrivmsgBuilder, parse_badges, Badge, Color, Emote};

/// A user posts a message to the chat room.
///
/// See [`Sending and Receiving Chat Messages`](https://dev.twitch.tv/docs/irc/send-receive-messages/), use [`Privmsg`](crate::encode) when sending messages.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Privmsg<'a> {
    /// The channel this message was sent to. Prefixed with a `#`
    pub channel: Cow<'a, str>,
    /// The author of the channel
    pub sender: Cow<'a, str>,
    /// Metadata attached to the message
    pub tags: Tags<'a>,
    /// The text message
    pub data: Cow<'a, str>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
}

impl<'a> Privmsg<'a> {
    /// Contains metadata related to the chat badges in the [`badges`](Self::badges) tag.
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    /// Badges attached to a user in a channel.
    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    /// Emotes in the message.
    pub fn emotes<'t: 'a>(&'t self) -> impl Iterator<Item = Emote<'a>> + 't {
        Emote::from_tags(&self.tags, &self.data)
    }

    /// The amount of Bits the user cheered, if the message was a Bits cheer.
    pub fn bits(&self) -> Option<usize> {
        self.tags.parsed("bits").transpose().ok().flatten()
    }

    /// The color of the user’s name in the chat room. This may be [`None`] if it is never set.
    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    /// The user’s display name
    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    /// unknown tag, see instead [`first_msg_from_user`](Self::first_msg_from_user)
    pub fn returning_chatter(&self) -> bool {
        self.tags
            .get("returning-chatter")
            .map(|s| s == "1")
            .unwrap_or_default()
    }

    /// Signifies if this is the users first message, ever, in the chat room.
    pub fn first_msg_from_user(&self) -> bool {
        self.tags
            .get("first-msg")
            .map(|s| s == "1")
            .unwrap_or_default()
    }

    /// The UNIX timestamp.
    pub fn tmi_sent_ts(&self) -> Option<&str> {
        self.tags.get("tmi-sent-ts")
    }

    /// An ID that uniquely identifies the message.
    pub fn msg_id(&self) -> Option<&str> {
        self.tags.get("id")
    }
    /// An ID that identifies the chat room (channel).
    pub fn room_id(&self) -> Option<&str> {
        self.tags.get("room-id")
    }

    /// An ID that uniquely identifies the parent message that this message is replying to.
    pub fn reply_parent_msg_id(&self) -> Option<&str> {
        self.tags.get("reply-parent-msg-id")
    }

    /// An ID that identifies the sender of the parent message.
    pub fn reply_parent_user_id(&self) -> Option<&str> {
        self.tags.get("reply-parent-user-id")
    }

    /// The login name of the sender of the parent message.
    pub fn reply_parent_user_login(&self) -> Option<&str> {
        self.tags.get("reply-parent-user-login")
    }

    /// The display name of the sender of the parent message.
    pub fn reply_parent_display_name(&self) -> Option<&str> {
        self.tags.get("reply-parent-display-name")
    }

    /// The text of the parent message.
    pub fn reply_parent_msg_body(&self) -> Option<&str> {
        self.tags.get("reply-parent-msg-body")
    }

    /// The type of user.
    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }

    /// The user’s ID.
    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    /// The message is from the broadcaster of the channel
    pub fn is_from_broadcaster(&self) -> bool {
        self.badges().any(|badge| badge.name == "broadcaster")
    }

    /// The message is from a moderator in the channel
    pub fn is_from_moderator(&self) -> bool {
        self.badges().any(|badge| badge.name == "moderator")
    }

    /// The message is from a VIP in the channel
    pub fn is_from_vip(&self) -> bool {
        self.badges().any(|badge| badge.name == "vip")
    }

    /// The message is from a subscriber of the channel
    pub fn is_from_subscriber(&self) -> bool {
        self.badges().any(|badge| badge.name == "subscriber")
    }

    /// The message is from Twitch staff
    pub fn is_from_staff(&self) -> bool {
        self.badges().any(|badge| badge.name == "staff")
    }

    /// The message is from a turbo user
    pub fn is_from_turbo(&self) -> bool {
        self.badges().any(|badge| badge.name == "turbo")
    }

    /// The message is from a global moderator
    pub fn is_from_global_moderator(&self) -> bool {
        self.badges().any(|badge| badge.name == "global_mod")
    }

    /// The message is from a admin
    pub fn is_from_admin(&self) -> bool {
        self.badges().any(|badge| badge.name == "admin")
    }

    /// A builder for constructing a [`Message`](crate::messages::Message) or [`Privmsg`](crate::messages::Privmsg)
    pub fn builder() -> PrivmsgBuilder {
        PrivmsgBuilder::default()
    }
}

impl Privmsg<'_> {
    fn validate(value: &Message<'_>) -> bool {
        matches!(value.prefix, Prefix::User { .. })
            && value.data.is_some()
            && !value.args.is_empty()
    }
}

impl<'a> TryFrom<Message<'a>> for Privmsg<'a> {
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

impl<'a, 'b> TryFrom<&'b Message<'a>> for Privmsg<'a> {
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
            test_util::parse_as::<Privmsg>(input),
            Privmsg {
                raw,
                tags,
                channel: Cow::from("#museun"),
                sender: Cow::from("museun"),
                data: Cow::from("testing")
            }
        );
    }
}
