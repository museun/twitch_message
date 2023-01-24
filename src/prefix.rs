use std::borrow::Cow;

use crate::Parse;

/// An IRC-styled prefix.
///
/// A prefix is attached to certain messages denoting whom sent it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Prefix<'a> {
    /// A user prefix.
    ///
    /// This is attached for messages sent by a user
    ///
    /// Currently, with Twitch, only `name` is is relevant
    User {
        /// Their (nick) name
        name: Cow<'a, str>,
        /// Their user name
        user: Cow<'a, str>,
        /// Their host
        host: Cow<'a, str>,
    },
    /// A server prefix.
    ///
    /// This is attached for messages sent by the server
    Server {
        /// The host of the server
        host: Cow<'a, str>,
    },
    /// No prefix was attached
    #[default]
    None,
}

impl<'a> Prefix<'a> {
    /// Is this a [User](Self::User) prefix?
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User { .. })
    }

    /// Is this a [Server](Self::Server) prefix?
    pub const fn is_server(&self) -> bool {
        matches!(self, Self::Server { .. })
    }

    /// Get the prefix as a `&str` (the user name, or the server host name)
    pub fn as_name_str(&self) -> Option<&str> {
        match self {
            Self::User { name, .. } | Self::Server { host: name } => Some(name),
            _ => None,
        }
    }
}

impl<'a> Parse<'a> for Prefix<'a> {
    type Output = Self;

    fn parse(input: &mut &'a str) -> Self::Output {
        if !input.starts_with(':') {
            return Self::None;
        }

        let (head, tail) = match input[1..].split_once(' ') {
            Some(val) => val,
            None => return Self::None,
        };
        *input = tail;

        let prefix = match head.find('!') {
            Some(bang) => {
                let name = &head[..bang];
                let (user, host) = match head[bang + 1..].split_once('@').map(|(k, v)| (k, v)) {
                    Some(val) => val,
                    None => return Self::None,
                };
                Self::User {
                    name: Cow::from(name),
                    user: Cow::from(user),
                    host: Cow::from(host),
                }
            }
            None => Self::Server {
                host: Cow::from(head),
            },
        };

        prefix
    }
}
