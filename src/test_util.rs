use std::borrow::Cow;

use crate::{typed_messages::TypedMessageMarker, Parse, Tags};

pub fn parse_as<'a, T: TypedMessageMarker<'a>>(input: &'a str) -> T {
    crate::parse_as::<T>(input).unwrap()
}

pub fn raw(input: &str) -> Cow<'_, str> {
    Cow::from(&input[..input.len() - 2])
}

pub fn raw_and_tags(input: &str) -> (Cow<'_, str>, Tags<'_>) {
    let s = &mut &input[..input.find(' ').unwrap() + 1];
    (raw(input), Tags::parse(s).unwrap())
}

#[test]
#[ignore]
fn log() {
    use std::io::{BufRead, BufReader, Write};

    simple_env_load::load_env_from([".secrets.env"]);
    let pass = std::env::var("TWITCH_OAUTH_TOKEN").unwrap();

    let stream = std::net::TcpStream::connect("irc.chat.twitch.tv:6667").unwrap();

    writeln!(&stream, "CAP REQ twitch.tv/commands\r\n").unwrap();
    writeln!(&stream, "CAP REQ twitch.tv/tags\r\n").unwrap();
    writeln!(&stream, "CAP REQ twitch.tv/membership\r\n").unwrap();
    writeln!(&stream, "PASS {pass}\r\n").unwrap();
    writeln!(&stream, "NICK museun\r\n").unwrap();
    writeln!(&stream, "JOIN #museun\r\n").unwrap();
    { &stream }.flush().unwrap();

    for line in BufReader::new(&stream).lines().flatten() {
        eprintln!("{}", line.escape_debug());
    }
}
