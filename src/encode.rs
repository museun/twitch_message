//! This module provides encoding of typed messages
//!
//! Without the `std` feature enabled, you'll only be able to encode to a [`core::fmt::Write`] source (e.g. a [`String`])
//!
//! If you enable the `std` feature, then you'll also be able to encode to a [`std::io::Write`] source (e.g. a [`Vec<u8>`] or a [`std::net::TcpStream`])
//!
//! # Provided traits:
//!
//! | Trait | Description | Feature | Example |
//! | --- | --- | --- | --- |
//! | [`Format`] | Format this message to the [`core::fmt::Write`] | -- | `ping("asdf").format(&mut buf)` |
//! | [`Formattable`] | Using [`core::fmt::Write`] format this message  | -- | `buf.format_msg(ping("asdf"))` |
//! | [`Encode`] | Encode this message to the [`std::io::Write`] | `std` | `ping("asdf").encode(&mut buf)` |
//! | [`Encodable`] | Using [`std::io::Write`] type encode this message | `std` | `buf.encode_msg(ping("asdf"))` |
//!
//! Using one of the [functions](#functions) creates one of the [types](#structs).
//!
//! These don't allocate directly and when using with `&'static str` can be stored in `static`/`const` contexts.
//!
//! # Example
//!
//! ```rust
//! use twitch_message::encode::{privmsg, Privmsg, Format, Formattable};
//!
//! const KAPPA: Privmsg<'static> = privmsg("museun", "Kappa");
//!
//! let vohiyo = privmsg("museun", "VoHiYo");
//!
//! let mut buf = String::new();
//! KAPPA.format(&mut buf)?;
//! buf.format_msg(vohiyo)?;
//!
//! assert_eq!(buf, "PRIVMSG #museun :Kappa\r\nPRIVMSG #museun :VoHiYo\r\n");
//! # Ok::<(),Box<dyn std::error::Error>>(())
//! ```

#[cfg(feature = "std")]
mod io;
#[cfg(feature = "std")]
pub use io::{Encodable, Encode};

mod fmt;
pub use fmt::{Format, Formattable};

mod capability;
pub use capability::{Capability, ALL_CAPABILITIES};

mod join;
pub use join::{join, Join};

mod part;
pub use part::{part, Part};

mod privmsg;
pub use privmsg::{privmsg, reply, Privmsg};

mod register;
pub use register::{register, Register};

mod ping;
pub use ping::{ping, Ping};

mod pong;
pub use pong::{pong, Pong};

mod raw;
pub use raw::{raw, Raw};

#[inline]
pub(crate) fn octo(data: &str) -> &str {
    if !data.starts_with('#') {
        "#"
    } else {
        ""
    }
}
