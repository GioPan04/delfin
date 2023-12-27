use anyhow::{anyhow, Context, Result};
use gtk::prelude::*;
use relm4::gtk;

use super::main_window::get_main_window;

pub struct InhibitCookie(u32);

impl InhibitCookie {
    pub fn new() -> Result<Self> {
        let main_window = get_main_window().context("Failed to get main window")?;
        match relm4::main_application().inhibit(
            Some(&main_window),
            gtk::ApplicationInhibitFlags::IDLE,
            Some("Playing media"),
        ) {
            0 => Err(anyhow!("Failed to inhibit")),
            cookie => Ok(Self(cookie)),
        }
    }
}

impl Drop for InhibitCookie {
    fn drop(&mut self) {
        relm4::main_application().uninhibit(self.0);
    }
}
