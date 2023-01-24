//! This is a crate to parse chat messages from [https://www.twitch.tv](https://www.twitch.tv)
//!
//! This crate does not provide any I/O rather just parsing of a `&str` into typed messages.
//!
//! A quick walkthrough:
//! ```rust
//! # fn read_line() -> &'static str { ":museun!museun@museun PRIVMSG #museun :hello world\r\n"}
//! use twitch_message::messages::*;
//! // get some data from somewhere
//! let data: &str = read_line();
//!
//! // parse returns a `ParseResult` which contains the remaining data (if any) and the parsed message
//! let result = twitch_message::parse(data).unwrap();
//! let msg: Message<'_> = result.message;
//!
//! match msg.kind {
//!     MessageKind::Ready => {
//!         let ready = msg.as_typed_message::<Ready>().unwrap();
//!         println!("connected as: {name}", name = ready.name);
//!     }
//!     MessageKind::Privmsg => {
//!         let pm = msg.as_typed_message::<Privmsg>().unwrap();
//!         println!("[{channel}] {sender}: {data}",
//!             channel = pm.channel,
//!             sender = pm.sender,
//!             data = pm.data
//!         );
//!     }
//!     MessageKind::Ping => {
//!         let ping = msg.as_typed_message::<Ping>().unwrap();
//!         let resp = twitch_message::encode::pong(&ping.token);
//!
//!         // you can format data to various 'sinks'
//!         use twitch_message::encode::Formattable;
//!         let mut out = String::new();
//!         resp.format(&mut out).unwrap();
//!         assert_eq!(out, "PONG :1234567890\r\n");
//!     }
//!     _ => {}
//! }
//! ```
//!
//! # Parsing
//! ### There are various *parse* methods provided by this crate:
//! - [`parse`](fn@parse)
//!
//! This will parse a single message, returning any remaining data
//!
//! - [`parse_many`]
//!
//! This will return an iterator over possibly many messages in the data
//!
//! - [`parse_as`]
//!
//! This is a shorthand for [`parse`](fn@parse) + [`Message::as_typed_message()`](crate::messages::Message::as_typed_message())
//!
//! - [`parse_badges`]
//!
//! This allows you to parse ***badges*** from a string
//!
//! - [`parse_emotes`]
//!
//! This allows you to parse ***emotes*** from a Twitch emote string + the associated data portion
//!
//! ---
//!
//! # Typed messages
//! Once you parse data into a [`Message`](crate::messages::Message), you can further narrow it to a specific type via two methods:
//! - [`Message::as_typed_message()`](crate::messages::Message::as_typed_message())
//! - [`Message::into_typed_message()`](crate::messages::Message::into_typed_message())
//!
//! `as_typed_message` will borrow from the message, forgoing any further allocations.
//!
//! `into_typed_message` will clone the data so you'll have an owned (`'static`) version of the message
//!
//! The argument (`type`) used for these are one of the main structs found in the [`messages`] module.
//!
//! # Ownership
//! If you have a `Message<'a>` or some sub type (found in [`messages`]) and want it to be `'static`, a trait is provided:
//!
//! [`IntoStatic`]
//!
//! This trait is implemented for all of the types. Once you import it, you can do `ty.into_static()` to get a `'static` version of the type.
//!
//! Why this trait instead of [`std::borrow::ToOwned`]? This trait allows more specific lifetime clauses and doesn't require `T: Clone`. But in general, its basically used the same way.
//!
//! # Builders
//! A few builders are provided:
//! - [`PrivmsgBuilder`](crate::builders::PrivmsgBuilder)
//! - [`TagsBuilder`](crate::builders::TagsBuilder)
//!
//! These allow you to construct messages for testing, or for custom purposes (mocking/faking, etc)
//!
//! # Encoding
//! The [`encode`] module provides a typed way of constructing messages to send to Twitch.
//!
//! By default, only encoding to a [`core::fmt::Write`] source (e.g. a `String`) is supported, via the [`Format`](crate::encode::Format) and [`Formattable`](crate::encode::Formattable) traits.
//!
//! If you enable the `std` feature (see [features](#features)), you will have access to the [`Encode`](crate::encode::Encode) and [`Encodable`](crate::encode::Encodable) traits which operate on a [`std::io::Write`] source. (e.g. a [`Vec<u8>`] or [`std::net::TcpStream`])
//!
//! ### Example
//! #### Format/Formattable
//! ```rust
//! // this adds the # to the channel, if its missing
//! let pm = twitch_message::encode::privmsg("museun", "hello, world.");
//!
//! // using `Formattable`
//! use twitch_message::encode::Formattable;
//! let mut buf = String::new();
//! pm.format(&mut buf).unwrap();
//! assert_eq!(buf, "PRIVMSG #museun :hello, world.\r\n");
//!
//! // using `Format`
//! use twitch_message::encode::Format;
//! let mut buf = String::new();
//! buf.format_msg(pm);
//! assert_eq!(buf, "PRIVMSG #museun :hello, world.\r\n");
//! ```
//!
//! #### Encode/Encodable
//! ```rust,ignore
//! // this adds the # to the channel, if its missing
//! let pm = twitch_message::encode::privmsg("museun", "hello, world.");
//!
//! // using `Encodable`
//! use twitch_message::encode::Encodable;
//! let mut buf = Vec::new();
//! pm.encode(&mut buf).unwrap();
//! assert_eq!(buf, b"PRIVMSG #museun :hello, world.\r\n");
//!
//! // using `Encode`
//! use twitch_message::encode::Encode;
//! let mut buf = Vec::new();
//! buf.encode_msg(pm);
//! assert_eq!(buf, b"PRIVMSG #museun :hello, world.\r\n");
//! ```
//!
//! # Features
//! | Feature | Description |
//! | --- | --- |
//! |default | there are no default features |
//! |ping | enables the [`PingTracker`] |
//! |std | enables the [`Encode`](crate::encode::Encode) and [`Encodable`](crate::encode::Encodable) traits |
//! |serde | enables [`serde`] derives on the types |
//! |hashbrown | enables using [`hashbrown`] for the internal `HashMap` |
//! |sync | enables using [`std::sync::Mutex`] over [`std::cell::RefCell`] see [`sharing data`](#sharing-data) |
//! |parking_lot | same as `sync` except uses a [`parking_lot::Mutex`] |
//!
//! # Utilities
//! ## PingTracker
//!
//! A `PingTracker` is provided, and is entirely optional (enabled with the `ping` feature).
//!
//! This is a simple type to help you determine when you should respond to a `PING` message.
//!
//! ## Tag (un)escaping
//! IRCv3 requires tags to be [escaped](https://ircv3.net/specs/extensions/message-tags.html#escaping-values).
//!
//! This crate provides a method to [`escape them`](crate::escape::escape_tag), and to [`unescape them`](crate::escape::unescape_tag).
//!
//! *NOTE* You don't have to worry about the escape-status of [`Tags`], interally these are used.
//!
//! These methods are provided for your own use cases.
//!
//! ---
//! Twitch chat reference: [`link`](https://dev.twitch.tv/docs/irc/)

#[cfg(not(feature = "hashbrown"))]
pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "hashbrown")]
pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V>;

mod error;
pub use error::Error;

mod prefix;
pub use prefix::Prefix;

mod tags;
pub use tags::Tags;

mod color;
pub use color::Color;

pub mod escape;

mod message_kind;

pub mod messages;

mod typed_messages;
use typed_messages::TypedMessageMarker;

mod message;

mod parse;
use parse::Parse;
pub use parse::{parse, parse_as, parse_many, ParseResult};

mod into_static;
pub use into_static::IntoStatic;

pub mod encode;

mod badges;
pub use badges::{parse_badges, Badge, BadgeInfo};

mod emotes;
pub use emotes::{parse_emotes, Emote};

pub mod builders {
    //! Builders for constructing your own types.
    pub use crate::message::{PrivmsgBuilder, PrivmsgBuilderError};
    pub use crate::tags::TagsBuilder;
}

#[cfg(feature = "ping")]
mod ping_tracker;
#[cfg(feature = "ping")]
pub use ping_tracker::PingTracker;

mod lock;

/// The Twitch IRC (tcp) address
pub const TWITCH_IRC_ADDRESS: &str = "irc.chat.twitch.tv:6667";

/// The Twitch IRC (tcp/tls) address
pub const TWITCH_IRC_ADDRESS_TLS: &str = "irc.chat.twitch.tv:6697";

/// The Twitch WebSocket address
pub const TWITCH_WS_ADDRESS: &str = "ws://irc-ws.chat.twitch.tv:80";
/// The Twitch WebSocket TLS address
pub const TWITCH_WS_ADDRESS_TLS: &str = "wss://irc-ws.chat.twitch.tv:443";

/// The TLS domain for Twitch
pub const TWITCH_TLS_DOMAIN: &str = "irc.chat.twitch.tv";

/// An anonymous login. This'll allow you to read messages without an `OAuth` token
pub const ANONYMOUS_LOGIN: (&str, &str) = (JUSTINFAN1234, JUSTINFAN1234);
pub(crate) const JUSTINFAN1234: &str = "justinfan1234";

// for unit testing
#[cfg(test)]
mod test_util;
