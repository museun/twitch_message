use std::borrow::Cow;

use crate::{messages::*, Badge, HashMap, Prefix, Tags};

/// A trait for converting a T: 'a to a T: 'static
///
/// This is used for going from a [`Message<'a>`](crate::messages::Message) to a `Message<'static>` or any of the sub types (see [`messages`](crate::messages))
pub trait IntoStatic {
    /// Output of [`IntoStatic::into_static`].
    ///
    /// This is an alternative to `ToOwned::Owned`
    type Output: 'static;

    /// This method casts `self` between `Self<'a>` and `Self<'static>`.
    fn into_static(self) -> Self::Output;
}

mod private {
    pub trait Sealed {}
    impl<T> Sealed for T where T: super::IntoStatic {}
}

impl<'a> IntoStatic for Cow<'a, str> {
    type Output = Cow<'static, str>;

    fn into_static(self) -> Self::Output {
        match self {
            Cow::Borrowed(s) => Cow::Owned(s.to_string()),
            Cow::Owned(s) => Cow::Owned(s),
        }
    }
}

impl<'a, T> IntoStatic for &'a T
where
    T: IntoStatic,
    T: Clone,
{
    type Output = T::Output;

    fn into_static(self) -> Self::Output {
        self.clone().into_static()
    }
}

impl IntoStatic for &str {
    type Output = Cow<'static, str>;

    fn into_static(self) -> Self::Output {
        self.to_string().into()
    }
}

impl IntoStatic for &UserIdRef {
    type Output = Cow<'static, UserIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<'a> IntoStatic for Cow<'a, UserIdRef> {
    type Output = Cow<'static, UserIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl IntoStatic for &UserNameRef {
    type Output = Cow<'static, UserNameRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<'a> IntoStatic for Cow<'a, UserNameRef> {
    type Output = Cow<'static, UserNameRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl IntoStatic for &MsgIdRef {
    type Output = Cow<'static, MsgIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<'a> IntoStatic for Cow<'a, MsgIdRef> {
    type Output = Cow<'static, MsgIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<'a> IntoStatic for Cow<'a, BadgeSetIdRef> {
    type Output = Cow<'static, BadgeSetIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<'a> IntoStatic for Cow<'a, ChatBadgeIdRef> {
    type Output = Cow<'static, ChatBadgeIdRef>;

    fn into_static(self) -> Self::Output {
        IntoCow::into_cow(self.to_string())
    }
}

impl<T> IntoStatic for Vec<T>
where
    T: IntoStatic,
{
    type Output = Vec<T::Output>;

    fn into_static(self) -> Self::Output {
        self.into_iter().map(IntoStatic::into_static).collect()
    }
}

impl<K, V> IntoStatic for crate::HashMap<K, V>
where
    K: IntoStatic,
    V: IntoStatic,
    K::Output: Eq + std::hash::Hash,
{
    type Output = HashMap<K::Output, V::Output>;

    fn into_static(self) -> Self::Output {
        self.into_iter()
            .map(|(k, v)| (k.into_static(), v.into_static()))
            .collect()
    }
}

impl<T> IntoStatic for Option<T>
where
    T: IntoStatic,
{
    type Output = Option<T::Output>;

    fn into_static(self) -> Self::Output {
        self.map(IntoStatic::into_static)
    }
}

impl<'a> IntoStatic for TwitchMessage<'a>
where
    'static: 'a,
{
    type Output = TwitchMessage<'static>;

    #[allow(deprecated)]
    fn into_static(self) -> Self::Output {
        match self {
            Self::UserState(msg) => TwitchMessage::UserState(msg.into_static()),
            Self::UserNotice(msg) => TwitchMessage::UserNotice(msg.into_static()),
            Self::Reconnect(msg) => TwitchMessage::Reconnect(msg.into_static()),
            Self::HostTarget(msg) => TwitchMessage::HostTarget(msg.into_static()),
            Self::RoomState(msg) => TwitchMessage::RoomState(msg.into_static()),
            Self::IrcReady(msg) => TwitchMessage::IrcReady(msg.into_static()),
            Self::Privmsg(msg) => TwitchMessage::Privmsg(msg.into_static()),
            Self::Whisper(msg) => TwitchMessage::Whisper(msg.into_static()),
            Self::Notice(msg) => TwitchMessage::Notice(msg.into_static()),
            Self::Ping(msg) => TwitchMessage::Ping(msg.into_static()),
            Self::Pong(msg) => TwitchMessage::Pong(msg.into_static()),
            Self::Ready(msg) => TwitchMessage::Ready(msg.into_static()),
            Self::GlobalUserState(msg) => TwitchMessage::GlobalUserState(msg.into_static()),
            Self::Join(msg) => TwitchMessage::Join(msg.into_static()),
            Self::Part(msg) => TwitchMessage::Part(msg.into_static()),
            Self::ClearMsg(msg) => TwitchMessage::ClearMsg(msg.into_static()),
            Self::Capability(msg) => TwitchMessage::Capability(msg.into_static()),
            Self::ClearChat(msg) => TwitchMessage::ClearChat(msg.into_static()),
            Self::Message(msg) => TwitchMessage::Message(msg.into_static()),
        }
    }
}

impl<'a> IntoStatic for Message<'a>
where
    'static: 'a,
{
    type Output = Message<'static>;

    fn into_static(self) -> Self::Output {
        Message {
            raw: self.raw.into_static(),
            tags: self.tags.into_static(),
            prefix: self.prefix.into_static(),
            kind: self.kind.into_static(),
            args: self.args.into_static(),
            data: self.data.into_static(),
        }
    }
}

impl<'a> IntoStatic for Tags<'a>
where
    'static: 'a,
{
    type Output = Tags<'static>;

    fn into_static(self) -> Self::Output {
        Tags {
            inner: self.inner.into_static(),
        }
    }
}

impl<'a> IntoStatic for Prefix<'a>
where
    'static: 'a,
{
    type Output = Prefix<'static>;

    fn into_static(self) -> Self::Output {
        match self {
            Self::User { name, user, host } => Prefix::User {
                name: name.into_static(),
                user: user.into_static(),
                host: host.into_static(),
            },
            Self::Server { host } => Prefix::Server {
                host: host.into_static(),
            },
            Self::None => Prefix::None,
        }
    }
}

impl<'a> IntoStatic for MessageKind<'a>
where
    'static: 'a,
{
    type Output = MessageKind<'static>;

    fn into_static(self) -> Self::Output {
        match self {
            Self::Capability => MessageKind::Capability,
            Self::Ping => MessageKind::Ping,
            Self::Pong => MessageKind::Pong,
            Self::IrcReady => MessageKind::IrcReady,
            Self::Ready => MessageKind::Ready,
            Self::GlobalUserState => MessageKind::GlobalUserState,
            Self::UserState => MessageKind::UserState,
            Self::RoomState => MessageKind::RoomState,
            Self::Privmsg => MessageKind::Privmsg,
            Self::Join => MessageKind::Join,
            Self::Part => MessageKind::Part,
            Self::ClearChat => MessageKind::ClearChat,
            Self::ClearMsg => MessageKind::ClearMsg,
            Self::Notice => MessageKind::Notice,
            Self::HostTarget => MessageKind::HostTarget,
            Self::UserNotice => MessageKind::UserNotice,
            Self::Whisper => MessageKind::Whisper,
            Self::Reconnect => MessageKind::Reconnect,
            Self::Numeric(n) => MessageKind::Numeric(n),
            Self::Unknown(s) => MessageKind::Unknown(IntoStatic::into_static(s)),
        }
    }
}

impl<'a> IntoStatic for Capability<'a>
where
    'static: 'a,
{
    type Output = Capability<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            acknowledged: self.acknowledged,
            kind: self.kind.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

impl<'a> IntoStatic for ClearChat<'a>
where
    'static: 'a,
{
    type Output = ClearChat<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            channel: self.channel.into_static(),
            target: self.target.into_static(),
            tags: self.tags.into_static(),
        }
    }
}

impl<'a> IntoStatic for ClearChatTarget<'a>
where
    'static: 'a,
{
    type Output = ClearChatTarget<'static>;
    fn into_static(self) -> Self::Output {
        match self {
            ClearChatTarget::All => Self::Output::All,
            ClearChatTarget::User(user) => Self::Output::User(user.into_static()),
        }
    }
}

impl<'a> IntoStatic for ClearMsg<'a>
where
    'static: 'a,
{
    type Output = ClearMsg<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            channel: self.channel.into_static(),
            message: self.message.into_static(),
            tags: self.tags.into_static(),
        }
    }
}

impl<'a> IntoStatic for GlobalUserState<'a>
where
    'static: 'a,
{
    type Output = GlobalUserState<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            tags: self.tags.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

#[allow(deprecated)]
impl<'a> IntoStatic for HostTarget<'a>
where
    'static: 'a,
{
    type Output = HostTarget<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            hosting_channel: self.hosting_channel.into_static(),
            host_mode: self.host_mode.into_static(),
        }
    }
}

#[allow(deprecated)]
impl IntoStatic for HostMode {
    type Output = Self;
    fn into_static(self) -> Self::Output {
        self
    }
}

impl<'a> IntoStatic for IrcReady<'a>
where
    'static: 'a,
{
    type Output = IrcReady<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            name: self.name.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

#[allow(deprecated)]
impl<'a> IntoStatic for Notice<'a>
where
    'static: 'a,
{
    type Output = Notice<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            channel: self.channel.into_static(),
            message: self.message.into_static(),
            tags: self.tags.into_static(),
        }
    }
}

impl<'a> IntoStatic for Ping<'a>
where
    'static: 'a,
{
    type Output = Ping<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            token: self.token.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

impl<'a> IntoStatic for Pong<'a>
where
    'static: 'a,
{
    type Output = Pong<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            token: self.token.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

impl<'a> IntoStatic for Privmsg<'a>
where
    'static: 'a,
{
    type Output = Privmsg<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            channel: self.channel.into_static(),
            sender: self.sender.into_static(),
            tags: self.tags.into_static(),
            data: self.data.into_static(),
            raw: self.raw.into_static(),
            action: self.action,
        }
    }
}

impl<'a> IntoStatic for Ready<'a>
where
    'static: 'a,
{
    type Output = Ready<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            name: self.name.into_static(),
            raw: self.raw.into_static(),
        }
    }
}

impl<'a> IntoStatic for Reconnect<'a>
where
    'static: 'a,
{
    type Output = Reconnect<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
        }
    }
}

impl<'a> IntoStatic for RoomState<'a>
where
    'static: 'a,
{
    type Output = RoomState<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            tags: self.tags.into_static(),
            raw: self.raw.into_static(),
            channel: self.channel.into_static(),
        }
    }
}

impl<'a> IntoStatic for UserState<'a>
where
    'static: 'a,
{
    type Output = UserState<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            tags: self.tags.into_static(),
            raw: self.raw.into_static(),
            channel: self.channel.into_static(),
        }
    }
}

impl<'a> IntoStatic for UserNotice<'a>
where
    'static: 'a,
{
    type Output = UserNotice<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            tags: self.tags.into_static(),
            channel: self.channel.into_static(),
            data: self.data.into_static(),
        }
    }
}

impl<'a> IntoStatic for Whisper<'a>
where
    'static: 'a,
{
    type Output = Whisper<'static>;
    fn into_static(self) -> Self::Output {
        Self::Output {
            raw: self.raw.into_static(),
            from_user: self.from_user.into_static(),
            to_user: self.to_user.into_static(),
            data: self.data.into_static(),
            tags: self.tags.into_static(),
        }
    }
}

impl<'a> IntoStatic for Join<'a> {
    type Output = Join<'static>;

    fn into_static(self) -> Self::Output {
        Join {
            user: self.user.into_static(),
            channel: self.channel.into_static(),
        }
    }
}

impl<'a> IntoStatic for Part<'a> {
    type Output = Part<'static>;

    fn into_static(self) -> Self::Output {
        Part {
            user: self.user.into_static(),
            channel: self.channel.into_static(),
        }
    }
}

impl<'a> IntoStatic for Badge<'a> {
    type Output = Badge<'static>;

    fn into_static(self) -> Self::Output {
        Badge {
            name: self.name.into_static(),
            version: self.version.into_static(),
        }
    }
}
