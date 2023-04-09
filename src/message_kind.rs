use std::borrow::Cow;

use crate::{Error, Parse};

/// The kind of the [`Message`](crate::messages::Message)
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum MessageKind<'a> {
    /// [`Capability`](super::messages::Capability)
    Capability,
    /// [`Ping`](super::messages::Ping)
    Ping,
    /// [`Pong`](super::messages::Pong)
    Pong,
    /// [`IrcReady`](super::messages::IrcReady)
    IrcReady,
    /// [`Ready`](super::messages::Ready)
    Ready,
    /// [`GlobalUserState`](super::messages::GlobalUserState)
    GlobalUserState,
    /// [`UserState`](super::messages::UserState)
    UserState,
    /// [`RoomState`](super::messages::RoomState)
    RoomState,
    /// [`Privmsg`](super::messages::Privmsg)
    Privmsg,
    /// [`Join`](super::messages::Join)
    Join,
    /// [`Part`](super::messages::Part)
    Part,
    /// [`ClearChat`](super::messages::ClearChat)
    ClearChat,
    /// [`ClearMsg`](super::messages::ClearMsg)
    ClearMsg,
    /// [`Notice`](super::messages::Notice)
    Notice,
    /// [`HostTarget`](super::messages::HostTarget)
    HostTarget,
    /// [`UserNotice`](super::messages::UserNotice)
    UserNotice,
    /// [`Whisper`](super::messages::Whisper)
    Whisper,
    /// [`Reconnect`](super::messages::Reconnect)
    Reconnect,
    /// IRC Numeric
    Numeric(u16),
    /// An unknown message kind
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
            Self::Join => "Join",
            Self::Part => "Part",
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

            "JOIN" => Self::Join,
            "PART" => Self::Part,

            s if s.chars().all(|c| c.is_ascii_digit()) => {
                Self::Numeric(s.parse().map_err(|_| Error::InvalidNumeric)?)
            }
            unknown => Self::Unknown(Cow::from(unknown)),
        };
        Ok(kind)
    }
}
