#![allow(deprecated)]

use crate::{
    messages::{
        Capability, ClearChat, ClearMsg, GlobalUserState, HostTarget, IrcReady, Join, Message,
        MessageKind, Notice, Part, Ping, Pong, Privmsg, Ready, Reconnect, RoomState, UserNotice,
        UserState, Whisper,
    },
    IntoStatic,
};

pub trait TypedMessageMarker<'a>
where
    Self: Sized + IntoStatic + Clone + private::Sealed,
    Self: for<'b> TryFrom<&'b Message<'a>> + TryFrom<Message<'a>>,
{
    #[inline(always)]
    fn is_kind(_kind: &MessageKind) -> bool {
        false
    }

    fn kind() -> &'static str;
}

mod private {
    pub trait Sealed {}
    impl<'a, T> Sealed for T where T: super::TypedMessageMarker<'a> {}
}

macro_rules! typed_message {
    ($($kind:ident => $subtype:ident ; $id:expr)*) => {
        $(
            impl<'a> super::TypedMessageMarker<'a> for $kind<'a> {
                fn is_kind(kind: &MessageKind) -> bool {
                    Self::is_kind(kind)
                }

                fn kind() -> &'static str {
                    stringify!($kind)
                }
            }

            impl $kind<'_> {
                const fn is_kind(kind: &MessageKind) -> bool {
                    Subtype::check::<{Subtype::$subtype}>(kind)
                }
            }
        )*

        struct Subtype;
        impl Subtype {
            const fn check<const K: u8>(this: &MessageKind) -> bool {
                match this {
                    $(MessageKind::$kind => Self::$subtype == K, )*
                    _=> false
                }
            }

            $( const $subtype: u8 = $id; )*
        }
    };
}

typed_message! {
    Capability      => CAP               ; 0
    Ping            => PING              ; 1
    Pong            => PONG              ; 2
    IrcReady        => IRC_READY         ; 3
    Ready           => READY             ; 4
    GlobalUserState => GLOBAL_USER_STATE ; 5
    UserState       => USER_STATE        ; 6
    RoomState       => ROOM_STATE        ; 7
    Privmsg         => PRIV_MSG          ; 8
    ClearChat       => CLEAR_CHAT        ; 9
    ClearMsg        => CLEAR_MSG         ; 10
    Notice          => NOTICE            ; 11
    HostTarget      => HOST_TARGET       ; 12
    UserNotice      => USER_NOTICE       ; 13
    Whisper         => WHISPER           ; 14
    Reconnect       => RECONNECT         ; 15
    Part            => PART              ; 16
    Join            => JOIN              ; 17
}
