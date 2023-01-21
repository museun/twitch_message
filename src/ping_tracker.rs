use std::time::{Duration, Instant};

use crate::{
    encode::{pong, Pong},
    lock::Lock,
    messages::Ping,
    Message,
};

pub struct PingTracker {
    threshold: Duration,
    last: Lock<Option<Instant>>,
}

impl PingTracker {
    pub const fn new(threshold: Duration) -> Self {
        Self {
            threshold,
            last: Lock::new(None),
        }
    }

    pub const fn threshold(&self) -> Duration {
        self.threshold
    }

    pub fn update(&self, msg: &Message<'_>) {
        if msg.as_typed_message::<Ping>().is_some() {
            self.last.borrow_mut().replace(Instant::now());
        }
    }

    pub fn should_pong(&self) -> Option<Pong<'static>> {
        self.last.borrow_mut().take().map(|_| pong("tmi.twitch.tv"))
    }

    pub fn probably_timed_out(&self) -> bool {
        if let Some(ping) = &*self.last.borrow() {
            return ping.elapsed() >= self.threshold;
        }
        false
    }
}
