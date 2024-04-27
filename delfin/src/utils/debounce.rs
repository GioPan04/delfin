use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use tokio::time::sleep;
use tracing::debug;

type DebounceCallback = Arc<Mutex<Box<dyn Fn() + Send>>>;

pub struct Debounce {
    timeout: Duration,
    id: Arc<AtomicUsize>,
    callback: DebounceCallback,
}

impl Debounce {
    #[must_use]
    pub fn new(timeout: Duration, callback: Box<dyn Fn() + Send>) -> Self {
        Self {
            timeout,
            id: Arc::new(AtomicUsize::new(0)),
            callback: Arc::new(Mutex::new(callback)),
        }
    }

    pub fn debounce(&mut self) {
        // fetch_add returns previous value, add 1 for current value
        let cur_id = self.id.fetch_add(1, Ordering::Relaxed) + 1;

        tokio::spawn({
            let timeout = self.timeout;
            let callback = self.callback.clone();
            let id = self.id.clone();
            async move {
                sleep(timeout).await;

                // Check if this is still the most recent callback
                if id.load(Ordering::Relaxed) != cur_id {
                    return;
                }

                if let Ok(callback) = callback.try_lock() {
                    callback();
                } else {
                    debug!("Debounce callback lock could not be acquired");
                }
            }
        });
    }
}
