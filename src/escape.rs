//! String escaping helpers
//!
//! # Table of escape mapping:
//! |Character|Sequence in \<escaped value\>|
//! |---|---|
//! |; (semicolon)|\\: (backslash and colon)|
//! |SPACE|\s|
//! |\\|\\\\|
//! |CR|\r|
//! |LF|\n|
//! |all others|the character itself|
use std::borrow::Cow;

/// Estimate how much extra bytes you'll need if you escaped this string
pub fn estimate_escape_size(input: &str) -> usize {
    const ESCAPEE: [char; 5] = [';', ' ', '\\', '\n', '\r'];
    input.chars().filter(|c| ESCAPEE.contains(c)).count()
}

/// Unescape the string according to [IRCv3 Tags](https://ircv3.net/specs/extensions/message-tags.html)
pub fn unescape_tag(input: &str) -> Cow<'_, str> {
    if !input.chars().any(|c| c == '\\') {
        return input.into();
    }

    let mut buf = String::with_capacity(input.len());
    let mut iter = input.chars();
    while let Some(ch) = iter.next() {
        match ch {
            '\\' => match iter.next() {
                Some(':') => buf.push(';'),
                Some('s') => buf.push(' '),
                Some('\\') => buf.push('\\'),
                Some('r') => buf.push('\r'),
                Some('n') => buf.push('\n'),
                Some(ch) => buf.push(ch),
                None => break,
            },
            ch => buf.push(ch),
        }
    }
    buf.into()
}

/// Escape the string according to [IRCv3 Tags](https://ircv3.net/specs/extensions/message-tags.html)
pub fn escape_tag(input: &str) -> Cow<'_, str> {
    let n = estimate_escape_size(input);
    if n == 0 {
        return input.into();
    }

    let mut buf = String::with_capacity(input.len() + n);
    for ch in input.chars() {
        match ch {
            ';' => buf.push_str(r"\:"),
            ' ' => buf.push_str(r"\s"),
            '\\' => buf.push_str(r"\\"),
            '\r' => buf.push_str(r"\r"),
            '\n' => buf.push_str(r"\n"),
            ch => buf.push(ch),
        }
    }
    buf.into()
}
