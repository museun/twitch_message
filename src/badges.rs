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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Badge<'a> {
    /// The set_id or name of the badge
    pub set_id: Cow<'a, BadgeSetIdRef>,
    /// The id, version or metadata for the badge
    pub id: Cow<'a, ChatBadgeIdRef>,
}

impl Ord for Badge<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other)
            .expect("badges are fully comparable")
    }
}

impl<'a> PartialOrd for Badge<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        static KNOWN_BADGES: &[&str] = &[
            "bits",
            "bits-leader",
            "premium",
            "turbo",
            "artist-badge",
            "subscriber",
            "partner",
            "moderator",
            "vip",
            "broadcaster",
            "global_mod",
            "admin",
            "staff",
        ];

        use core::cmp::Ordering;

        match self.set_id.partial_cmp(&other.set_id) {
            Some(Ordering::Equal) => {}
            // XXX: order known badges like first-party site
            // FIXME: is the reflexive and transitive? as needed by Ord
            ord => match (
                KNOWN_BADGES.iter().position(|b| *b == self.set_id.as_str()),
                KNOWN_BADGES
                    .iter()
                    .position(|b| *b == other.set_id.as_str()),
            ) {
                (Some(left), Some(right)) => return left.partial_cmp(&right),
                (Some(_), None) => return Some(Ordering::Less),
                (None, Some(_)) => return Some(Ordering::Greater),
                _ => return ord,
            },
        }

        let is_all_digit = |s: &Self| s.id.as_str().chars().all(|c| c.is_ascii_digit());
        let parse_num = |s: &Self| {
            is_all_digit(s)
                .then(|| s.id.as_str().parse::<u32>().ok())
                .flatten()
        };

        if !is_all_digit(self) {
            return None;
        }
        if !is_all_digit(other) {
            return None;
        }

        match (parse_num(self), parse_num(other)) {
            (Some(left), Some(right)) if left != right => left.partial_cmp(&right),
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
mod tests {
    use super::*;

    #[test]
    fn composite_ordering() {
        let badges = "premium/1,broadcaster/1,subscriber/5,subscriber/4,subscriber/12";
        let mut badges: Vec<_> = parse_badges(badges).collect();

        let expected = [
            ("premium", "1"),
            ("broadcaster", "1"),
            ("subscriber", "5"),
            ("subscriber", "4"),
            ("subscriber", "12"),
        ]
        .into_iter()
        .map(|(k, v)| Badge {
            set_id: Cow::from(BadgeSetIdRef::from_static(k)),
            id: Cow::from(ChatBadgeIdRef::from_static(v)),
        })
        .collect::<Vec<_>>();

        assert_eq!(badges, expected);

        badges.sort();

        let expected = [
            ("premium", "1"),
            ("subscriber", "4"),
            ("subscriber", "5"),
            ("subscriber", "12"),
            ("broadcaster", "1"),
        ]
        .into_iter()
        .map(|(k, v)| Badge {
            set_id: Cow::from(BadgeSetIdRef::from_static(k)),
            id: Cow::from(ChatBadgeIdRef::from_static(v)),
        })
        .collect::<Vec<_>>();

        assert_eq!(
            badges, expected,
            "\ngot: {badges:#?}\n\nexpected: {expected:#?}\n"
        );
    }

    #[test]
    fn id_ordering() {
        let badges = "U/1U,U/11,U/5";

        let mut badges: Vec<_> = parse_badges(badges).collect();

        let expected = [("U", "1U"), ("U", "11"), ("U", "5")]
            .into_iter()
            .map(|(k, v)| Badge {
                set_id: Cow::from(BadgeSetIdRef::from_static(k)),
                id: Cow::from(ChatBadgeIdRef::from_static(v)),
            })
            .collect::<Vec<_>>();

        assert_eq!(badges, expected);

        badges.sort();

        let expected = [("U", "1U"), ("U", "5"), ("U", "11")]
            .into_iter()
            .map(|(k, v)| Badge {
                set_id: Cow::from(BadgeSetIdRef::from_static(k)),
                id: Cow::from(ChatBadgeIdRef::from_static(v)),
            })
            .collect::<Vec<_>>();

        assert_eq!(badges, expected);
    }

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
        assert_eq!(
            badges.iter().map(|b| b.set_id.as_str()).collect::<Vec<_>>(),
            vec!["bits", "broadcaster", "staff",]
        );

        let mut badges: Vec<_> = parse_badges("aaa/1,zzz/1,staff/1").collect();
        badges.sort();
        assert_eq!(
            badges.iter().map(|b| b.set_id.as_str()).collect::<Vec<_>>(),
            vec!["staff", "aaa", "zzz"]
        );
    }
}
