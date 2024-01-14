use relm4::{once_cell::sync::Lazy, SharedState};

use crate::{config::Config, utils::shift_state::ShiftState};

pub static CONFIG: Lazy<SharedState<Config>> = Lazy::new(|| {
    let state = SharedState::new();
    *state.write() = Config::new().expect("Failed to load config");
    state
});

pub static SHIFT_STATE: Lazy<SharedState<ShiftState>> = Lazy::new(|| {
    let state = SharedState::new();
    *state.write() = ShiftState::default();
    state
});
