use serde::Serialize;
use std::sync::atomic::AtomicU64;

#[derive(Serialize, Debug)]
pub(crate) struct Stats {
    pub(crate) bot_requests: AtomicU64,
    pub(crate) human_requests: AtomicU64,
    pub(crate) elements_randomized: AtomicU64,
    pub(crate) elements_removed: AtomicU64,
}

impl Stats {
    pub(crate) fn new() -> Self {
        Self {
            bot_requests: AtomicU64::new(0),
            human_requests: AtomicU64::new(0),
            elements_randomized: AtomicU64::new(0),
            elements_removed: AtomicU64::new(0),
        }
    }
}
