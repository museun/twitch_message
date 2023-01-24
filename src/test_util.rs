use std::borrow::Cow;

use crate::{typed_messages::TypedMessageMarker, Parse, Tags};

#[track_caller]
pub fn parse_as<'a, T: TypedMessageMarker<'a>>(input: &'a str) -> T {
    crate::parse_as::<T>(input).unwrap()
}

#[track_caller]
pub fn raw(input: &str) -> Cow<'_, str> {
    Cow::from(&input[..input.len() - 2])
}

#[track_caller]
pub fn raw_and_tags(input: &str) -> (Cow<'_, str>, Tags<'_>) {
    let s = &mut &input[..input.find(' ').unwrap() + 1];
    (raw(input), Tags::parse(s).unwrap())
}
