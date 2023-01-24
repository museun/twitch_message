use super::octo;

/// Sends a message to a channel
pub const fn privmsg<'a>(channel: &'a str, data: &'a str) -> Privmsg<'a> {
    Privmsg {
        channel,
        data,
        reply_id: None,
    }
}

/// Sends a message to a channel, with a provided `reply-parent-msg-id` attached
pub const fn reply<'a>(id: &'a str, channel: &'a str, data: &'a str) -> Privmsg<'a> {
    Privmsg {
        reply_id: Some(id),
        channel,
        data,
    }
}

/// The type produced by [`privmsg`] or [`reply`]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Privmsg<'a> {
    reply_id: Option<&'a str>,
    channel: &'a str,
    data: &'a str,
}

impl<'a> Privmsg<'a> {
    // TODO split this into chunks
    fn fmt<W, E>(
        &self,
        writer: &mut W,
        apply: fn(&mut W, core::fmt::Arguments<'_>) -> Result<(), E>,
    ) -> Result<(), E> {
        if let Some(id) = self.reply_id {
            apply(writer, format_args!("@reply-parent-msg-id={id} "))?;
        }
        apply(
            writer,
            format_args!(
                "PRIVMSG {octo}{channel} :{data}\r\n",
                octo = octo(self.channel),
                channel = self.channel,
                data = self.data
            ),
        )
    }
}

#[cfg(feature = "std")]
impl<'a> crate::encode::io::Encodable for Privmsg<'a> {
    fn encode(&self, mut writer: impl std::io::Write) -> std::io::Result<()> {
        self.fmt(&mut writer, std::io::Write::write_fmt)
    }
}

impl<'a> crate::encode::fmt::Formattable for Privmsg<'a> {
    fn format(&self, mut writer: impl core::fmt::Write) -> core::fmt::Result {
        self.fmt(&mut writer, core::fmt::Write::write_fmt)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn privmsg() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let privmsg = super::privmsg("test", "hello, world");
        privmsg.format(&mut out).unwrap();
        assert_eq!(out, "PRIVMSG #test :hello, world\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn privmsg_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let privmsg = super::privmsg("test", "hello, world");
        privmsg.encode(&mut out).unwrap();
        assert_eq!(out, b"PRIVMSG #test :hello, world\r\n");
    }

    #[test]
    fn reply() {
        use crate::encode::Formattable;

        let mut out = String::new();
        let reply = super::reply("123456", "test", "Kappa");
        reply.format(&mut out).unwrap();
        assert_eq!(out, "@reply-parent-msg-id=123456 PRIVMSG #test :Kappa\r\n");
    }

    #[test]
    #[cfg(feature = "std")]
    fn reply_std() {
        use crate::encode::Encodable;

        let mut out = vec![];
        let reply = super::reply("123456", "test", "Kappa");
        reply.encode(&mut out).unwrap();
        assert_eq!(out, b"@reply-parent-msg-id=123456 PRIVMSG #test :Kappa\r\n");
    }
}
