use relm4::prelude::*;

use crate::utils::constants::{MAX_LIBRARY_WIDTH, PAGE_MARGIN};

#[relm4::widget_template(pub)]
impl WidgetTemplate for LibraryContainer {
    view! {
        gtk::ScrolledWindow {
            #[name = "contents"]
            adw::Clamp {
                set_maximum_size: MAX_LIBRARY_WIDTH,
                set_tightening_threshold: MAX_LIBRARY_WIDTH,
                set_margin_all: PAGE_MARGIN,
            },
        }
    }
}
