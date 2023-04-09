use std::borrow::Cow;

use crate::{
    encode::octo,
    messages::{MessageKind, Privmsg, TwitchMessage},
    typed_messages::TypedMessageMarker,
    Error, IntoStatic, Parse, Prefix, Tags,
};

/// A twitch chat message.
///
/// See [`parse`](fn@crate::parse)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Message<'a> {
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// Metadata embedded in the message
    pub tags: Tags<'a>,
    /// IRC-styled prefix
    pub prefix: Prefix<'a>,
    /// The kind of message
    pub kind: MessageKind<'a>,
    /// Arguments for the message kind
    pub args: Vec<Cow<'a, str>>,
    /// Data attached to the message
    pub data: Option<Cow<'a, str>>,
}

impl<'a> From<Cow<'a, Message<'a>>> for Message<'static> {
    fn from(value: Cow<'a, Message<'a>>) -> Self {
        match value {
            Cow::Borrowed(msg) => msg.into_static(),
            Cow::Owned(msg) => msg.into_static(),
        }
    }
}

impl<'a> Message<'a>
where
    'static: 'a,
{
    /// Convert this message to a typed message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_message::messages::*;
    /// let data = ":museun!museun@museun PRIVMSG #museun :hello world\r\n";
    /// let message = twitch_message::parse(data)?.message;
    ///
    /// let privmsg = message.as_typed_message::<Privmsg>().expect("invalid message");
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    pub fn as_typed_message<T>(&self) -> Option<T>
    where
        T: TypedMessageMarker<'a>,
    {
        if !T::is_kind(&self.kind) {
            return None;
        }

        T::try_from(self).ok()
    }

    /// Get this type as an enumeration of all possible types
    pub fn as_enum(&self) -> TwitchMessage<'a> {
        fn convert<'a, T>(msg: Message<'a>) -> TwitchMessage<'a>
        where
            T: TypedMessageMarker<'a>,
            TwitchMessage<'a>: From<T>,
            T: 'a,
        {
            match msg.clone().as_typed_message::<T>() {
                Some(sub) => sub.into(),
                None => TwitchMessage::Message(msg.clone()),
            }
        }

        let this = self.clone();
        use {crate::messages::*, MessageKind as K};
        #[allow(deprecated)]
        match self.kind {
            K::Capability => convert::<Capability<'a>>(this),
            K::Ping => convert::<Ping<'a>>(this),
            K::Pong => convert::<Pong<'a>>(this),
            K::IrcReady => convert::<IrcReady<'a>>(this),
            K::Ready => convert::<Ready<'a>>(this),
            K::GlobalUserState => convert::<GlobalUserState<'a>>(this),
            K::UserState => convert::<UserState<'a>>(this),
            K::RoomState => convert::<RoomState<'a>>(this),
            K::Privmsg => convert::<Privmsg<'a>>(this),
            K::Join => convert::<Join<'a>>(this),
            K::Part => convert::<Part<'a>>(this),
            K::ClearChat => convert::<ClearChat<'a>>(this),
            K::ClearMsg => convert::<ClearMsg<'a>>(this),
            K::Notice => convert::<Notice<'a>>(this),
            K::HostTarget => convert::<HostTarget<'a>>(this),
            K::UserNotice => convert::<UserNotice<'a>>(this),
            K::Whisper => convert::<Whisper<'a>>(this),
            K::Reconnect => convert::<Reconnect<'a>>(this),
            _ => TwitchMessage::Message(self.clone()),
        }
    }
}

impl Message<'static> {
    /// Convert this message into an owned typed message.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twitch_message::messages::*;
    /// let data = ":museun!museun@museun PRIVMSG #museun :hello world\r\n";
    /// let message = twitch_message::parse(data)?.message;
    ///
    /// let privmsg = message.into_typed_message::<Privmsg>().expect("invalid message");
    /// # Ok::<(),Box<dyn std::error::Error>>(())
    /// ```
    #[allow(clippy::result_large_err)]
    pub fn into_typed_message<T>(self) -> Result<<T as IntoStatic>::Output, Message<'static>>
    where
        Self: 'static,
        T: TypedMessageMarker<'static>,
        <T as TryFrom<Self>>::Error: Into<Self>,
    {
        if !T::is_kind(&self.kind) {
            return Err(self);
        }

        T::try_from(self)
            .map(crate::IntoStatic::into_static)
            .map_err(Into::into)
    }

    /// Convert this type into an enumeration of all possible types
    pub fn into_enum(self) -> TwitchMessage<'static> {
        fn convert<T>(msg: Message<'static>) -> TwitchMessage<'static>
        where
            Message<'static>: 'static,
            T: TypedMessageMarker<'static>,
            <T as TryFrom<Message<'static>>>::Error: Into<Message<'static>>,
            TwitchMessage<'static>: From<<T as IntoStatic>::Output>,
        {
            match msg.into_typed_message::<T>() {
                Ok(sub) => sub.into(),
                Err(sub) => sub.into(),
            }
        }

        use {crate::messages::*, MessageKind as K};
        #[allow(deprecated)]
        match self.kind {
            K::Capability => convert::<Capability>(self),
            K::Ping => convert::<Ping>(self),
            K::Pong => convert::<Pong>(self),
            K::IrcReady => convert::<IrcReady>(self),
            K::Ready => convert::<Ready>(self),
            K::GlobalUserState => convert::<GlobalUserState>(self),
            K::UserState => convert::<UserState>(self),
            K::RoomState => convert::<RoomState>(self),
            K::Privmsg => convert::<Privmsg>(self),
            K::Part => convert::<Part>(self),
            K::Join => convert::<Join>(self),
            K::ClearChat => convert::<ClearChat>(self),
            K::ClearMsg => convert::<ClearMsg>(self),
            K::Notice => convert::<Notice>(self),
            K::HostTarget => convert::<HostTarget>(self),
            K::UserNotice => convert::<UserNotice>(self),
            K::Whisper => convert::<Whisper>(self),
            K::Reconnect => convert::<Reconnect>(self),
            _ => self.into(),
        }
    }
}

impl<'a> Parse<'a> for Message<'a> {
    type Output = Result<Self, Error>;

    fn parse(input: &mut &'a str) -> Self::Output {
        fn parse_args<'a>(input: &mut &'a str) -> Vec<Cow<'a, str>> {
            if let Some(tail) = input.strip_prefix(':') {
                *input = tail;
                return vec![];
            }

            if let Some(end) = input.find(" :") {
                let args = input[..end]
                    .split_ascii_whitespace()
                    .map(Cow::from)
                    .collect();
                *input = &input[end + 2..];
                return args;
            }

            let args = vec![Cow::from(*input)];
            *input = "";
            args
        }

        fn parse_data<'a>(input: &mut &'a str) -> Option<Cow<'a, str>> {
            (!input.is_empty()).then(|| Cow::from(std::mem::take(input)))
        }

        Ok(Self {
            raw: Cow::from(&**input),
            tags: Tags::parse(input).unwrap_or_default(),
            prefix: Prefix::parse(input),
            kind: MessageKind::parse(input)?,
            args: parse_args(input),
            data: parse_data(input),
        })
    }
}

/// Errors for [`PrivmsgBuilder::finish_privmsg`] and [`PrivmsgBuilder::finish_message`]
#[derive(Debug)]
#[non_exhaustive]
pub enum PrivmsgBuilderError {
    /// No sender specified
    MissingSender,
    /// No channel specified
    MissingChannel,
    /// Missing data
    MissingData,
}

impl std::fmt::Display for PrivmsgBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSender => f.write_str("Missing sender"),
            Self::MissingChannel => f.write_str("Missing channel"),
            Self::MissingData => f.write_str("Missing data"),
        }
    }
}

impl std::error::Error for PrivmsgBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

/// A builder for constructing a [`Message`](crate::messages::Message) or [`Privmsg`](crate::messages::Privmsg) (and its `raw` format)
///
/// ```rust
/// use twitch_message::builders::PrivmsgBuilder;
/// use twitch_message::messages::{Privmsg, Message, MessageKind};
///
/// let builder = PrivmsgBuilder::default()
///     .channel("museun")
///     .sender("shaken_bot")
///     .data("~ Kappa");
///
/// let expected = ":shaken_bot!shaken_bot@shaken_bot.tmi.twitch.tv PRIVMSG #museun :~ Kappa\r\n";
///
/// let msg: Message<'_> = builder.clone().finish_message()?;
/// assert_eq!(msg.raw, expected);
/// assert_eq!(msg.kind, MessageKind::Privmsg);
///
/// let pm: Privmsg<'_> = builder.finish_privmsg()?;
/// assert_eq!(pm.raw, expected);
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
#[derive(Default, Clone)]
pub struct PrivmsgBuilder {
    tags: Option<Tags<'static>>,
    sender: Option<Cow<'static, str>>,
    channel: Option<Cow<'static, str>>,
    data: Option<Cow<'static, str>>,
}

impl PrivmsgBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Give it some *optional* tags
    pub fn tags(mut self, tags: Tags<'_>) -> Self {
        self.tags.replace(tags.into_static());
        self
    }

    /// Give it the *required* sender
    pub fn sender(mut self, sender: &str) -> Self {
        self.sender.replace(sender.into_static());
        self
    }

    /// Give it the *required* channel
    pub fn channel(mut self, channel: &str) -> Self {
        self.channel.replace(Cow::from(channel.to_string()));
        self
    }

    /// Give it the *required* data
    pub fn data(mut self, data: &str) -> Self {
        self.data.replace(Cow::from(data.to_string()));
        self
    }

    /// Construct a [`Privmsg`](crate::messages::Privmsg) from this builder
    pub fn finish_privmsg(self) -> Result<Privmsg<'static>, PrivmsgBuilderError> {
        Ok(self
            .finish_message()?
            .into_typed_message::<Privmsg>()
            .unwrap())
    }

    /// Construct a [`Message`](crate::messages::Message) from this builder
    pub fn finish_message(self) -> Result<Message<'static>, PrivmsgBuilderError> {
        let tags = self.tags.unwrap_or_default();

        let get =
            |field: Option<Cow<'static, str>>, err| field.filter(|s| !s.is_empty()).ok_or(err);

        let prefix = get(self.sender, PrivmsgBuilderError::MissingSender)?;
        let channel = get(self.channel, PrivmsgBuilderError::MissingChannel)?;
        let data = get(self.data, PrivmsgBuilderError::MissingData)?;

        let raw = format!(
            "{tags}{space}:{prefix}!{prefix}@{prefix}.tmi.twitch.tv PRIVMSG {octo}{channel} :{data}\r\n",
            tags = tags.to_raw(),
            space = if tags.inner.is_empty() { "" } else { " " },
            prefix = prefix,
            octo = octo(&channel),
            channel = channel,
            data = data,
        );

        Ok(Message {
            raw: Cow::from(raw),
            tags,
            prefix: Prefix::User {
                name: prefix.clone(),
                user: prefix.clone(),
                host: prefix,
            },
            kind: MessageKind::Privmsg,
            args: vec![channel],
            data: Some(data),
        })
    }
}
