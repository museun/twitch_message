use std::borrow::Cow;

use super::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct HostTarget<'a> {
    pub raw: Cow<'a, str>,
    pub hosting_channel: Cow<'a, str>,
    pub host_mode: HostMode,
}

impl HostTarget<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty() && value.data.is_some()
    }

    fn parse_host_mode(input: &str) -> Option<HostMode> {
        if let Some(viewers) = input.strip_prefix("- ") {
            return Some(HostMode::End {
                viewers: viewers.parse().ok()?,
            });
        }

        let (start, end) = (0, input.find(' ')?);

        if let Ok(viewers) = input[end + 1..].parse() {
            return Some(HostMode::Start {
                channel: (start, end),
                viewers,
            });
        }

        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum HostMode {
    Start {
        channel: (usize, usize),
        viewers: usize,
    },
    End {
        viewers: usize,
    },
}

impl<'a> TryFrom<Message<'a>> for HostTarget<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        let data = value.data.as_deref().unwrap();
        let host_mode = match Self::parse_host_mode(data) {
            Some(mode) => mode,
            None => return Err(value),
        };

        Ok(Self {
            raw: value.raw,
            hosting_channel: value.args.remove(0),
            host_mode,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for HostTarget<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        let data = value.data.as_deref().unwrap();
        let host_mode = match Self::parse_host_mode(data) {
            Some(mode) => mode,
            None => return Err(value),
        };

        Ok(Self {
            raw: value.raw.clone(),
            hosting_channel: value.args[0].clone(),
            host_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn host_target_start() {
        let input = ":tmi.twitch.tv HOSTTARGET #museun :shaken_bot 10\r\n";
        let raw = test_util::raw(input);

        assert_eq!(
            test_util::parse_as::<HostTarget>(input),
            HostTarget {
                raw,
                hosting_channel: Cow::from("#museun"),
                host_mode: HostMode::Start {
                    channel: (0, "shaken_bot".len()),
                    viewers: 10
                }
            }
        );
    }

    #[test]
    fn host_target_end() {
        let input = ":tmi.twitch.tv HOSTTARGET #museun :- 10\r\n";
        let raw = test_util::raw(input);

        assert_eq!(
            test_util::parse_as::<HostTarget>(input),
            HostTarget {
                raw,
                hosting_channel: Cow::from("#museun"),
                host_mode: HostMode::End { viewers: 10 }
            }
        );
    }
}
