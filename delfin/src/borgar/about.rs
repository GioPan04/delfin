use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, SimpleComponent};

pub(crate) struct About;

#[relm4::component(pub(crate))]
impl SimpleComponent for About {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        // TODO: Load from appstream
        adw::AboutWindow {
            set_modal: true,
            set_visible: true,

            set_application_name: "Delfin",
            set_developer_name: "Avery ❤️",
            set_version: "0.0",
            set_license_type: gtk::License::Agpl30,
            set_website: "https://codeberg.org/avery42/delfin",
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
