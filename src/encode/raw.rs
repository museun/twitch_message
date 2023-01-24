/// Send a raw message to the server
pub const fn raw(raw: &str) -> Raw<'_> {
    Raw { raw }
}

/// The type produced by [`raw`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Raw<'a> {
    raw: &'a str,
}

impl<'a> Raw<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(writer, format_args!("{raw}\r\n", raw = self.raw))
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Raw<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}

impl<'a> crate::encode::fmt::Formattable for Raw<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn raw() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let raw = super::raw("hello world");
        raw.format(&mut out).unwrap();
        assert_eq!(out, "hello world\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn raw_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let raw = super::raw("hello world");
        raw.encode(&mut out).unwrap();
        assert_eq!(out, b"hello world\r\n")
    }
}
