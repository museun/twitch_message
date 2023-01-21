# twitch_message

This is a crate to parse chat messages from [https://www.twitch.tv](https://www.twitch.tv)

This crate does not provide any I/O rather just parsing of a `&str` into typed messages.

A quick walkthrough:
```rust
use twitch_messages::messages::*;
let data: &'a str = read_line();
let msg: Message<'a> = twitch_messages::parse(data).unwrap();
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
        let resp = twitch_messages::encode::pong(&ping.token);
        io_sink.encode_msg(resp).unwrap();
    }
    _ => {}
}
```

## Parsing
#### There are various *parse* methods provided by this crate:
- [`fn@parse`]

This will parse a single message, returning any remaining data

- [`fn@parse_many`]

This will return an iterator over possibly message messages in the data

- [`fn@parse_as`]

This is a shorthand for [`fn@parse`] + [`Message::as_typed_message()`](crate::messages::Message::as_typed_message())

- [`fn@parse_badges`]

This allows you to parse ***badges*** from a string

- [`fn@parse_emotes`]

This allows you to parse ***emotes*** from a Twitch emote string + the associated data portion

---

## Typed messages
Once you parse data into a [`Message`](crate::messages::Message), you can further narrow it to a specific type via two methods:
- [`Message::as_typed_message()`](crate::messages::Message::as_typed_message())
- [`Message::into_typed_message()`](crate::messages::Message::into_typed_message())

`as_typed_message` will borrow from the message, forgoing any further allocations.

`into_typed_message` will clone the data so you'll have an owned (`'static`) version of the message

The argument (`type`) used for these are one of the main structs found in the [`messages`] module.

## Ownership
If you have a `Message<'a>` or some sub type (found in [`messages`]) and want it to be `'static`, a trait is provided:

[`IntoStatic`]

This trait is implemented for all of the types. Once you import it, you can do `ty.into_static()` to get a `'static` version of the type.

## Builders
A few builders are provided:
- [`PrivmsgBuilder`](crate::builders::PrivmsgBuilder)
- [`TagsBuilder`](crate::builders::TagsBuilder)

These allow you to construct messages for testing, or for custom purposes (mocking/faking, etc)

## Encoding
The [`encode`] module provides a typed way of constructing messages to send to Twitch.

By default, only encoding to a [`core::fmt::Write`] source (e.g. a `String`) is support, via the [`Format`](crate::encode::Format) and [`Formattable`](crate::encode::Formattable) traits.

If you enable the `std` feature (see [features](#features)), you will have access to the [`Encode`](crate::encode::Encode) and [`Encodable`](crate::encode::Encodable) traits which operate on a [`std::io::Write`] source. (e.g. a [`Vec<u8>`] or [`std::net::TcpStream`])

#### Example
##### Format/Formattable
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

##### Encode/Encodable
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
| Feature | Description |
| --- | --- |
|default | there are no default features |
|ping | enables the [`PingTracker`] |
|std | enables the [`Encode`](crate::encode::Encode) and [`Encodable`](crate::encode::Encodable) traits |
|serde | enables [`serde`] derives on the types |
|hashbrown | enables using [`hashbrown`] for the internal `HashMaps` |
|sync | enables using [`std::sync::Mutex`] over [`std::cell::RefCell`] see [`sharing data`](#sharing-data) |
|parking_lot | same as `sync` except uses a [`parking_lot::Mutex`] |

## Utilities
### PingTracker

A `PingTracker` is provided, and is entirely optional (enabled with the `ping` feature).

This is a simple type to help you determine when you should respond to a `PING` message.
```rust
// create a new tracker, the `threshold` is used to determine when a connection is dead/stale.
let pt = PingTracker::new(std::time::Duration::from_secs(10 * 60));

// in some loop
// if its been a while (such as if you have a way to keep track of time)
if pt.probably_timed_out() {
    // we should reconnect
    return;
}

// this might block for a while
let msg = read_message();
// update the tracker
pt.update(&msg);

// check to see if you should reply.
// this returns a message you can write to your sink
if let Some(pong) = pt.should_pong() {
    io_sink.encode_msg(pong).unwrap();
}
```

### Tag (un)escaping
IRCv3 requires tags to be [escaped](https://ircv3.net/specs/extensions/message-tags.html#escaping-values).

This crate provides a method to [escape them](crate::escape::escape_tag), and to [unescape them](crate::escape::unescape_tag).

*NOTE* You don't have to worry about the escape-status of [`Tags`], interally these are used.

These methods are provided for your own use cases.

---
Twitch chat reference: [`link`](https://dev.twitch.tv/docs/irc/)

License: 0BSD
