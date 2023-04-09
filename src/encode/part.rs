use super::octo;

/// Leave a channel
///
/// This prepends a '#' if you forget to
pub const fn part(channel: &str) -> Part<'_> {
    Part { channel }
}

/// The type produced by [`part`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Part<'a> {
    channel: &'a str,
}

impl<'a> Part<'a> {
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        apply(
            writer,
            format_args!(
                "PART {octo}{channel}\r\n",
                octo = octo(self.channel),
                channel = self.channel
            ),
        )
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Part<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}

impl<'a> crate::encode::fmt::Formattable for Part<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

impl<'a> std::fmt::Display for Part<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn part() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let part = super::part("museun");
        part.format(&mut out).unwrap();
        assert_eq!(out, "PART #museun\r\n");
    }

    #[test]
    fn part_display() {
        let raw = super::part("museun");
        assert_eq!(raw.to_string(), "PART #museun\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn part_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let part = super::part("museun");
        part.encode(&mut out).unwrap();
        assert_eq!(out, b"PART #museun\r\n");
    }
}
