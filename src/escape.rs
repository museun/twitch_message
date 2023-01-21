use std::borrow::Cow;

pub fn estimate_escape_size(input: &str) -> usize {
    const ESCAPEE: [char; 5] = [';', ' ', '\\', '\n', '\r'];
    input.chars().filter(|c| ESCAPEE.contains(c)).count()
}

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
