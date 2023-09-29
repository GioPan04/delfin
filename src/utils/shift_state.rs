use gtk::gdk;
use gtk::glib;
use relm4::gtk;

use crate::globals::SHIFT_STATE;

#[derive(Default)]
pub struct ShiftState {
    pub left: bool,
    pub right: bool,
}

impl ShiftState {
    pub fn pressed(&self) -> bool {
        self.left || self.right
    }
}

pub fn shift_state_controller() -> gtk::EventControllerKey {
    let controller = gtk::EventControllerKey::new();

    controller.connect_key_pressed(|_, key, _, _| {
        match key {
            gdk::Key::Shift_L => {
                let mut shift_state = SHIFT_STATE.write();
                shift_state.left = true;
            }
            gdk::Key::Shift_R => {
                let mut shift_state = SHIFT_STATE.write();
                shift_state.right = true;
            }
            _ => {}
        };
        glib::Propagation::Proceed
    });

    controller.connect_key_released(|_, key, _, _| {
        match key {
            gdk::Key::Shift_L => {
                let mut shift_state = SHIFT_STATE.write();
                shift_state.left = false;
            }
            gdk::Key::Shift_R => {
                let mut shift_state = SHIFT_STATE.write();
                shift_state.right = false;
            }
            _ => {}
        };
    });

    controller
}
