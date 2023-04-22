use std::borrow::Cow;

use crate::{
    messages::{BadgeSetIdRef, ChatBadgeIdRef, IntoCow},
    Tags,
};

/// Parse badges from a string
///
/// ```rust
/// use twitch_message::{Badge, parse_badges};
/// use std::borrow::Cow;
///
/// let input = "broadcaster/1,foo/bar";
/// let expected = [
///     Badge{ set_id: Cow::Borrowed("broadcaster".into()), id: Cow::Borrowed("1".into()) },
///     Badge{ set_id: Cow::Borrowed("foo".into()), id: Cow::Borrowed("bar".into()) },
/// ];
/// for (i, badge) in parse_badges(input).enumerate() {
///     assert_eq!(expected[i], badge)
/// }
/// ```
///
/// If you have a parsed [`Tags`] value, you can use [`Badge::from_tags`]
pub fn parse_badges(input: &str) -> impl Iterator<Item = Badge<'_>> + '_ {
    input
        .split(',')
        .flat_map(|badge| badge.split_once('/'))
        .map(|(set_id, id)| {
            let mut id = Cow::Borrowed(id);
            Badge::unescape(&mut id);
            Badge {
                set_id: Cow::Borrowed(set_id.into()),
                id: IntoCow::into_cow(id),
            }
        })
}

/// A badge attached to a message
#[derive(Debug, Clone, PartialEq, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Badge<'a> {
    /// The set_id or name of the badge
    pub set_id: Cow<'a, BadgeSetIdRef>,
    /// The id, version or metadata for the badge
    pub id: Cow<'a, ChatBadgeIdRef>,
}

impl<'a> PartialOrd for Badge<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        static KNOWN_BADGES: &[&'_ str] = &[
            "staff",
            "admin",
            "global_mod",

            "broadcaster",
            "vip",
            "moderator",

            "partner",

            "subscriber",

            "artist-badge",

            "turbo",
            "premium",

            "bits-leader",
            "bits",
        ];
        match self.set_id.partial_cmp(&other.set_id) {
            Some(core::cmp::Ordering::Equal) => {}
            // XXX: order known badges like first-party site
            // FIXME: is the reflexive and transitive? as needed by Ord
            ord => match (
                KNOWN_BADGES.iter().position(|b| *b == self.set_id.as_str()),
                KNOWN_BADGES
                    .iter()
                    .position(|b| *b == other.set_id.as_str()),
            ) {
                (Some(s), Some(other)) => return s.partial_cmp(&other),
                (Some(_), None) => return Some(core::cmp::Ordering::Less),
                (None, Some(_)) => return Some(core::cmp::Ordering::Greater),
                _ => return ord,
            },
        }
        // XXX: numerical partial_cmp on self.id, so that subscriber/12 > subscriber/9
        match (
            self.id.as_str().parse::<u32>(),
            other.id.as_str().parse::<u32>(),
        ) {
            (Ok(s), Ok(other)) => s.partial_cmp(&other),
            (_, _) => self.id.partial_cmp(&other.id),
        }
    }
}

impl<'a> Badge<'a> {
    /// Parse badges from a [`Tags`]
    ///
    /// ```rust
    /// use twitch_message::{Tags, Badge};
    /// use std::borrow::Cow;
    ///
    /// let tags = Tags::builder().add("badges", "broadcaster/1,foo/bar").finish();
    /// let expected = [
    ///     Badge{ set_id: Cow::Borrowed("broadcaster".into()), id: Cow::Borrowed("1".into()) },
    ///     Badge{ set_id: Cow::Borrowed("foo".into()), id: Cow::Borrowed("bar".into()) },
    /// ];
    /// for (i, badge) in Badge::from_tags(&tags).enumerate() {
    ///     assert_eq!(expected[i], badge)
    /// }
    /// ```
    ///
    /// If you already have a **badges** tag, you can use [`parse_badges`]
    pub fn from_tags<'t: 'a>(tags: &'t Tags<'a>) -> impl Iterator<Item = Badge<'a>> + 't {
        tags.get("badges").into_iter().flat_map(parse_badges)
    }
}

/// Currently an alias for [`Badge`]
pub type BadgeInfo<'a> = Badge<'a>;

impl Badge<'_> {
    fn unescape(s: &mut Cow<'_, str>) {
        const ESCAPED: [char; 1] = ['⸝'];
        const REPLACEMENTS: [char; 1] = [','];

        // XXX: the fast path doesn't reallocate
        if !s.chars().any(|c| ESCAPED.contains(&c)) {
            return;
        }

        *s = s
            .chars()
            .map(|c| {
                if let Some(p) = ESCAPED.iter().position(|&s| s == c) {
                    REPLACEMENTS[p]
                } else {
                    c
                }
            })
            .collect::<String>()
            .into();
    }
}

#[cfg(test)]
#[test]
fn badge_ordering() {
    #[track_caller]
    fn parse(s: &str) -> Badge<'_> {
        parse_badges(s).next().unwrap()
    }
    assert!(parse("subscriber/12") > parse("subscriber/9"));
    assert!(parse("subscriber/0") < parse("subscriber/1"));
    let mut badges: Vec<_> = parse_badges("bits/1,staff/1,broadcaster/1").collect();
    badges.sort();
    assert_eq!(badges.iter().map(|b| b.set_id.as_str()).collect::<Vec<_>>(), vec!["staff", "broadcaster", "bits"]);

    let mut badges: Vec<_> = parse_badges("aaa/1,zzz/1,staff/1").collect();
    badges.sort();
    assert_eq!(badges.iter().map(|b| b.set_id.as_str()).collect::<Vec<_>>(), vec!["staff", "aaa", "zzz"]);

}
