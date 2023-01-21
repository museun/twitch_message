use std::borrow::Cow;

use crate::{badges::badges_from_tags, parse_badges, Badge, Color};

use super::{Message, Tags, UserType};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct UserState<'a> {
    pub tags: Tags<'a>,
    pub raw: Cow<'a, str>,
}

impl<'a> UserState<'a> {
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        badges_from_tags(&self.tags)
    }

    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    pub fn msg_id(&self) -> Option<&str> {
        self.tags.get("id")
    }

    pub fn is_moderator(&self) -> bool {
        self.tags.bool("mod")
    }

    pub fn is_subscriber(&self) -> bool {
        self.tags.bool("subscriber")
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

    pub fn emote_sets(&self) -> impl Iterator<Item = &str> {
        self.tags
            .get("emote-sets")
            .into_iter()
            .flat_map(|s| s.split(','))
    }
}

impl<'a> TryFrom<Message<'a>> for UserState<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for UserState<'a> {
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
            UserState { raw, tags }
        );
    }
}
