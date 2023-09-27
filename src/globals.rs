use relm4::{once_cell::sync::Lazy, SharedState};

use crate::config::Config;

pub static CONFIG: Lazy<SharedState<Config>> = Lazy::new(|| {
    let state = SharedState::new();
    *state.write() = Config::new().expect("Error loading config.");
    state
});
