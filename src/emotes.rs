use std::borrow::Cow;

use crate::Tags;

/// An emote attached to a message
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Emote<'a> {
    /// The `id` of the emote (e.g. `25` for `Kappa`)
    pub id: Cow<'a, str>,
    /// The `name` of the emote (e.g. `Kappa`)
    pub name: Cow<'a, str>,
    /// The byte position of the emote (name) in the provided `data`
    pub byte_pos: (usize, usize),
}

impl<'a> Emote<'a> {
    /// Parse the emotes from tags and associated data
    ///
    /// ```rust
    /// use twitch_message::{Tags, Emote};
    /// use std::borrow::Cow;
    ///
    /// let tags = Tags::builder().add("emotes", "25:0-4,14-18").finish();
    /// let data = "Kappa testing Kappa";
    /// let expected = [
    ///     Emote { id: Cow::from("25"), name: Cow::from("Kappa"), byte_pos: (0, 5) },
    ///     Emote { id: Cow::from("25"), name: Cow::from("Kappa"), byte_pos: (14, 19) },
    /// ];
    /// for (i, emote) in Emote::from_tags(&tags, data).enumerate() {
    ///     assert_eq!(expected[i], emote);
    /// }
    /// ```
    /// ## See also
    /// Parsing from a string, such as from `tags.get("emotes")`, you can use [`parse_emotes`]
    pub fn from_tags<'t: 'a>(
        tags: &'t Tags<'a>,
        data: &'a str,
    ) -> impl Iterator<Item = Emote<'a>> + 't {
        tags.get("emotes")
            .into_iter()
            .flat_map(|input| parse_emotes(input, data))
    }
}

/// Parse emotes from a tag value and associated data
///
/// ```rust
/// use twitch_message::{Tags, parse_emotes, Emote};
/// use std::borrow::Cow;
///
/// let emotes = "25:0-4,14-18";
/// let data = "Kappa testing Kappa";
/// let expected = [
///     Emote { id: Cow::from("25"), name: Cow::from("Kappa"), byte_pos: (0, 5) },
///     Emote { id: Cow::from("25"), name: Cow::from("Kappa"), byte_pos: (14, 19) },
/// ];
/// for (i, emote) in parse_emotes(emotes, data).enumerate() {
///     assert_eq!(expected[i], emote);
/// }
/// ```
/// ## See also
/// If you have an already parsed [`Tags`] you can use [`Emote::from_tags`]
pub fn parse_emotes<'a>(input: &'a str, data: &'a str) -> impl Iterator<Item = Emote<'a>> + 'a {
    fn substr(data: &str, start: &mut usize, end: &mut usize) -> Cow<'static, str> {
        let (s, e) = (*start, *end);
        *start = data.chars().map(|s| s.len_utf8()).take(s).sum();
        *end = data.chars().map(|s| s.len_utf8()).take(e).sum();
        data.chars().skip(s).take(e).collect()
    }

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
        .map(|(emote, (mut start, end))| {
            let (end, start) = (&mut (end + start), &mut start);

            Emote {
                id: Cow::from(emote),
                name: substr(data, start, end),
                byte_pos: (*start, *end),
            }
        })
}

impl<'a> std::ops::Index<&Emote<'a>> for str {
    type Output = str;
    fn index(&self, index: &Emote<'a>) -> &Self::Output {
        let (s, e) = index.byte_pos;
        &self[s..e]
    }
}
