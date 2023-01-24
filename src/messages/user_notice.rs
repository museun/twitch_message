#![allow(deprecated)]

use std::borrow::Cow;

use crate::{parse_badges, Badge, Color, Emote, Tags};

use super::{Message, UserType};

/// [`USERNOTICE`](https://dev.twitch.tv/docs/irc/commands/#usernotice). Sent when events like someone subscribing to the channel occurs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct UserNotice<'a> {
    /// The raw underlying string
    pub raw: Cow<'a, str>,
    /// Metadata attached to the message
    pub tags: Tags<'a>,
    /// The name of the channel that the event occurred in.
    pub channel: Cow<'a, str>,
    /// Data attached to the notice
    pub data: Option<Cow<'a, str>>,
}

impl<'a> UserNotice<'a> {
    /// Contains metadata related to the chat badges in the [`badges`](Self::badges) tag.
    pub fn badge_info<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        self.tags
            .get("badge-info")
            .into_iter()
            .flat_map(parse_badges)
    }

    /// Badges attached to a user in a channel.
    pub fn badges<'t: 'a>(&'t self) -> impl Iterator<Item = Badge<'a>> + 't {
        Badge::from_tags(&self.tags)
    }

    /// Emotes in the message.
    pub fn emotes<'t: 'a>(&'t self) -> impl Iterator<Item = Emote<'a>> + 't {
        self.data
            .iter()
            .flat_map(|data| Emote::from_tags(&self.tags, data))
    }

    /// The color of the user’s name in the chat room. This may be [`None`] if it is never set.
    pub fn color(&self) -> Option<Color> {
        self.tags.color()
    }

    /// The user’s display name
    pub fn display_name(&self) -> Option<&str> {
        self.tags.get("display-name")
    }

    /// The message Twitch shows in the chat room for this notice.
    pub fn system_msg(&self) -> Option<&str> {
        self.tags.get("system-msg")
    }

    /// The user is a moderator in the channel.
    pub fn is_moderator(&self) -> bool {
        self.tags.bool("mod")
    }

    /// The user is a subscriber of the channel.
    pub fn is_subscriber(&self) -> bool {
        self.tags.bool("subscriber")
    }

    /// The user is a turbo user.
    pub fn is_turbo(&self) -> bool {
        self.tags.bool("turbo")
    }

    /// The login name of the user whose action generated the message.
    pub fn login(&self) -> Option<&str> {
        self.tags.get("login")
    }

    /// An ID that uniquely identifies this message.
    pub fn id(&self) -> Option<&str> {
        self.tags.get("id")
    }

    /// The type of notice
    pub fn msg_id(&self) -> Option<UserNoticeId> {
        self.tags.get("msg-id").map(UserNoticeId::parse)
    }

    /// An ID that identifies the chat room (channel).
    pub fn room_id(&self) -> Option<&str> {
        self.tags.get("room_id")
    }

    /// The user’s ID.

    pub fn user_id(&self) -> Option<&str> {
        self.tags.get("user-id")
    }

    /// The UNIX timestamp.
    pub fn tmi_sent_ts(&self) -> Option<&str> {
        self.tags.get("tmi-sent-ts")
    }

    /// The user’s type.
    pub fn user_type(&self) -> UserType {
        self.tags
            .get("user-type")
            .map(UserType::parse)
            .unwrap_or_default()
    }

    /// The total number of months the user has subscribed.
    ///
    /// Included only with [`sub`](UserNoticeId::Sub) and [`resub`](UserNoticeId::Resub) notices
    pub fn msg_param_cumulative_months(&self) -> Option<&str> {
        self.tags.get("msg-param-cumulative-months")
    }

    /// The display name of the broadcaster raiding this channel.
    ///
    /// Included only with [`raid`](UserNoticeId::Raid) notices
    pub fn msg_param_display_name(&self) -> Option<&str> {
        // XXX: docs have this in a weird casing, going to try the other as well.
        self.tags
            .get("msg-param-displayName")
            .or_else(|| self.tags.get("msg-param-display-name"))
    }

    /// The display name of the broadcaster raiding this channel.
    ///
    /// Included only with [`raid`](UserNoticeId::Raid) notices
    pub fn msg_param_login(&self) -> Option<&str> {
        self.tags.get("msg-param-login")
    }

    /// The total number of months the user has subscribed.
    ///
    /// Included only with [`subgift`](UserNoticeId::Subgift) notices
    ///
    /// # See also
    ///
    /// [`msg-param-cumulative-months`](Self::msg_param_cumulative_months)
    pub fn msg_param_months(&self) -> Option<&str> {
        self.tags.get("msg-param-months")
    }

    /// The number of gifts the gifter has given during the promo indicated by [`msg-param-promo-name`](Self::msg_param_promo_name)
    ///
    /// Included only with [`anongiftpaidupgrade`](UserNoticeId::AnonGiftPaidUpgrade) and [`giftpaidupgrade`](UserNoticeId::GiftPaidUpgrade) notices
    pub fn msg_param_promo_gift_total(&self) -> Option<&str> {
        self.tags.get("msg-param-promo-gift-total")
    }

    /// The subscriptions promo, if any, that is ongoing (for example, Subtember 2018).
    ///
    /// Included only with [`anongiftpaidupgrade`](UserNoticeId::AnonGiftPaidUpgrade) and [`giftpaidupgrade`](UserNoticeId::GiftPaidUpgrade) notices
    pub fn msg_param_promo_name(&self) -> Option<&str> {
        self.tags.get("msg-param-promo-name")
    }

    /// The display name of the subscription gift recipient.
    ///
    /// Included only with [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_recipient_display_name(&self) -> Option<&str> {
        self.tags.get("msg-param-recipient-display-name")
    }

    /// The user ID of the subscription gift recipient.
    ///
    /// Included only with [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_recipient_id(&self) -> Option<&str> {
        self.tags.get("msg-param-recipient-id")
    }

    /// The user name of the subscription gift recipient.
    ///
    /// Included only with [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_recipient_user_name(&self) -> Option<&str> {
        self.tags.get("msg-param-recipient-user-name")
    }

    /// The login name of the user who gifted the subscription.
    ///
    /// Included only with [`giftpaidupgrade`](UserNoticeId::GiftPaidUpgrade) notices
    pub fn msg_param_sender_login(&self) -> Option<&str> {
        self.tags.get("msg-param-sender-login")
    }

    /// The display name of the user who gifted the subscription.
    ///
    /// Included only with [`giftpaidupgrade`](UserNoticeId::GiftPaidUpgrade) notices
    pub fn msg_param_sender_name(&self) -> Option<&str> {
        self.tags.get("msg-param-sender-name")
    }

    /// Indicates whether the user wants their streaks shared.
    ///
    /// Included only with [`sub`](UserNoticeId::Sub) and [`resub`](UserNoticeId::Resub) notices
    pub fn msg_param_should_share_streak(&self) -> Option<&str> {
        self.tags.get("msg-param-should-share-streak")
    }

    /// The number of consecutive months the user has subscribed.
    ///
    /// Included only with [`sub`](UserNoticeId::Sub) and [`resub`](UserNoticeId::Resub) notices
    pub fn msg_param_streak_months(&self) -> Option<&str> {
        self.tags.get("msg-param-streak-months")
    }

    /// The type of subscription plan being used.
    ///
    /// Included only with [`sub`](UserNoticeId::Sub), [`resub`](UserNoticeId::Resub) and [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_sub_plan(&self) -> Option<&str> {
        self.tags.get("msg-param-sub-plan")
    }

    /// The display name of the subscription plan.
    ///
    /// Included only with [`sub`](UserNoticeId::Sub), [`resub`](UserNoticeId::Resub) and [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_sub_plan_name(&self) -> Option<&str> {
        self.tags.get("msg-param-sub-plan-name")
    }

    /// The number of viewers raiding this channel from the broadcaster’s channel.
    ///
    /// Included only with [`raid`](UserNoticeId::Raid) notices
    pub fn msg_param_viewer_count(&self) -> Option<&str> {
        self.tags.get("msg-param-viewerCount")
    }

    #[deprecated]
    #[allow(missing_docs)]
    pub fn msg_param_ritual_name(&self) -> Option<&str> {
        self.tags.get("msg-param-ritual-name")
    }

    /// The tier of the Bits badge the user just earned.
    ///
    /// Included only with [`bitsbadgetier`](UserNoticeId::BitsBadgeTier) notices
    pub fn msg_param_threshold(&self) -> Option<&str> {
        self.tags.get("msg-param-threshold")
    }

    /// The number of months gifted as part of a single, multi-month gift.
    ///
    /// Included only with [`subgift`](UserNoticeId::Subgift) notices
    pub fn msg_param_gift_months(&self) -> Option<&str> {
        self.tags.get("msg-param-gift-months")
    }

    /// The domain of the rewards being gifted (e.g. "pride_megacommerce_2020").
    ///
    /// Included only with [`rewardgift`](UserNoticeId::RewardGift) notices
    pub fn msg_param_domain(&self) -> Option<&str> {
        self.tags.get("msg-param-domain")
    }

    /// The type of monetary event that triggered the reward gift (e.g., "SUBGIFT", "CHEER").
    ///
    /// Included only with [`rewardgift`](UserNoticeId::RewardGift) notices
    pub fn msg_param_trigger_type(&self) -> Option<&str> {
        self.tags.get("msg-param-trigger-type")
    }

    /// The number of gifted rewards as part of the primary selection.
    ///
    /// Included only with [`rewardgift`](UserNoticeId::RewardGift) notices
    pub fn msg_param_selected_count(&self) -> Option<&str> {
        self.tags.get("msg-param-selected-count")
    }
    /// The total number of rewards being gifted (e.g. 5 emotes).
    ///
    /// Included only with [`rewardgift`](UserNoticeId::RewardGift) notices
    pub fn msg_param_total_reward_count(&self) -> Option<&str> {
        self.tags.get("msg-param-total-reward-count")
    }
    /// The number of instances of the trigger (e.g. 1 sub gift or 300 bits).
    ///
    /// Included only with [`rewardgift`](UserNoticeId::RewardGift) notices
    pub fn msg_param_trigger_amount(&self) -> Option<&str> {
        self.tags.get("msg-param-trigger-amount")
    }
}

/// The type of notice
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum UserNoticeId {
    /// A subscription event
    Sub,
    /// A resubscription event
    Resub,
    /// A gift subscription event
    Subgift,
    /// A mass subscription event occurs.
    SubMysteryGift,
    /// A gifted subscription is continued
    GiftPaidUpgrade,
    /// Monetary event triggered emotes to be shared
    RewardGift,
    /// A gifted subscription from an anonymous user is continued
    AnonGiftPaidUpgrade,
    /// A raid happens
    Raid,
    /// A raid from the channel to another is cancelled.
    Unraid,
    #[allow(missing_docs)]
    #[deprecated]
    Ritual,
    /// A user shares a new bits badge
    BitsBadgeTier,
    /// Unknown notice
    #[default]
    Unknown,
}

impl UserNoticeId {
    fn parse(input: &str) -> Self {
        match input {
            "sub" => Self::Sub,
            "resub" => Self::Resub,
            "subgift" => Self::Subgift,
            "submysterygift" => Self::SubMysteryGift,
            "giftpaidupgrade" => Self::GiftPaidUpgrade,
            "rewardgift" => Self::RewardGift,
            "anongiftpaidupgrade" => Self::AnonGiftPaidUpgrade,
            "raid" => Self::Raid,
            "unraid" => Self::Unraid,
            "ritual" => Self::Ritual,
            "bitsbadgetier" => Self::BitsBadgeTier,
            _ => Self::Unknown,
        }
    }
}

impl UserNotice<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
    }
}

impl<'a> TryFrom<Message<'a>> for UserNotice<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw,
            tags: value.tags,
            channel: value.args.remove(0),
            data: value.data,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for UserNotice<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw.clone(),
            tags: value.tags.clone(),
            channel: value.args[0].clone(),
            data: value.data.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn user_notice() {
        let inputs = [
            "@badge-info=;badges=staff/1,broadcaster/1,turbo/1;color=#008000;display-name=ronni;emotes=;id=db25007f-7a18-43eb-9379-80131e44d633;login=ronni;mod=0;msg-id=resub;msg-param-cumulative-months=6;msg-param-streak-months=2;msg-param-should-share-streak=1;msg-param-sub-plan=Prime;msg-param-sub-plan-name=Prime;room-id=12345678;subscriber=1;system-msg=ronni\\shas\\ssubscribed\\sfor\\s6\\smonths!;tmi-sent-ts=1507246572675;turbo=1;user-id=87654321;user-type=staff :tmi.twitch.tv USERNOTICE #dallas :Great stream -- keep it up!\r\n",
            "@badge-info=;badges=staff/1,premium/1;color=#0000FF;display-name=TWW2;emotes=;id=e9176cd8-5e22-4684-ad40-ce53c2561c5e;login=tww2;mod=0;msg-id=subgift;msg-param-months=1;msg-param-recipient-display-name=Mr_Woodchuck;msg-param-recipient-id=55554444;msg-param-recipient-name=mr_woodchuck;msg-param-sub-plan-name=House\\sof\\sNyoro~n;msg-param-sub-plan=1000;room-id=19571752;subscriber=0;system-msg=TWW2\\sgifted\\sa\\sTier\\s1\\ssub\\sto\\sMr_Woodchuck!;tmi-sent-ts=1521159445153;turbo=0;user-id=87654321;user-type=staff :tmi.twitch.tv USERNOTICE #forstycup\r\n",
            "@badge-info=;badges=turbo/1;color=#9ACD32;display-name=TestChannel;emotes=;id=3d830f12-795c-447d-af3c-ea05e40fbddb;login=testchannel;mod=0;msg-id=raid;msg-param-displayName=TestChannel;msg-param-login=testchannel;msg-param-viewerCount=15;room-id=33332222;subscriber=0;system-msg=15\\sraiders\\sfrom\\sTestChannel\\shave\\sjoined\\n!;tmi-sent-ts=1507246572675;turbo=1;user-id=123456;user-type= :tmi.twitch.tv USERNOTICE #othertestchannel\r\n",
            "@badge-info=;badges=;color=;display-name=SevenTest1;emotes=30259:0-6;id=37feed0f-b9c7-4c3a-b475-21c6c6d21c3d;login=seventest1;mod=0;msg-id=ritual;msg-param-ritual-name=new_chatter;room-id=87654321;subscriber=0;system-msg=Seventoes\\sis\\snew\\shere!;tmi-sent-ts=1508363903826;turbo=0;user-id=77776666;user-type= :tmi.twitch.tv USERNOTICE #seventoes :HeyGuys\r\n",
        ];

        for (input, (channel, data)) in inputs.into_iter().zip([
            ("#dallas", Some("Great stream -- keep it up!")),
            ("#forstycup", None),
            ("#othertestchannel", None),
            ("#seventoes", Some("HeyGuys")),
        ]) {
            let (raw, tags) = test_util::raw_and_tags(input);
            assert_eq!(
                crate::test_util::parse_as::<UserNotice>(input),
                UserNotice {
                    raw,
                    tags,
                    channel: Cow::from(channel),
                    data: data.map(Cow::from)
                }
            );
        }
    }
}
