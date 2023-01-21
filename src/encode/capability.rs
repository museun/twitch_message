#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
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
