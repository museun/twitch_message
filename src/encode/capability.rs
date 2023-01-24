/// Request a [capability](https://dev.twitch.tv/docs/irc/capabilities/) from Twitch
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum Capability {
    /// Lets your bot send PRIVMSG messages that include Twitch
    /// [chat commands](https://dev.twitch.tv/docs/irc/chat-commands/)
    /// *(note, these are deprecated)* and receive [Twitch-specific IRC messages](https://dev.twitch.tv/docs/irc/#twitch-specific-irc-messages).
    Commands,
    /// Lets your bot receive JOIN and PART messages when users join and leave the chat room.
    Membership,
    /// Adds additional metadata to the command and membership messages.
    ///
    /// For the list of metadata available with each message, see [Twitch tags](https://dev.twitch.tv/docs/irc/tags/).
    ///
    /// To request the tags capability, you must also request the commands capability.
    Tags,
}

impl Capability {
    /// Get the capability as a string
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Commands => "twitch.tv/commands",
            Self::Membership => "twitch.tv/membership",
            Self::Tags => "twitch.tv/tags",
        }
    }
}

/// All of the [`Capabilities`](Capability)
pub const ALL_CAPABILITIES: [Capability; 3] = [
    Capability::Commands,
    Capability::Membership,
    Capability::Tags,
];
