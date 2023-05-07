/// Construct a QUIT message, with a reason
///
/// This is used for asking the server to close the connection
pub const fn quit(reason: &str) -> Quit<'_> {
    Quit { reason }
}

/// The type produced by [`quit`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Quit<'a> {
    reason: &'a str,
}

impl<'a> Quit<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(
            writer,
            format_args!("QUIT :{reason}\r\n", reason = self.reason),
        )
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Quit<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}
impl<'a> crate::encode::fmt::Formattable for Quit<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

impl<'a> std::fmt::Display for Quit<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn quit() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let quit = super::quit("leaving this server");
        quit.format(&mut out).unwrap();
        assert_eq!(out, "QUIT :leaving this server\r\n");
    }

    #[test]
    fn quit_display() {
        let raw = super::quit("leaving this server");
        assert_eq!(raw.to_string(), "QUIT :leaving this server\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn quit_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let quit = super::quit("leaving this server");
        quit.encode(&mut out).unwrap();
        assert_eq!(out, b"QUIT :leaving this server\r\n");
    }
}
