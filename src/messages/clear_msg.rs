use std::borrow::Cow;

use super::{Message, Tags};

/// [`CLEARMSG`](https://dev.twitch.tv/docs/irc/commands/#clearmsg) command. Sent when a bot or user with moderator privileges deletes a single message from the chat room.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct ClearMsg<'a> {
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// The channel where the `CLEARMSG` was sent
    pub channel: Cow<'a, str>,
    /// The chat message that was deleted
    pub message: Cow<'a, str>,
    /// Metadata attached to the command
    pub tags: Tags<'a>,
}

impl<'a> ClearMsg<'a> {
    /// The name of the user who sent the message.
    pub fn login(&self) -> Option<&str> {
        self.tags.get("login")
    }

    /// The ID of the channel (chat room) where the message was removed from.
    pub fn room_id(&self) -> Option<&str> {
        self.tags.get("room-id")
    }

    /// A UUID that identifies the message that was removed.
    pub fn target_msg_id(&self) -> Option<&str> {
        self.tags.get("target-msg-id")
    }

    /// The UNIX timestamp.
    pub fn tmi_sent_ts(&self) -> Option<&str> {
        self.tags.get("tmi-sent-ts")
    }
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
