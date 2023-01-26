use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use native_tls::TlsConnector;

use twitch_message::{
    encode::{register, Encodable, ALL_CAPABILITIES},
    parse_many, ANONYMOUS_LOGIN, TWITCH_IRC_ADDRESS_TLS, TWITCH_TLS_DOMAIN,
};

fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect(TWITCH_IRC_ADDRESS_TLS)?;

    let connector = TlsConnector::new()?;
    let mut stream = connector.connect(TWITCH_TLS_DOMAIN, stream)?;

    let (name, oauth) = ANONYMOUS_LOGIN;
    register(name, oauth, ALL_CAPABILITIES).encode(&mut stream)?;
    stream.flush()?;

    for line in BufReader::new(stream).lines() {
        let line = line?;
        for msg in parse_many(&line) {
            eprintln!("{}", msg?.raw);
        }
    }

    Ok(())
}
