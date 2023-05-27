/// Sends a private message to a user
pub const fn whisper<'a>(user: &'a str, data: &'a str) -> Whisper<'a> {
    Whisper { user, data }
}

/// The type produced by [`whisper`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Whisper<'a> {
    user: &'a str,
    data: &'a str,
}

impl<'a> Whisper<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(
            writer,
            format_args!(
                "PRIVMSG jtv :/w {user} {data}\r\n",
                user = self.user,
                data = self.data
            ),
        )
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Whisper<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}

impl<'a> crate::encode::fmt::Formattable for Whisper<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

impl<'a> std::fmt::Display for Whisper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn whisper() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let whisper = super::whisper("museun", "hello, world");
        whisper.format(&mut out).unwrap();
        assert_eq!(out, "PRIVMSG jtv :/w museun hello, world\r\n");
    }

    #[test]
    fn whisper_display() {
        let raw = super::whisper("museun", "hello, world");
        assert_eq!(raw.to_string(), "PRIVMSG jtv :/w museun hello, world\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn whisper_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let whisper = super::whisper("museun", "hello, world");
        whisper.encode(&mut out).unwrap();
        assert_eq!(out, b"PRIVMSG jtv :/w museun hello, world\r\n");
    }
}
