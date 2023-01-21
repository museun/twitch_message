use std::borrow::Cow;

use crate::Tags;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Emote<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub byte_pos: (usize, usize),
}

pub fn emotes_from_tags<'a, 't: 'a>(
    tags: &'t Tags<'a>,
    data: &'a str,
) -> impl Iterator<Item = Emote<'a>> + 't {
    tags.get("emotes")
        .into_iter()
        .flat_map(|input| parse_emotes(input, data))
}

pub fn parse_emotes<'a>(input: &'a str, data: &'a str) -> impl Iterator<Item = Emote<'a>> + 'a {
    input
        .split('/')
        .flat_map(|s| s.split_once(':'))
        .flat_map(|(emote, range)| {
            range
                .split(',')
                .flat_map(|c| c.split_once('-').map(|(s, e)| (s.parse(), e.parse())))
                .flat_map(|(start, end)| Some((start.ok()?, end.ok()?)))
                .zip(std::iter::repeat(emote))
                .map(|((start, end), kind): ((usize, usize), _)| (kind, (start, end - start + 1)))
        })
        .map(|(emote, (start, end))| Emote {
            id: Cow::from(emote),
            name: Cow::from(&data[start..start + end]),
            byte_pos: (start, start + end),
        })
}
