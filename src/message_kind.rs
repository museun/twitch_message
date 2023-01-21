use std::borrow::Cow;

use crate::{Error, Parse};

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MessageKind<'a> {
    Capability,
    Ping,
    Pong,
    IrcReady,
    Ready,
    GlobalUserState,
    UserState,
    RoomState,
    Privmsg,
    ClearChat,
    ClearMsg,
    Notice,
    HostTarget,
    UserNotice,
    Whisper,
    Reconnect,
    Numeric(u16),
    Unknown(Cow<'a, str>),
}

impl MessageKind<'_> {
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::Capability => "Capability",
            Self::Ping => "Ping",
            Self::Pong => "Pong",
            Self::IrcReady => "IrcReady",
            Self::Ready => "Ready",
            Self::GlobalUserState => "GlobalUserState",
            Self::UserState => "UserState",
            Self::RoomState => "RoomState",
            Self::Privmsg => "Privmsg",
            Self::ClearChat => "ClearChat",
            Self::ClearMsg => "ClearMsg",
            Self::Notice => "Notice",
            Self::HostTarget => "HostTarget",
            Self::UserNotice => "UserNotice",
            Self::Whisper => "Whisper",
            Self::Reconnect => "Reconnect",
            Self::Numeric(_) => "Numeric",
            Self::Unknown(_) => "Unknown",
        }
    }
}

impl<'a> Parse<'a> for MessageKind<'a> {
    type Output = Result<Self, Error>;

    fn parse(input: &mut &'a str) -> Self::Output {
        let head = match input.split_once(' ') {
            Some((head, tail)) => {
                *input = tail;
                head
            }
            None => {
                let head = *input;
                *input = "";
                head
            }
        };

        let kind = match head {
            "CAP" => Self::Capability,

            "PING" => Self::Ping,
            "PONG" => Self::Pong,

            "001" => Self::IrcReady,
            "376" => Self::Ready,

            "GLOBALUSERSTATE" => Self::GlobalUserState,
            "USERSTATE" => Self::UserState,
            "ROOMSTATE" => Self::RoomState,

            "PRIVMSG" => Self::Privmsg,
            "NOTICE" => Self::Notice,

            "CLEARCHAT" => Self::ClearChat,
            "CLEARMSG" => Self::ClearMsg,
            "HOSTTARGET" => Self::HostTarget,

            "USERNOTICE" => Self::UserNotice,

            "WHISPER" => Self::Whisper,

            "RECONNECT" => Self::Reconnect,

            s if s.chars().all(|c| c.is_ascii_digit()) => {
                Self::Numeric(s.parse().map_err(|_| Error::InvalidNumeric)?)
            }
            unknown => Self::Unknown(Cow::from(unknown)),
        };
        Ok(kind)
    }
}
