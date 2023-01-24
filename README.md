# twitch_message

Read the [docs](https://docs.rs/twitch_message/latest/twitch_message) for more detailed information

---

This is a crate to parse chat messages from [https://www.twitch.tv](https://www.twitch.tv)

This crate does not provide any I/O rather just parsing of a `&str` into typed messages.

A quick walkthrough:

```rust
use twitch_message::messages::*;
// get some data from somewhere
let data: &str = read_line();

// parse returns a `ParseResult` which contains the remaining data (if any) and the parsed message
let result = twitch_message::parse(data).unwrap();
let msg: Message<'_> = result.message;

match msg.kind {
    MessageKind::Ready => {
        let ready = msg.as_typed_message::<Ready>().unwrap();
        println!("connected as: {name}", name = ready.name);
    }
    MessageKind::Privmsg => {
        let pm = msg.as_typed_message::<Privmsg>().unwrap();
        println!("[{channel}] {sender}: {data}",
            channel = pm.channel,
            sender = pm.sender,
            data = pm.data
        );
    }
    MessageKind::Ping => {
        let ping = msg.as_typed_message::<Ping>().unwrap();
        let resp = twitch_message::encode::pong(&ping.token);

        // you can format data to various 'sinks'
        use twitch_message::encode::Formattable;
        let mut out = String::new();
        resp.format(&mut out).unwrap();
        assert_eq!(out, "PONG :1234567890\r\n");
    }
    _ => {}
}
```

---

## Encoding

### Format/Formattable

```rust
// this adds the # to the channel, if its missing
let pm = twitch_message::encode::privmsg("museun", "hello, world.");

// using `Formattable`
use twitch_message::encode::Formattable;
let mut buf = String::new();
pm.format(&mut buf).unwrap();
assert_eq!(buf, "PRIVMSG #museun :hello, world.\r\n");

// using `Format`
use twitch_message::encode::Format;
let mut buf = String::new();
buf.format_msg(pm);
assert_eq!(buf, "PRIVMSG #museun :hello, world.\r\n");
```

### Encode/Encodable

```rust
// this adds the # to the channel, if its missing
let pm = twitch_message::encode::privmsg("museun", "hello, world.");

// using `Encodable`
use twitch_message::encode::Encodable;
let mut buf = Vec::new();
pm.encode(&mut buf).unwrap();
assert_eq!(buf, b"PRIVMSG #museun :hello, world.\r\n");

// using `Encode`
use twitch_message::encode::Encode;
let mut buf = Vec::new();
buf.encode_msg(pm);
assert_eq!(buf, b"PRIVMSG #museun :hello, world.\r\n");
```

## Features

| Feature     | Description                                                                                        |
| ----------- | -------------------------------------------------------------------------------------------------- |
| default     | there are no default features                                                                      |
| ping        | enables the [`PingTracker`]                                                                        |
| std         | enables the [`Encode`](crate::encode::Encode) and [`Encodable`](crate::encode::Encodable) traits   |
| serde       | enables [`serde`] derives on the types                                                             |
| hashbrown   | enables using [`hashbrown`] for the internal `HashMap`                                             |
| sync        | enables using [`std::sync::Mutex`] over [`std::cell::RefCell`] see [`sharing data`](#sharing-data) |
| parking_lot | same as `sync` except uses a [`parking_lot::Mutex`]                                                |

---

Twitch chat reference: [`link`](https://dev.twitch.tv/docs/irc/)

License: 0BSD
