use std::borrow::Cow;

use super::Message;

/// Happens when a user joins a channel
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Join<'a> {
    /// The username that joined the channel
    pub user: Cow<'a, str>,
    /// The channel the user joined
    pub channel: Cow<'a, str>,
}

impl<'a> TryFrom<Message<'a>> for Join<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            user: match value.prefix {
                crate::Prefix::User { name, .. } => name,
                _ => return Err(value),
            },
            channel: value.args.remove(0),
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Join<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            user: value
                .prefix
                .as_name_str()
                .map(ToOwned::to_owned)
                .map(Cow::from)
                .ok_or_else(|| value)?,
            channel: value.args.get(0).cloned().ok_or_else(|| value)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part() {
        let input =
            ":justinfan1234!justinfan1234@justinfan1234.tmi.twitch.tv JOIN #some_channel\r\n";

        assert_eq!(
            crate::test_util::parse_as::<Join>(input),
            Join {
                channel: Cow::from("#some_channel"),
                user: Cow::from("justinfan1234")
            }
        );
    }
}
