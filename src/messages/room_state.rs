use std::borrow::Cow;

use super::{Message, Tags, UserIdRef};

/// [`ROOMSTATE`](https://dev.twitch.tv/docs/irc/commands/#roomstate). Sent when the bot joins a channel or when the channel’s chat settings change.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RoomState<'a> {
    /// Metadata attached to the message
    pub tags: Tags<'a>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// The channel this event happened on
    pub channel: Cow<'a, str>,
}

impl<'a> RoomState<'a> {
    /// Room is "emote" only
    pub fn emote_only(&self) -> bool {
        self.tags.bool("emote-only")
    }

    /// Room is followers only. The value indicates how long, in minutes, the user must have followed the broadcaster before posting chat messages.
    pub fn followers_only(&self) -> Option<usize> {
        self.tags.parsed("followers-only")?.ok()
    }

    /// Room is r9k/unique only
    pub fn r9k(&self) -> bool {
        self.tags.bool("r9k")
    }

    /// An ID that identifies the chat room (channel).
    pub fn room_id(&self) -> Option<&UserIdRef> {
        self.tags.get("room-id").map(Into::into)
    }

    /// Room is in slow mode. The value determines how long, in seconds, users must wait between sending messages.
    pub fn slow(&self) -> Option<usize> {
        self.tags.parsed("slow")?.ok()
    }

    /// Room is subscribers and moderators only.
    pub fn subs_only(&self) -> bool {
        self.tags.bool("subs-only")
    }
}

impl<'a> TryFrom<Message<'a>> for RoomState<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
            channel: value.args.remove(0),
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for RoomState<'a> {
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
    fn room_state() {
        let input = "@emote-only=0;followers-only=-1;r9k=0;room-id=23196011;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #museun\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);

        assert_eq!(
            test_util::parse_as::<RoomState>(input),
            RoomState {
                raw,
                tags,
                channel: Cow::from("#museun")
            }
        );
    }
}

// "display-name": "museun",
// "emotes": "",
// "room-id": "23196011",
// "badges": "broadcaster/1,premium/1",
// "returning-chatter": "0",
// "tmi-sent-ts": "1674253118899",
// "subscriber": "0",
// "mod": "0",
// "user-type": "",
// "id": "0bf7d0eb-e1aa-490d-907e-bf64c5cac6ac",
// "color": "#008000",
// "badge-info": "",
// "turbo": "0",
// "user-id": "23196011",
// "flags": "",
// "first-msg": "0",
