use std::{
    fmt::Debug,
    sync::{RwLock, RwLockReadGuard},
};

use relm4::MessageBroker;

pub struct ResettableMessageBroker<T: Debug>(RwLock<MessageBroker<T>>);

impl<T: Debug> ResettableMessageBroker<T> {
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
