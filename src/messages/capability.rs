use std::borrow::Cow;

use super::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Capability<'a> {
    pub acknowledged: bool,
    pub kind: Cow<'a, str>,
    pub raw: Cow<'a, str>,
}

impl Capability<'_> {
    fn validate(value: &Message<'_>) -> bool {
        value.args.len() == 2 || value.data.is_some()
    }
}

impl<'a> TryFrom<Message<'a>> for Capability<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            acknowledged: value.args[1] == "ACK",
            kind: value.data.unwrap(),
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Capability<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            acknowledged: value.args[1] == "ACK",
            kind: value.data.clone().unwrap(),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn capability_acknowledged() {
        let input = ":tmi.twitch.tv CAP * ACK :foobar\r\n";
        let raw = test_util::raw(input);

        assert_eq!(
            test_util::parse_as::<Capability>(input),
            Capability {
                acknowledged: true,
                kind: Cow::from("foobar"),
                raw,
            }
        );
    }

    #[test]
    fn capability_failed() {
        let input = ":tmi.twitch.tv CAP * NAK :foobar\r\n";
        let raw = test_util::raw(input);

        assert_eq!(
            test_util::parse_as::<Capability>(input),
            Capability {
                acknowledged: false,
                kind: Cow::from("foobar"),
                raw,
            }
        );
    }
}
