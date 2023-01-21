use std::borrow::Cow;

use super::{Message, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RoomState<'a> {
    pub tags: Tags<'a>,
    pub raw: Cow<'a, str>,
}

impl<'a> RoomState<'a> {
    pub fn emote_only(&self) -> bool {
        self.tags.bool("emote-only")
    }

    pub fn followers_only(&self) -> Option<usize> {
        self.tags.parsed("followers-only")?.ok()
    }

    pub fn r9k(&self) -> bool {
        self.tags.bool("r9k")
    }

    pub fn room_id(&self) -> Option<&str> {
        self.tags.get("room-id")
    }

    pub fn slow(&self) -> Option<usize> {
        self.tags.parsed("slow")?.ok()
    }

    pub fn subs_only(&self) -> bool {
        self.tags.bool("subs-only")
    }
}

impl<'a> TryFrom<Message<'a>> for RoomState<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for RoomState<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags.clone(),
            raw: value.raw.clone(),
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
            RoomState { raw, tags }
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
