use twitch_message::{
    encode::{join, register, Encodable as _, Encode, ALL_CAPABILITIES},
    messages::{Message, MessageKind, Privmsg, Ready},
    parse, ParseResult, PingTracker, ANONYMOUS_LOGIN,
};

use std::{io::BufReader, net::TcpStream, time::Duration};

fn main() {
    // open a connection to twitch
    let mut stream = TcpStream::connect(twitch_message::TWITCH_IRC_ADDRESS).unwrap();

    // register with an anonymous user
    let (name, pass) = ANONYMOUS_LOGIN;

    // we can encode message to a writer
    stream
        .encode_msg(register(name, pass, ALL_CAPABILITIES))
        .unwrap();

    // create a new PingTracker. this makes handling PINGs trivial
    let pt = PingTracker::new(Duration::from_secs(5 * 60));

    // create a buffer we'll reuse for every line
    let mut buf = String::with_capacity(1024);
    let mut reader = BufReader::new(&stream);

    loop {
        let msg = read_message(&mut buf, &mut reader);

        // update the ping tracker's internal state
        pt.update(&msg);

        // if the tracker has a pong for us, sent it back out
        if let Some(pong) = pt.should_pong() {
            pong.encode(&stream).unwrap();
        }

        // the `kind` determines what kind of message it is
        if let MessageKind::Ready = msg.kind {
            // its safe to unwrap this because we checked the kind above
            let msg = msg.as_typed_message::<Ready>().unwrap();
            eprintln!("we're connected as {}", msg.name);

            eprintln!("joining a channel..");
            // we're ready to do things. so lets join a channel

            // this prepends a # for you, if you forgot to
            // (this is the other way of encoding messages)
            join("museun").encode(&stream).unwrap();
        }

        // if you're only looking for a specific message, you can do this:
        // if let Ok(msg) = parse_as::<Privmsg>(raw_line) {}

        // but since we've already parsed the message, convert it to a subtype
        if let Some(msg) = msg.as_typed_message::<Privmsg>() {
            eprintln!(
                "[{channel}] {sender}: {data}",
                channel = msg.channel,
                sender = msg.sender,
                data = msg.data
            );
        }
    }
}

fn read_message<'a>(buf: &'a mut String, mut read: impl std::io::BufRead + Sized) -> Message<'a> {
    // clear the buffer
    buf.clear();

    // read a line into the buffer, returning how many bytes were read
    let pos = read.read_line(buf).unwrap();

    // parse a message from the read buffer
    // this returns any remaining data (partial parses)
    // but because we read a single line, it'd be empty.
    let ParseResult { message, .. } = parse(&buf[..pos]).unwrap();
    message
}
