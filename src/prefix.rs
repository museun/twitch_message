use std::borrow::Cow;

use crate::Parse;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Prefix<'a> {
    User {
        name: Cow<'a, str>,
        user: Cow<'a, str>,
        host: Cow<'a, str>,
    },
    Server {
        host: Cow<'a, str>,
    },
    #[default]
    None,
}

impl<'a> Prefix<'a> {
    pub const fn is_user(&self) -> bool {
        matches!(self, Self::User { .. })
    }

    pub const fn is_server(&self) -> bool {
        matches!(self, Self::Server { .. })
    }

    pub fn as_str(&self) -> Option<&str> {
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
