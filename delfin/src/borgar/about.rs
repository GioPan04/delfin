use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, SimpleComponent};

use crate::meson_config::APP_ID;

pub(crate) struct About;

#[relm4::component(pub(crate))]
impl SimpleComponent for About {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::AboutWindow::from_appdata(
            &format!("/cafe/avery/Delfin/{}.metainfo.xml", APP_ID),
            None,
        ) {
            set_modal: true,
            set_visible: true,
            set_version: "0.0",
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let widgets = view_output!();
        let model = About;
        ComponentParts { model, widgets }
    }
}
