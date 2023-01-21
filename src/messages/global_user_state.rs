use std::borrow::Cow;

use super::{Message, Tags};

// "badge-info": "",
// "badges": "premium/1",
// "emote-sets": "0,19194,300374282,300597048,301337952,460515209,537206155,564265402,592920959,610186276",
// "color": "#008000",
// "user-id": "23196011",
// "user-type": "",
// "display-name": "museun"

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct GlobalUserState<'a> {
    pub tags: Tags<'a>,
    pub raw: Cow<'a, str>,
}

impl<'a> TryFrom<Message<'a>> for GlobalUserState<'a> {
    type Error = Message<'a>;

    fn try_from(value: Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags,
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for GlobalUserState<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            tags: value.tags.clone(),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::test_util;

    use super::*;
    #[test]
    fn global_user_state() {
        let input = "@badge-info=;\
        badges=premium/1;\
        color=#008000;\
        display-name=museun;\
        emote-sets=0,19194,300374282,300597048,301337952,460515209,537206155,564265402,592920959,610186276;\
        user-id=23196011;\
        user-type= \
        :tmi.twitch.tv GLOBALUSERSTATE\r\n";

        let (raw, tags) = test_util::raw_and_tags(input);
        assert_eq!(
            test_util::parse_as::<GlobalUserState>(input),
            GlobalUserState { raw, tags }
        );
    }
}
