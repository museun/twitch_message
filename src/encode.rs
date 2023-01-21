use std::io::Write;

pub trait Encode: Write {
    fn encode_msg(&mut self, msg: impl Encodable) -> std::io::Result<()> {
        msg.encode(self)
    }
}

impl<T> Encode for T where T: Write {}

pub trait Encodable {
    fn encode(&self, writer: impl Write) -> std::io::Result<()>;
}

#[inline]
fn octo(data: &str) -> &str {
    data.starts_with('#').then_some("").unwrap_or("#")
}

pub const fn join(channel: &str) -> Join<'_> {
    Join { channel }
}

pub struct Join<'a> {
    channel: &'a str,
}

impl<'a> Encodable for Join<'a> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        write!(
            &mut writer,
            "JOIN {octo}{channel}\r\n",
            octo = octo(self.channel),
            channel = self.channel
        )
    }
}

pub const fn part(channel: &str) -> Part<'_> {
    Part { channel }
}

pub struct Part<'a> {
    channel: &'a str,
}

impl<'a> Encodable for Part<'a> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        write!(
            &mut writer,
            "PART {octo}{channel}\r\n",
            octo = octo(self.channel),
            channel = self.channel
        )
    }
}

pub const fn privmsg<'a>(channel: &'a str, data: &'a str) -> Privmsg<'a> {
    Privmsg {
        channel,
        data,
        reply_id: None,
    }
}

pub const fn reply<'a>(id: &'a str, channel: &'a str, data: &'a str) -> Privmsg<'a> {
    Privmsg {
        reply_id: Some(id),
        channel,
        data,
    }
}

pub struct Privmsg<'a> {
    reply_id: Option<&'a str>,
    channel: &'a str,
    data: &'a str,
}

impl<'a> Encodable for Privmsg<'a> {
    // TODO split this into chunks
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        if let Some(id) = self.reply_id {
            write!(&mut writer, "@reply-parent-msg-id={id} ")?;
        }
        write!(
            &mut writer,
            "PRIVMSG {octo}{channel} :{data}\r\n",
            octo = octo(self.channel),
            channel = self.channel,
            data = self.data
        )
    }
}

pub const fn register<'a, const N: usize>(
    name: &'a str,
    oauth: &'a str,
    caps: [Capability; N],
) -> Register<'a, N> {
    Register { name, oauth, caps }
}

pub struct Register<'a, const N: usize> {
    name: &'a str,
    oauth: &'a str,
    caps: [Capability; N],
}

impl<'a, const N: usize> Encodable for Register<'a, N> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        for cap in self.caps {
            write!(&mut writer, "CAP REQ {cap}\r\n", cap = cap.as_str())?;
        }
        write!(&mut writer, "PASS {pass}\r\n", pass = self.oauth)?;
        write!(&mut writer, "NICK {name}\r\n", name = self.name)
    }
}

pub const fn ping(token: &str) -> Ping<'_> {
    Ping { token }
}

pub struct Ping<'a> {
    token: &'a str,
}

impl<'a> Encodable for Ping<'a> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        write!(&mut writer, "PING {token}\r\n", token = self.token)
    }
}

pub const fn pong(token: &str) -> Pong<'_> {
    Pong { token }
}

pub struct Pong<'a> {
    token: &'a str,
}

impl<'a> Encodable for Pong<'a> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        write!(&mut writer, "PONG :{token}\r\n", token = self.token)
    }
}

pub const fn raw(raw: &str) -> Raw<'_> {
    Raw { raw }
}

pub struct Raw<'a> {
    raw: &'a str,
}

impl<'a> Encodable for Raw<'a> {
    fn encode(&self, mut writer: impl Write) -> std::io::Result<()> {
        write!(&mut writer, "{raw}\r\n", raw = self.raw)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Capability {
    Commands,
    Membership,
    Tags,
}

impl Capability {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Commands => "twitch.tv/commands",
            Self::Membership => "twitch.tv/membership",
            Self::Tags => "twitch.tv/tags",
        }
    }
}
