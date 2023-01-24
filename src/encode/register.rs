use super::Capability;

/// This allows you to initialize the registration handshake with the server
///
/// You must provide a name and an associated chat `OAuth` token to send messages to Twitch
/// If you just want to read messages, then the [`ANONYMOUS_LOGIN`](crate::ANONYMOUS_LOGIN) tuple is provided.
///
/// # Capabilities
/// These request Twitch enable more features for your user agent.
///
/// - To get tags, use the [`Capability::Tags`] request
/// - To see joins/parts of users, use the [`Capability::Membership`] request
/// - To send messages (PRIVMSG), use the [`Capability::Commands`] request
///
/// You can use [ALL_CAPABILITIES](crate::encode::ALL_CAPABILITIES) to request all of these
pub const fn register<'a, const N: usize>(
    name: &'a str,
    oauth: &'a str,
    caps: [Capability; N],
) -> Register<'a, N> {
    Register { name, oauth, caps }
}

/// The type produced by [`register`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Register<'a, const N: usize> {
    name: &'a str,
    oauth: &'a str,
    caps: [Capability; N],
}

impl<'a, const N: usize> Register<'a, N> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        for cap in self.caps {
            apply(
                writer,
                format_args!("CAP REQ {cap}\r\n", cap = cap.as_str()),
            )?;
        }
        apply(writer, format_args!("PASS {pass}\r\n", pass = self.oauth))?;
        apply(writer, format_args!("NICK {name}\r\n", name = self.name))
    }
}

#[cfg(feature = "std")]
impl<'a, const N: usize> crate::encode::io::Encodable for Register<'a, N> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}

impl<'a, const N: usize> crate::encode::fmt::Formattable for Register<'a, N> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn register() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let register = super::register("test", "password", [crate::encode::Capability::Tags]);
        register.format(&mut out).unwrap();
        assert_eq!(
            out,
            "CAP REQ twitch.tv/tags\r\nPASS password\r\nNICK test\r\n"
        );
    }

    #[test]
    #[cfg(feature = "std")]
    fn register_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let register = super::register("test", "password", [crate::encode::Capability::Tags]);
        register.encode(&mut out).unwrap();
        assert_eq!(
            out,
            b"CAP REQ twitch.tv/tags\r\nPASS password\r\nNICK test\r\n"
        );
    }
}
