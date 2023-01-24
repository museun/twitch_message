use std::borrow::Cow;

use crate::{parse_badges, Badge, Color};

use super::{Message, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GlobalUserState<'a> {
    pub tags: Tags<'a>,
    pub raw: Cow<'a, str>,
}

impl<'a> GlobalUserState<'a> {
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    pub fn emote_sets(&self) -> impl Iterator<Item = &str> {
        self.tags
            .get("emote-sets")
            .into_iter()
            .flat_map(|s| s.split(','))
    }

    pub fn is_turbo(&self) -> bool {
        self.tags.bool("turbo")
    }

    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum UserType {
    Admin,
    GlobalMod,
    Staff,
    #[default]
    Normal,
}

impl UserType {
    pub(crate) fn parse(input: &str) -> Self {
        match input {
            "admin" => Self::Admin,
            "global_mod" => Self::GlobalMod,
            "staff" => Self::Staff,
            _ => Self::Normal,
        }
    }
}

impl<'a> TryFrom<Message<'a>> for GlobalUserState<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for GlobalUserState<'a> {
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
    use crate::test_util;

    use super::*;
    #[test]
    fn global_user_state() {
        let input = "@badge-info=;\
        badges=premium/1;\
        color=#008000;\
        display-name=museun;\
        emote-sets=0,19194,300374282,300597048,301337952,460515209,537206155,564265402,592920959,610186276;\
        user-id=23196011;\
        user-type= \
        :tmi.twitch.tv GLOBALUSERSTATE\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<GlobalUserState>(input),
            GlobalUserState { raw, tags }
        );
    }
}
