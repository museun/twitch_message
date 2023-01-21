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
