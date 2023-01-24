use super::octo;

/// Join a channel
///
/// This prepends a '#' if you forget to
pub const fn join(channel: &str) -> Join<'_> {
    Join { channel }
}

/// The type produced by [`join`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Join<'a> {
    channel: &'a str,
}

impl<'a> Join<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(
            writer,
            format_args!(
                "JOIN {octo}{channel}\r\n",
                octo = octo(self.channel),
                channel = self.channel
            ),
        )
    }
}
#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Join<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}
impl<'a> crate::encode::fmt::Formattable for Join<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn join() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let join = super::join("museun");
        join.format(&mut out).unwrap();
        assert_eq!(out, "JOIN #museun\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn join_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let join = super::join("museun");
        join.encode(&mut out).unwrap();
        assert_eq!(out, b"JOIN #museun\r\n");
    }
}
