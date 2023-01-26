#![allow(deprecated)]
use crate::messages::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
#[allow(deprecated)]
/// All possible Twitch messages created by this crate
pub enum TwitchMessage<'a> {
    /// State received after joining a channel or sending a [`Privmsg`](crate::encode::Privmsg)
    UserState(UserState<'a>),
    /// [`USERNOTICE`](https://dev.twitch.tv/docs/irc/commands/#usernotice). Sent when events like someone subscribing to the channel occurs.
    UserNotice(UserNotice<'a>),
    /// [`RECONNECT`](https://dev.twitch.tv/docs/irc/commands/#reconnect). is sent when the Twitch IRC server needs to terminate the connection.
    Reconnect(Reconnect<'a>),
    #[deprecated(
        note = "hosting has been deprecated, see https://help.twitch.tv/s/article/how-to-use-host-mode?language=en_US"
    )]
    /// A HostTarget of the `/host` command
    HostTarget(HostTarget<'a>),
    /// [`ROOMSTATE`](https://dev.twitch.tv/docs/irc/commands/#roomstate). Sent when the bot joins a channel or when the channel’s chat settings change.
    RoomState(RoomState<'a>),
    /// A 001 IRC-styled Ready
    IrcReady(IrcReady<'a>),
    /// A user posts a message to the chat room.
    Privmsg(Privmsg<'a>),
    /// Sent when a `WHISPER` message is directed specifically to the connected user.
    Whisper(Whisper<'a>),
    /// [`NOTICE`](https://dev.twitch.tv/docs/irc/commands/#notice) Sent to indicate the outcome of an action like banning a user.
    #[deprecated(
        note = "twitch has deprecated chat commands through irc, see https://discuss.dev.twitch.tv/t/deprecation-of-chat-commands-through-irc/40486 "
    )]
    Notice(Notice<'a>),
    /// Tests the presence of a connection. A [PING](Self) message results in a [PONG](crate::encode::Pong) reply.
    Ping(Ping<'a>),
    /// This command is a reply to the [PING](crate::encode::Ping) command
    Pong(Pong<'a>),
    /// A TMI-styled ready, sent after [`IrcReady`](super::IrcReady)
    Ready(Ready<'a>),
    /// [`GLOBALUSERSTATE`](https://dev.twitch.tv/docs/irc/commands/#globaluserstate) command. The Twitch IRC server sends this message after the bot authenticates with the server.
    GlobalUserState(GlobalUserState<'a>),
    /// [`CLEARMSG`](https://dev.twitch.tv/docs/irc/commands/#clearmsg) command. Sent when a bot or user with moderator privileges deletes a single message from the chat room.
    ClearMsg(ClearMsg<'a>),
    /// A capability signals extra functionality, received when requesting capabilities on server join
    Capability(Capability<'a>),
    /// [`CLEARCHAT`](https://dev.twitch.tv/docs/irc/commands/#clearchat) command. Sent when a bot or moderator removes all messages from the chat room or removes all messages for the specified user.
    ClearChat(ClearChat<'a>),
    /// A twitch chat message.
    ///
    /// This is a 'catchall' for when a message cannot be turned into a [`self::TwitchMessage`]
    Message(Message<'a>),
}

impl<'a> From<UserState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: UserState<'a>) -> Self {
        Self::UserState(ty)
    }
}

impl<'a, 'b: 'a> From<&'b UserState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b UserState<'a>) -> Self {
        Self::UserState(ty.clone())
    }
}

impl<'a> From<UserNotice<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: UserNotice<'a>) -> Self {
        Self::UserNotice(ty)
    }
}

impl<'a, 'b: 'a> From<&'b UserNotice<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b UserNotice<'a>) -> Self {
        Self::UserNotice(ty.clone())
    }
}

impl<'a> From<Reconnect<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Reconnect<'a>) -> Self {
        Self::Reconnect(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Reconnect<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Reconnect<'a>) -> Self {
        Self::Reconnect(ty.clone())
    }
}

impl<'a> From<HostTarget<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: HostTarget<'a>) -> Self {
        Self::HostTarget(ty)
    }
}

impl<'a, 'b: 'a> From<&'b HostTarget<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b HostTarget<'a>) -> Self {
        Self::HostTarget(ty.clone())
    }
}

impl<'a> From<RoomState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: RoomState<'a>) -> Self {
        Self::RoomState(ty)
    }
}

impl<'a, 'b: 'a> From<&'b RoomState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b RoomState<'a>) -> Self {
        Self::RoomState(ty.clone())
    }
}

impl<'a> From<IrcReady<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: IrcReady<'a>) -> Self {
        Self::IrcReady(ty)
    }
}

impl<'a, 'b: 'a> From<&'b IrcReady<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b IrcReady<'a>) -> Self {
        Self::IrcReady(ty.clone())
    }
}

impl<'a> From<Privmsg<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Privmsg<'a>) -> Self {
        Self::Privmsg(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Privmsg<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Privmsg<'a>) -> Self {
        Self::Privmsg(ty.clone())
    }
}

impl<'a> From<Whisper<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Whisper<'a>) -> Self {
        Self::Whisper(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Whisper<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Whisper<'a>) -> Self {
        Self::Whisper(ty.clone())
    }
}

impl<'a> From<Notice<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Notice<'a>) -> Self {
        Self::Notice(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Notice<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Notice<'a>) -> Self {
        Self::Notice(ty.clone())
    }
}

impl<'a> From<Ping<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Ping<'a>) -> Self {
        Self::Ping(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Ping<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Ping<'a>) -> Self {
        Self::Ping(ty.clone())
    }
}

impl<'a> From<Pong<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Pong<'a>) -> Self {
        Self::Pong(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Pong<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Pong<'a>) -> Self {
        Self::Pong(ty.clone())
    }
}

impl<'a> From<Ready<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Ready<'a>) -> Self {
        Self::Ready(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Ready<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Ready<'a>) -> Self {
        Self::Ready(ty.clone())
    }
}

impl<'a> From<GlobalUserState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: GlobalUserState<'a>) -> Self {
        Self::GlobalUserState(ty)
    }
}

impl<'a, 'b: 'a> From<&'b GlobalUserState<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b GlobalUserState<'a>) -> Self {
        Self::GlobalUserState(ty.clone())
    }
}

impl<'a> From<ClearMsg<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: ClearMsg<'a>) -> Self {
        Self::ClearMsg(ty)
    }
}

impl<'a, 'b: 'a> From<&'b ClearMsg<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b ClearMsg<'a>) -> Self {
        Self::ClearMsg(ty.clone())
    }
}

impl<'a> From<Capability<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Capability<'a>) -> Self {
        Self::Capability(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Capability<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Capability<'a>) -> Self {
        Self::Capability(ty.clone())
    }
}

impl<'a> From<ClearChat<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: ClearChat<'a>) -> Self {
        Self::ClearChat(ty)
    }
}

impl<'a, 'b: 'a> From<&'b ClearChat<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b ClearChat<'a>) -> Self {
        Self::ClearChat(ty.clone())
    }
}

impl<'a> From<Message<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: Message<'a>) -> Self {
        Self::Message(ty)
    }
}

impl<'a, 'b: 'a> From<&'b Message<'a>> for TwitchMessage<'a> {
    #[inline]
    fn from(ty: &'b Message<'a>) -> Self {
        Self::Message(ty.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion() {
        use std::borrow::Cow;

        let inputs = [
"@emote-only=0;followers-only=-1;r9k=0;room-id=23196011;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #museun\r\n",
":tmi.twitch.tv RECONNECT\r\n",
":tmi.twitch.tv 376 museun :>\r\n",
"PONG :1234567890\r\n",
"PING :1234567890\r\n",
":tmi.twitch.tv 001 museun :Welcome, GLHF!\r\n",
":tmi.twitch.tv HOSTTARGET #museun :shaken_bot 10\r\n",
"@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARCHAT #museun :shaken_bot\r\n",
"@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv CLEARMSG #museun :Kappa\r\n",
":tmi.twitch.tv CAP * ACK :foobar\r\n",
"@emote-only=0;followers-only=-1;r9k=0;room-id=23196011;slow=0;subs-only=0 :tmi.twitch.tv ROOMSTATE #museun\r\n",
":museun!museun@museun.tmi.twitch.tv PRIVMSG #museun :hello world\r\n",
"@room-id=12345678;tmi-sent-ts=1642715695392 :museun!museun@museun.tmi.twitch.tv WHISPER shaken_bot :this is a test\r\n",
"@badge-info=;badges=premium/1;color=#008000;display-name=museun;emote-sets=0,19194,300374282,300597048,301337952,460515209,537206155,564265402,592920959,610186276;user-id=23196011;user-type= :tmi.twitch.tv GLOBALUSERSTATE\r\n"
        ];

        let expected = vec![
            TwitchMessage::RoomState(RoomState {
                tags: Tags::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::Reconnect(Reconnect {
                raw: Cow::default(),
            }),
            TwitchMessage::Ready(Ready {
                name: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::Pong(Pong {
                token: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::Ping(Ping {
                token: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::IrcReady(IrcReady {
                name: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::HostTarget(HostTarget {
                raw: Cow::default(),
                hosting_channel: Cow::default(),
                host_mode: HostMode::End { viewers: 0 },
            }),
            TwitchMessage::ClearChat(ClearChat {
                raw: Cow::default(),
                channel: Cow::default(),
                target: ClearChatTarget::All,
                tags: Tags::default(),
            }),
            TwitchMessage::ClearMsg(ClearMsg {
                raw: Cow::default(),
                channel: Cow::default(),
                message: Cow::default(),
                tags: Tags::default(),
            }),
            TwitchMessage::Capability(Capability {
                acknowledged: false,
                kind: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::RoomState(RoomState {
                tags: Tags::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::Privmsg(Privmsg {
                channel: Cow::default(),
                sender: Cow::default(),
                tags: Tags::default(),
                data: Cow::default(),
                raw: Cow::default(),
            }),
            TwitchMessage::Whisper(Whisper {
                raw: Cow::default(),
                from_user: Cow::default(),
                to_user: Cow::default(),
                data: Cow::default(),
                tags: Tags::default(),
            }),
            TwitchMessage::GlobalUserState(GlobalUserState {
                tags: Tags::default(),
                raw: Cow::default(),
            }),
        ];

        let expected = expected.iter().map(std::mem::discriminant);

        for (input, discriminant) in inputs.into_iter().zip(expected) {
            let res = crate::parse(input).unwrap();
            assert!(res.remaining.is_empty());

            let e = res.message.as_enum();
            assert_eq!(std::mem::discriminant(&e), discriminant);

            let e = res.message.into_enum();
            assert_eq!(std::mem::discriminant(&e), discriminant)
        }
    }
}
