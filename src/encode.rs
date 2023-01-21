#[cfg(feature = "std")]
mod io;
#[cfg(feature = "std")]
pub use io::{Encodable, Encode};

mod fmt;
pub use fmt::{Format, Formattable};

mod capability;
pub use capability::Capability;

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
fn octo(data: &str) -> &str {
    if !data.starts_with('#') {
        "#"
    } else {
        ""
    }
}
