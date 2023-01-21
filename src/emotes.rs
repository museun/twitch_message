use std::borrow::Cow;

use crate::messages::PrivMsg;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Emote<'a> {
    pub id: Cow<'a, str>,
    pub name: Cow<'a, str>,
    pub byte_pos: (usize, usize),
}

pub fn parse_emotes<'a, 't: 'a>(msg: &'t PrivMsg<'a>) -> impl Iterator<Item = Emote<'a>> + 't {
    msg.tags
        .get("emotes")
        .into_iter()
        .flat_map(|emotes| {
            emotes
                .split('/')
                .flat_map(|s| s.split_once(':'))
                .flat_map(|(emote, range)| {
                    range
                        .split(',')
                        .flat_map(|c| c.split_once('-').map(|(s, e)| (s.parse(), e.parse())))
                        .flat_map(|(start, end)| Some((start.ok()?, end.ok()?)))
                        .zip(std::iter::repeat(emote))
                        .map(|((start, end), kind): ((usize, usize), _)| {
                            (kind, (start, end - start + 1))
                        })
                })
        })
        .map(|(emote, (start, end))| Emote {
            id: Cow::from(emote),
            name: Cow::from(&msg.data[start..start + end]),
            byte_pos: (start, start + end),
        })
}
