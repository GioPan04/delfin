use gst::prelude::Cast;
use gtk::prelude::ListModelExt;
use relm4::{
    adw,
    gtk::{self, traits::WidgetExt},
};

pub const MAIN_APP_WINDOW_NAME: &str = "main_app_window";

pub fn get_main_window() -> Option<adw::ApplicationWindow> {
    let toplevels = gtk::Window::toplevels();
    for i in 0..toplevels.n_items() {
        if let Some(window) = toplevels.item(i) {
            if let Ok(window) = window.downcast::<adw::ApplicationWindow>() {
                if window.widget_name().as_str() == MAIN_APP_WINDOW_NAME {
                    return Some(window);
                }
            }
        }
    }
    None
}
