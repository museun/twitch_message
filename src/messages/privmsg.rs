use std::borrow::Cow;

use super::{Message, Prefix, Tags, UserType};
use crate::{
    badges::badges_from_tags, emotes::emotes_from_tags, parse_badges, Badge, Color, Emote,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Privmsg<'a> {
    pub channel: Cow<'a, str>,
    pub sender: Cow<'a, str>,
    pub tags: Tags<'a>,
    pub data: Cow<'a, str>,
    pub raw: Cow<'a, str>,
}

impl<'a> Privmsg<'a> {
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        badges_from_tags(&self.tags)
    }

    pub fn emotes<'t: 'a>(&'t self) -> impl Iterator<Item = Emote<'a>> + 't {
        emotes_from_tags(&self.tags, &self.data)
    }

    pub fn bits(&self) -> Option<usize> {
        self.tags.parsed("bits").transpose().ok().flatten()
    }

    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    pub fn returning_chatter(&self) -> bool {
        self.tags
            .get("returning-chatter")
            .map(|s| s == "1")
            .unwrap_or_default()
    }

    pub fn first_msg_from_user(&self) -> bool {
        self.tags
            .get("first-msg")
            .map(|s| s == "1")
            .unwrap_or_default()
    }

    pub fn tmi_sent_ts(&self) -> Option<&str> {
        self.tags.get("tmi-sent-ts")
    }

    pub fn msg_id(&self) -> Option<&str> {
        self.tags.get("id")
    }

    pub fn room_id(&self) -> Option<&str> {
        self.tags.get("room_id")
    }

    pub fn reply_parent_msg_id(&self) -> Option<&str> {
        self.tags.get("reply-parent-msg-id")
    }

    pub fn reply_parent_user_id(&self) -> Option<&str> {
        self.tags.get("reply-parent-user-id")
    }

    pub fn reply_parent_user_login(&self) -> Option<&str> {
        self.tags.get("reply-parent-user-login")
    }

    pub fn reply_parent_display_name(&self) -> Option<&str> {
        self.tags.get("reply-parent-display-name")
    }

    pub fn reply_parent_msg_body(&self) -> Option<&str> {
        self.tags.get("reply-parent-msg-body")
    }

    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }

    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    pub fn is_from_broadcaster(&self) -> bool {
        self.badges().any(|badge| badge.name == "broadcaster")
    }

    pub fn is_from_moderator(&self) -> bool {
        self.badges().any(|badge| badge.name == "moderator")
    }

    pub fn is_from_vip(&self) -> bool {
        self.badges().any(|badge| badge.name == "vip")
    }

    pub fn is_from_subscriber(&self) -> bool {
        self.badges().any(|badge| badge.name == "subscriber")
    }

    pub fn is_from_staff(&self) -> bool {
        self.badges().any(|badge| badge.name == "staff")
    }

    pub fn is_from_turbo(&self) -> bool {
        self.badges().any(|badge| badge.name == "turbo")
    }

    pub fn is_from_global_moderator(&self) -> bool {
        self.badges().any(|badge| badge.name == "global_mod")
    }

    pub fn is_from_admin(&self) -> bool {
        self.badges().any(|badge| badge.name == "admin")
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
