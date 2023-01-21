use std::borrow::Cow;

use super::{Message, Tags};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Notice<'a> {
    pub raw: Cow<'a, str>,
    pub channel: Cow<'a, str>,
    pub message: Cow<'a, str>,
    pub tags: Tags<'a>,
}

impl<'a> Notice<'a> {
    pub fn notice_kind(&self) -> NoticeKind {
        self.tags
            .get("msg-id")
            .map(NoticeKind::parse)
            .unwrap_or_default()
    }

    pub fn target_user_id(&self) -> Option<&str> {
        self.tags.get("target-user-id")
    }
}

impl Notice<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty() && value.data.is_some()
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum NoticeKind {
    AlreadyBanned,
    AlreadyEmoteOnlyOff,
    AlreadyEmoteOnlyOn,
    AlreadyFollowersOff,
    AlreadyFollowersOn,
    AlreadyR9kOff,
    AlreadyR9kOn,
    AlreadySlowOff,
    AlreadySlowOn,
    AlreadySubsOff,
    AlreadySubsOn,
    AutohostReceive,
    BadBanAdmin,
    BadBanAnon,
    BadBanBroadcaster,
    BadBanMod,
    BadBanSelf,
    BadBanStaff,
    BadCommercialError,
    BadDeleteMessageBroadcaster,
    BadDeleteMessageMod,
    BadHostError,
    BadHostHosting,
    BadHostRateExceeded,
    BadHostRejected,
    BadHostSelf,
    BadModBanned,
    BadModMod,
    BadSlowDuration,
    BadTimeoutAdmin,
    BadTimeoutAnon,
    BadTimeoutBroadcaster,
    BadTimeoutDuration,
    BadTimeoutMod,
    BadTimeoutSelf,
    BadTimeoutStaff,
    BadUnbanNoBan,
    BadUnhostError,
    BadUnmodMod,
    BadVipGranteeBanned,
    BadVipGranteeAlreadyVip,
    BadVipMaxVipsReached,
    BadVipAchievementIncomplete,
    BadUnvipGranteeNotVip,
    BanSuccess,
    CmdsAvailable,
    ColorChanged,
    CommercialSuccess,
    DeleteMessageSuccess,
    DeleteStaffMessageSuccess,
    EmoteOnlyOff,
    EmoteOnlyOn,
    FollowersOff,
    FollowersOn,
    FollowersOnZero,
    HostOff,
    HostOn,
    HostReceive,
    HostReceiveNoCount,
    HostTargetWentOffline,
    HostsRemaining,
    InvalidUser,
    ModSuccess,
    MsgBanned,
    MsgBadCharacters,
    MsgChannelBlocked,
    MsgChannelSuspended,
    MsgDuplicate,
    MsgEmoteonly,
    MsgFollowersonly,
    MsgFollowersonlyFollowed,
    MsgFollowersonlyZero,
    MsgR9k,
    MsgRatelimit,
    MsgRejected,
    MsgRejectedMandatory,
    MsgRequiresVerifiedPhoneNumber,
    MsgSlowmode,
    MsgSubsonly,
    MsgSuspended,
    MsgTimedout,
    MsgVerifiedEmail,
    NoHelp,
    NoMods,
    NoVips,
    NotHosting,
    NoPermission,
    R9kOff,
    R9kOn,
    RaidErrorAlreadyRaiding,
    RaidErrorForbidden,
    RaidErrorSelf,
    RaidErrorTooManyViewers,
    RaidErrorUnexpected,
    RaidNoticeMature,
    RaidNoticeRestrictedChat,
    RoomMods,
    SlowOff,
    SlowOn,
    SubsOff,
    SubsOn,
    TimeoutNoTimeout,
    TimeoutSuccess,
    TosBan,
    TurboOnlyColor,
    UnavailableCommand,
    UnbanSuccess,
    UnmodSuccess,
    UnraidErrorNoActiveRaid,
    UnraidErrorUnexpected,
    UnraidSuccess,
    UnrecognizedCmd,
    UntimeoutBanned,
    UntimeoutSuccess,
    UnvipSuccess,
    UsageBan,
    UsageClear,
    UsageColor,
    UsageCommercial,
    UsageDisconnect,
    UsageDelete,
    UsageEmoteOnlyOff,
    UsageEmoteOnlyOn,
    UsageFollowersOff,
    UsageFollowersOn,
    UsageHelp,
    UsageHost,
    UsageMarker,
    UsageMe,
    UsageMod,
    UsageMods,
    UsageR9kOff,
    UsageR9kOn,
    UsageRaid,
    UsageSlowOff,
    UsageSlowOn,
    UsageSubsOff,
    UsageSubsOn,
    UsageTimeout,
    UsageUnban,
    UsageUnhost,
    UsageUnmod,
    UsageUnraid,
    UsageUntimeout,
    UsageUnvip,
    UsageUser,
    UsageVip,
    UsageVips,
    UsageWhisper,
    VipSuccess,
    VipsSuccess,
    WhisperBanned,
    WhisperBannedRecipient,
    WhisperInvalidLogin,
    WhisperInvalidSelf,
    WhisperLimitPerMin,
    WhisperLimitPerSec,
    WhisperRestricted,
    WhisperRestrictedRecipient,
    #[default]
    Unknown,
}

impl NoticeKind {
    fn parse(input: &str) -> Self {
        match input {
            "already_banned" => Self::AlreadyBanned,
            "already_emote_only_off" => Self::AlreadyEmoteOnlyOff,
            "already_emote_only_on" => Self::AlreadyEmoteOnlyOn,
            "already_followers_off" => Self::AlreadyFollowersOff,
            "already_followers_on" => Self::AlreadyFollowersOn,
            "already_r9k_off" => Self::AlreadyR9kOff,
            "already_r9k_on" => Self::AlreadyR9kOn,
            "already_slow_off" => Self::AlreadySlowOff,
            "already_slow_on" => Self::AlreadySlowOn,
            "already_subs_off" => Self::AlreadySubsOff,
            "already_subs_on" => Self::AlreadySubsOn,
            "autohost_receive" => Self::AutohostReceive,
            "bad_ban_admin" => Self::BadBanAdmin,
            "bad_ban_anon" => Self::BadBanAnon,
            "bad_ban_broadcaster" => Self::BadBanBroadcaster,
            "bad_ban_mod" => Self::BadBanMod,
            "bad_ban_self" => Self::BadBanSelf,
            "bad_ban_staff" => Self::BadBanStaff,
            "bad_commercial_error" => Self::BadCommercialError,
            "bad_delete_message_broadcaster" => Self::BadDeleteMessageBroadcaster,
            "bad_delete_message_mod" => Self::BadDeleteMessageMod,
            "bad_host_error" => Self::BadHostError,
            "bad_host_hosting" => Self::BadHostHosting,
            "bad_host_rate_exceeded" => Self::BadHostRateExceeded,
            "bad_host_rejected" => Self::BadHostRejected,
            "bad_host_self" => Self::BadHostSelf,
            "bad_mod_banned" => Self::BadModBanned,
            "bad_mod_mod" => Self::BadModMod,
            "bad_slow_duration" => Self::BadSlowDuration,
            "bad_timeout_admin" => Self::BadTimeoutAdmin,
            "bad_timeout_anon" => Self::BadTimeoutAnon,
            "bad_timeout_broadcaster" => Self::BadTimeoutBroadcaster,
            "bad_timeout_duration" => Self::BadTimeoutDuration,
            "bad_timeout_mod" => Self::BadTimeoutMod,
            "bad_timeout_self" => Self::BadTimeoutSelf,
            "bad_timeout_staff" => Self::BadTimeoutStaff,
            "bad_unban_no_ban" => Self::BadUnbanNoBan,
            "bad_unhost_error" => Self::BadUnhostError,
            "bad_unmod_mod" => Self::BadUnmodMod,
            "bad_vip_grantee_banned" => Self::BadVipGranteeBanned,
            "bad_vip_grantee_already_vip" => Self::BadVipGranteeAlreadyVip,
            "bad_vip_max_vips_reached" => Self::BadVipMaxVipsReached,
            "bad_vip_achievement_incomplete" => Self::BadVipAchievementIncomplete,
            "bad_unvip_grantee_not_vip" => Self::BadUnvipGranteeNotVip,
            "ban_success" => Self::BanSuccess,
            "cmds_available" => Self::CmdsAvailable,
            "color_changed" => Self::ColorChanged,
            "commercial_success" => Self::CommercialSuccess,
            "delete_message_success" => Self::DeleteMessageSuccess,
            "delete_staff_message_success" => Self::DeleteStaffMessageSuccess,
            "emote_only_off" => Self::EmoteOnlyOff,
            "emote_only_on" => Self::EmoteOnlyOn,
            "followers_off" => Self::FollowersOff,
            "followers_on" => Self::FollowersOn,
            "followers_on_zero" => Self::FollowersOnZero,
            "host_off" => Self::HostOff,
            "host_on" => Self::HostOn,
            "host_receive" => Self::HostReceive,
            "host_receive_no_count" => Self::HostReceiveNoCount,
            "host_target_went_offline" => Self::HostTargetWentOffline,
            "hosts_remaining" => Self::HostsRemaining,
            "invalid_user" => Self::InvalidUser,
            "mod_success" => Self::ModSuccess,
            "msg_banned" => Self::MsgBanned,
            "msg_bad_characters" => Self::MsgBadCharacters,
            "msg_channel_blocked" => Self::MsgChannelBlocked,
            "msg_channel_suspended" => Self::MsgChannelSuspended,
            "msg_duplicate" => Self::MsgDuplicate,
            "msg_emoteonly" => Self::MsgEmoteonly,
            "msg_followersonly" => Self::MsgFollowersonly,
            "msg_followersonly_followed" => Self::MsgFollowersonlyFollowed,
            "msg_followersonly_zero" => Self::MsgFollowersonlyZero,
            "msg_r9k" => Self::MsgR9k,
            "msg_ratelimit" => Self::MsgRatelimit,
            "msg_rejected" => Self::MsgRejected,
            "msg_rejected_mandatory" => Self::MsgRejectedMandatory,
            "msg_requires_verified_phone_number" => Self::MsgRequiresVerifiedPhoneNumber,
            "msg_slowmode" => Self::MsgSlowmode,
            "msg_subsonly" => Self::MsgSubsonly,
            "msg_suspended" => Self::MsgSuspended,
            "msg_timedout" => Self::MsgTimedout,
            "msg_verified_email" => Self::MsgVerifiedEmail,
            "no_help" => Self::NoHelp,
            "no_mods" => Self::NoMods,
            "no_vips" => Self::NoVips,
            "not_hosting" => Self::NotHosting,
            "no_permission" => Self::NoPermission,
            "r9k_off" => Self::R9kOff,
            "r9k_on" => Self::R9kOn,
            "raid_error_already_raiding" => Self::RaidErrorAlreadyRaiding,
            "raid_error_forbidden" => Self::RaidErrorForbidden,
            "raid_error_self" => Self::RaidErrorSelf,
            "raid_error_too_many_viewers" => Self::RaidErrorTooManyViewers,
            "raid_error_unexpected" => Self::RaidErrorUnexpected,
            "raid_notice_mature" => Self::RaidNoticeMature,
            "raid_notice_restricted_chat" => Self::RaidNoticeRestrictedChat,
            "room_mods" => Self::RoomMods,
            "slow_off" => Self::SlowOff,
            "slow_on" => Self::SlowOn,
            "subs_off" => Self::SubsOff,
            "subs_on" => Self::SubsOn,
            "timeout_no_timeout" => Self::TimeoutNoTimeout,
            "timeout_success" => Self::TimeoutSuccess,
            "tos_ban" => Self::TosBan,
            "turbo_only_color" => Self::TurboOnlyColor,
            "unavailable_command" => Self::UnavailableCommand,
            "unban_success" => Self::UnbanSuccess,
            "unmod_success" => Self::UnmodSuccess,
            "unraid_error_no_active_raid" => Self::UnraidErrorNoActiveRaid,
            "unraid_error_unexpected" => Self::UnraidErrorUnexpected,
            "unraid_success" => Self::UnraidSuccess,
            "unrecognized_cmd" => Self::UnrecognizedCmd,
            "untimeout_banned" => Self::UntimeoutBanned,
            "untimeout_success" => Self::UntimeoutSuccess,
            "unvip_success" => Self::UnvipSuccess,
            "usage_ban" => Self::UsageBan,
            "usage_clear" => Self::UsageClear,
            "usage_color" => Self::UsageColor,
            "usage_commercial" => Self::UsageCommercial,
            "usage_disconnect" => Self::UsageDisconnect,
            "usage_delete" => Self::UsageDelete,
            "usage_emote_only_off" => Self::UsageEmoteOnlyOff,
            "usage_emote_only_on" => Self::UsageEmoteOnlyOn,
            "usage_followers_off" => Self::UsageFollowersOff,
            "usage_followers_on" => Self::UsageFollowersOn,
            "usage_help" => Self::UsageHelp,
            "usage_host" => Self::UsageHost,
            "usage_marker" => Self::UsageMarker,
            "usage_me" => Self::UsageMe,
            "usage_mod" => Self::UsageMod,
            "usage_mods" => Self::UsageMods,
            "usage_r9k_off" => Self::UsageR9kOff,
            "usage_r9k_on" => Self::UsageR9kOn,
            "usage_raid" => Self::UsageRaid,
            "usage_slow_off" => Self::UsageSlowOff,
            "usage_slow_on" => Self::UsageSlowOn,
            "usage_subs_off" => Self::UsageSubsOff,
            "usage_subs_on" => Self::UsageSubsOn,
            "usage_timeout" => Self::UsageTimeout,
            "usage_unban" => Self::UsageUnban,
            "usage_unhost" => Self::UsageUnhost,
            "usage_unmod" => Self::UsageUnmod,
            "usage_unraid" => Self::UsageUnraid,
            "usage_untimeout" => Self::UsageUntimeout,
            "usage_unvip" => Self::UsageUnvip,
            "usage_user" => Self::UsageUser,
            "usage_vip" => Self::UsageVip,
            "usage_vips" => Self::UsageVips,
            "usage_whisper" => Self::UsageWhisper,
            "vip_success" => Self::VipSuccess,
            "vips_success" => Self::VipsSuccess,
            "whisper_banned" => Self::WhisperBanned,
            "whisper_banned_recipient" => Self::WhisperBannedRecipient,
            "whisper_invalid_login" => Self::WhisperInvalidLogin,
            "whisper_invalid_self" => Self::WhisperInvalidSelf,
            "whisper_limit_per_min" => Self::WhisperLimitPerMin,
            "whisper_limit_per_sec" => Self::WhisperLimitPerSec,
            "whisper_restricted" => Self::WhisperRestricted,
            "whisper_restricted_recipient" => Self::WhisperRestrictedRecipient,
            _ => Self::Unknown,
        }
    }
}

impl<'a> TryFrom<Message<'a>> for Notice<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw,
            channel: value.args.remove(0),
            message: value.data.unwrap(),
            tags: value.tags,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for Notice<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            raw: value.raw.clone(),
            channel: value.args[0].clone(),
            message: value.data.clone().unwrap(),
            tags: value.tags.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn notice() {
        let input =
            "@room-id=12345678;tmi-sent-ts=1642715695392 :tmi.twitch.tv NOTICE #museun :TOS ban.\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<Notice>(input),
            Notice {
                raw,
                tags,
                channel: Cow::from("#museun"),
                message: Cow::from("TOS ban.")
            }
        );
    }
}
