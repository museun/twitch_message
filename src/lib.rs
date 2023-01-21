#[cfg(not(feature = "hashbrown"))]
pub(crate) type HashMap<K, V> = std::collections::HashMap<K, V>;
#[cfg(feature = "hashbrown")]
pub(crate) type HashMap<K, V> = hashbrown::HashMap<K, V>;

mod error;
pub use error::Error;

mod prefix;
pub use prefix::Prefix;

mod tags;
pub use tags::{Tags, TagsBuilder};

mod color;
pub use color::Color;

pub mod escape;

mod message_kind;
pub use message_kind::MessageKind;

pub mod messages;

mod typed_messages;
use typed_messages::TypedMessageMarker;

mod message;
pub use message::{Message, PrivmsgBuilder, PrivmsgBuilderError};

mod parse;
use parse::Parse;
pub use parse::{parse, parse_many, ParseResult};

mod into_static;
pub use into_static::IntoStatic;

pub mod owned;

pub mod encode;

mod badges;
pub use badges::{parse_badges, Badge, BadgeInfo};

mod emotes;
pub use emotes::{parse_emotes, Emote};

#[cfg(feature = "ping")]
mod ping_tracker;
#[cfg(feature = "ping")]
pub use ping_tracker::PingTracker;

mod lock;

pub const TWITCH_IRC_ADDRESS: &str = "irc.chat.twitch.tv:6667";
pub const TWITCH_IRC_ADDRESS_TLS: &str = "irc.chat.twitch.tv:6697";

pub const TWITCH_WS_ADDRESS: &str = "ws://irc-ws.chat.twitch.tv:80";
pub const TWITCH_WS_ADDRESS_TLS: &str = "wss://irc-ws.chat.twitch.tv:443";

pub const TWITCH_TLS_DOMAIN: &str = "irc.chat.twitch.tv";

pub const ANONYMOUS_LOGIN: (&str, &str) = (JUSTINFAN1234, JUSTINFAN1234);
pub(crate) const JUSTINFAN1234: &str = "justinfan1234";

// for unit testing
#[cfg(test)]
mod test_util;
