use std::borrow::Cow;

use crate::messages::PrivMsg;

pub fn parse_badges<'a, 't: 'a>(pm: &'t PrivMsg<'a>) -> impl Iterator<Item = Badge<'a>> + 't {
    pm.tags
        .get("badges")
        .into_iter()
        .flat_map(|badges| badges.split(','))
        .flat_map(|badge| badge.split_once('/'))
        .map(|(name, version)| Badge {
            name: Cow::from(name),
            version: Cow::from(version),
        })
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Badge<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

// XXX: is it meta or info
pub type BadgeInfo<'a> = Badge<'a>;
