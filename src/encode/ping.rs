/// Constructs a PING the server should reply to
pub const fn ping(token: &str) -> Ping<'_> {
    Ping { token }
}

/// The type produced by [`ping`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Ping<'a> {
    token: &'a str,
}

impl<'a> Ping<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(writer, format_args!("PING {token}\r\n", token = self.token))
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Ping<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}
impl<'a> crate::encode::fmt::Formattable for Ping<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ping() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let ping = super::ping("1234567890");
        ping.format(&mut out).unwrap();
        assert_eq!(out, "PING 1234567890\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn ping_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let ping = super::ping("1234567890");
        ping.encode(&mut out).unwrap();
        assert_eq!(out, b"PING 1234567890\r\n");
    }
}
