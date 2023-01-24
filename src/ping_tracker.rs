use std::time::{Duration, Instant};

use crate::{
    encode::{pong, Pong},
    lock::Lock,
    messages::{Message, Ping},
};

/// A simple type to track PINGs and if you should PONG
///
/// This requires the `ping` feature to be enabled
///
/// If either `sync` or `parking_lot` features are also enabled, then this type is safe to send to other threads
///
/// ```rust
/// # use twitch_message::messages::Message;
/// # use twitch_message::encode::Encode as _;
/// # fn read_message() -> Message<'static> { twitch_message::parse(":tmi.twitch.tv PING :1234567890\r\n").unwrap().message }
/// # let mut io_sink = vec![];
/// use twitch_message::PingTracker;
/// // create a new tracker, the `threshold` is used to determine when a connection is dead/stale.
/// let pt = PingTracker::new(std::time::Duration::from_secs(10 * 60));
///
/// // in some loop
/// // if its been a while (such as if you have a way to keep track of time)
/// if pt.probably_timed_out() {
///     // we should reconnect
///     return Err("timed out".into());
/// }
///
/// // this might block for a while
/// let msg = read_message();
/// // update the tracker
/// pt.update(&msg);
///
/// // check to see if you should reply.
/// // this returns a message you can write to your sink
/// if let Some(pong) = pt.should_pong() {
///     io_sink.encode_msg(pong)?;
/// }
/// # Ok::<(),Box<dyn std::error::Error>>(())
/// ```
pub struct PingTracker {
    threshold: Duration,
    last: Lock<Option<Instant>>,
}

impl PingTracker {
    /// Create a new [`PingTracker`] with a 'timeout' duration
    pub const fn new(threshold: Duration) -> Self {
        Self {
            threshold,
            last: Lock::new(None),
        }
    }

    /// Get the 'timeout' duration
    pub const fn threshold(&self) -> Duration {
        self.threshold
    }

    /// Update the tracker with this message
    pub fn update(&self, msg: &Message<'_>) {
        if msg.as_typed_message::<Ping>().is_some() {
            self.last.borrow_mut().replace(Instant::now());
        }
    }

    /// Determines whether you should PONG
    ///
    /// This returns the message you should encode
    pub fn should_pong(&self) -> Option<Pong<'static>> {
        // TODO save the token
        self.last.borrow_mut().take().map(|_| pong("tmi.twitch.tv"))
    }

    /// Determines if the timeout threshold has been reached
    pub fn probably_timed_out(&self) -> bool {
        if let Some(ping) = &*self.last.borrow() {
            return ping.elapsed() >= self.threshold;
        }
        false
    }
}
