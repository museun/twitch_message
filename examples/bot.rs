use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
    time::{Duration, Instant},
};

use twitch_message::{
    encode::{join, privmsg, register, reply, Encodable, Encode, ALL_CAPABILITIES},
    messages::{Message, MessageKind, Privmsg},
    parse, PingTracker, TWITCH_IRC_ADDRESS,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    simple_env_load::load_env_from([".example.env", ".secrets.env"]);

    fn get(key: &str) -> Result<String, String> {
        std::env::var(key).map_err(|_| format!("please set {key} in .example.env"))
    }

    let name = get("TWITCH_USER_NAME")?;
    let pass = get("TWITCH_OAUTH_TOKEN")?;

    let commands = Commands::default()
        .with("~hello", Bot::hello)
        .with("~uptime", Bot::uptime)
        .with("~github", Bot::send_github);

    eprintln!("connecting");
    Bot::connect(&name, &pass, "museun", commands)?.run();
    eprintln!("disconnected");

    Ok(())
}

struct Bot {
    stream: BufReader<TcpStream>,
    commands: Commands,
    start: Instant,
    ping: PingTracker,
}

impl Bot {
    fn connect(
        name: &str,
        pass: &str,
        channel: &str,
        commands: Commands,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut stream = TcpStream::connect(TWITCH_IRC_ADDRESS)?;
        stream.encode_msg(register(name, pass, ALL_CAPABILITIES))?;
        let mut stream = BufReader::new(stream);

        let ping = PingTracker::new(Duration::from_secs(5 * 60));
        let mut buf = String::new();
        loop {
            let msg = Self::read_message(&mut buf, &mut stream, &ping);
            if let MessageKind::Ready = msg.kind {
                break;
            }
        }

        eprintln!("connected. joining {channel}");
        join(channel).encode(stream.get_ref()).unwrap();

        Ok(Self {
            stream,
            commands,
            start: Instant::now(),
            ping,
        })
    }

    fn run(mut self) {
        let mut buf = String::with_capacity(1024);

        loop {
            let msg = Self::read_message(&mut buf, &mut self.stream, &self.ping);
            if let Some(pm) = msg.as_typed_message::<Privmsg>() {
                eprintln!(
                    "[{channel}] {sender}: {data}",
                    channel = pm.channel,
                    sender = pm.sender,
                    data = pm.data
                );

                if let Some(func) = self.commands.map.get(&*pm.data) {
                    (func)(&self, &pm)
                }
            }
        }
    }

    fn read_message<'a, T>(
        buf: &'a mut String,
        reader: &mut BufReader<T>,
        pt: &PingTracker,
    ) -> Message<'a>
    where
        T: std::io::Read + std::io::Write,
        for<'i> &'i T: std::io::Read + std::io::Write,
    {
        buf.clear();
        let pos = reader.read_line(buf).unwrap();
        let msg = parse(&buf[..pos]).unwrap().message;

        pt.update(&msg);
        if let Some(pong) = pt.should_pong() {
            pong.encode(reader.get_ref()).unwrap()
        }

        msg
    }
}

impl Bot {
    fn hello(&self, pm: &Privmsg<'_>) {
        privmsg(&pm.channel, &format!("hello: {}", pm.sender))
            .encode(self.stream.get_ref())
            .unwrap();
    }

    fn uptime(&self, pm: &Privmsg<'_>) {
        privmsg(
            &pm.channel,
            &format!("uptime is {:.2?}", self.start.elapsed()),
        )
        .encode(self.stream.get_ref())
        .unwrap();
    }

    fn send_github(&self, pm: &Privmsg<'_>) {
        reply(
            pm.msg_id().unwrap(),
            &pm.channel,
            "https://github.com/museun/twitch_message",
        )
        .encode(self.stream.get_ref())
        .unwrap();
    }
}

type Handler = for<'a> fn(&Bot, &Privmsg<'a>);

#[derive(Default)]
struct Commands {
    map: HashMap<String, Handler>,
}

impl Commands {
    fn with(mut self, command: impl ToString, func: Handler) -> Self {
        self.map.insert(command.to_string(), func);
        self
    }
}
