use std::{
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard},
};

use relm4::MessageBroker;

pub struct ResettableMessageBroker<T: Debug>(RwLock<MessageBroker<T>>);

impl<T: Debug> Default for ResettableMessageBroker<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Debug> ResettableMessageBroker<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self(RwLock::new(MessageBroker::new()))
    }

    pub fn read(&self) -> RwLockReadGuard<MessageBroker<T>> {
        self.0.read().expect("Error reading control broker")
    }

    pub fn send(&self, input: T) {
        self.read().send(input);
    }

    pub fn reset(&self) {
        *self.0.write().expect("Error resetting control broker") = MessageBroker::new();
    }
}
