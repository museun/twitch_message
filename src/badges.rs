use std::borrow::Cow;

use crate::Tags;

pub fn parse_badges(input: &str) -> impl Iterator<Item = Badge<'_>> + '_ {
    input
        .split(',')
        .flat_map(|badge| badge.split_once('/'))
        .map(|(name, version)| Badge {
            name: Cow::from(name),
            version: Cow::from(version),
        })
}

pub fn badges_from_tags<'a, 't: 'a>(tags: &'t Tags<'a>) -> impl Iterator<Item = Badge<'a>> + 't {
    tags.get("badges").into_iter().flat_map(parse_badges)
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Badge<'a> {
    pub name: Cow<'a, str>,
    pub version: Cow<'a, str>,
}

pub type BadgeInfo<'a> = Badge<'a>;
