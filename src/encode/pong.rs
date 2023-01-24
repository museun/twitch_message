/// Construct a PONG message, these are used to respond to a Ping
///
/// A Ping message gives you a token which you should provide to this.
pub const fn pong(token: &str) -> Pong<'_> {
    Pong { token }
}

/// The type produced by [`pong`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pong<'a> {
    token: &'a str,
}

impl<'a> Pong<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(
            writer,
            format_args!("PONG :{token}\r\n", token = self.token),
        )
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Pong<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}
impl<'a> crate::encode::fmt::Formattable for Pong<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn pong() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let pong = super::pong("1234567890");
        pong.format(&mut out).unwrap();
        assert_eq!(out, "PONG :1234567890\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn pong_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let pong = super::pong("1234567890");
        pong.encode(&mut out).unwrap();
        assert_eq!(out, b"PONG :1234567890\r\n");
    }
}
