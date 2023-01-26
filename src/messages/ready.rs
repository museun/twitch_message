use std::borrow::Cow;

use super::{IntoCow, Message, UserNameRef};

/// A TMI-styled ready, sent after [`IrcReady`](super::IrcReady)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Ready<'a> {
    /// The name of the connected user
    pub name: Cow<'a, UserNameRef>,
    /// The raw underlying string
    pub raw: Cow<'a, str>,
}

impl Ready<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
    }
}

impl<'a> TryFrom<Message<'a>> for Ready<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            name: IntoCow::into_cow(value.args.remove(0)),
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Ready<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            name: IntoCow::into_cow(value.args[0].clone()),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn ready() {
        let input = ":tmi.twitch.tv 376 museun :>\r\n";

        let raw = test_util::raw(input);
        assert_eq!(
            test_util::parse_as::<Ready>(input),
            Ready {
                name: IntoCow::into_cow("museun"),
                raw,
            }
        );
    }
}
