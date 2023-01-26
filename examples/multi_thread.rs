use std::{
    io::{BufRead, BufReader, Write},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use twitch_message::{
    encode::{register, Formattable, ALL_CAPABILITIES},
    messages::Message,
    parse_many, IntoStatic, ANONYMOUS_LOGIN, TWITCH_IRC_ADDRESS,
};

fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect(TWITCH_IRC_ADDRESS)?;

    let (tx, message) = std::sync::mpsc::channel();
    let read_handle = std::thread::spawn({
        let stream = stream.try_clone()?;
        let stream = BufReader::new(stream);
        move || reader(stream, tx)
    });

    let (sender, rx) = std::sync::mpsc::channel();
    let write_handle = std::thread::spawn(move || writer(stream, rx));

    let mut buf = String::with_capacity(1024);
    let (name, oauth) = ANONYMOUS_LOGIN;
    register(name, oauth, ALL_CAPABILITIES).format(&mut buf)?;

    sender.send(std::mem::take(&mut buf))?;

    for msg in message {
        eprintln!("{}", msg.raw.escape_debug());
    }

    read_handle.join().expect("thread should not panic")?;
    write_handle.join().expect("thread should not panic")?;

    Ok(())
}

fn reader<R>(stream: R, out: Sender<Message<'static>>) -> anyhow::Result<()>
where
    R: BufRead + 'static + Send + Sync,
{
    for line in stream.lines() {
        let line = line?;
        for msg in parse_many(&line) {
            out.send(msg?.into_static())?;
        }
    }

    Ok(())
}

fn writer<W>(mut stream: W, out: Receiver<String>) -> anyhow::Result<()>
where
    W: Write + 'static + Send + Sync,
{
    for line in out {
        stream.write_all(line.as_bytes())?;
        stream.flush()?;
    }
    Ok(())
}
