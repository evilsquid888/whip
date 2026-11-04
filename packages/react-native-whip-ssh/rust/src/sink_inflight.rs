//! In-flight tracking for foreign event sinks.
//!
//! Teardown waits for the in-flight count to reach zero before reporting the
//! bridge detached. The count must drop even if a sink callback panics, or a
//! later teardown would spin forever.

use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) struct InflightGuard(&'static AtomicU64);

impl InflightGuard {
    pub(crate) fn enter(counter: &'static AtomicU64) -> Self {
        counter.fetch_add(1, Ordering::SeqCst);
        Self(counter)
    }
}

impl Drop for InflightGuard {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::SeqCst);
    }
}
