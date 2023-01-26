use std::borrow::Cow;

use super::{IntoCow, Message, Tags};

/// [`CLEARCHAT`](https://dev.twitch.tv/docs/irc/commands/#clearchat) command. Sent when a bot or moderator removes all messages from the chat room or removes all messages for the specified user.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ClearChat<'a> {
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// The channel where the `CLEARCHAT` was sent
    pub channel: Cow<'a, str>,
    /// The target of the `CLEARCHAT`
    pub target: ClearChatTarget<'a>,
    /// Metadata attached to the command
    pub tags: Tags<'a>,
}

impl<'a> ClearChat<'a> {
    /// The duration in seconds the user has been timed out for. Is [`None`] if the command targets all chat messages.
    pub fn ban_duration(&self) -> Option<usize> {
        self.tags.parsed("ban-duration").transpose().ok().flatten()
    }

    /// The ID of the channel where the messages were removed from.
    pub fn room_id(&self) -> Option<&super::UserIdRef> {
        self.tags.get("room-id").map(Into::into)
    }

    /// The ID of the user that was banned or put in a timeout. The user was banned if the message doesn’t include the ban-duration tag.
    pub fn target_user_id(&self) -> Option<&super::UserIdRef> {
        self.tags.get("target-user-id").map(Into::into)
    }

    /// The UNIX timestamp.
    pub fn tmi_sent_ts(&self) -> Option<&str> {
        self.tags.get("tmi-sent-ts")
    }
}

impl ClearChat<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
    }
}

/// The target of a [`ClearChat`]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum ClearChatTarget<'a> {
    /// The `CLEARCHAT` targets all chat messages
    All,
    /// The `CLEARCHAT` targets the specified user with login
    User(Cow<'a, super::UserNameRef>),
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
                .map(IntoCow::into_cow)
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
                .map(IntoCow::into_cow)
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
                target: ClearChatTarget::User(IntoCow::into_cow("shaken_bot"))
            }
        );
    }
}
