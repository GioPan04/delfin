use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::models::collection_type::CollectionType,
    tr,
};

use super::media_tile::MediaTileDisplay;

pub struct MediaButton;

#[relm4::component(pub)]
impl SimpleComponent for MediaButton {
    type Init = (BaseItemDto, MediaTileDisplay);
    type Input = ();
    type Output = ();

    view! {
        gtk::Button {
            add_css_class: "pill",
            set_width_request: display.width().into(),
            set_margin_bottom: 10,

            #[wrap(Some)]
            set_child = &adw::ButtonContent {
                set_label: media.name.as_ref().unwrap_or(tr!("library-unnamed-collection")),
                set_icon_name: &collection_type.icon(),
                set_halign: gtk::Align::Center,
            },

            connect_clicked[media] => move |_| {
                APP_BROKER.send(AppInput::ShowCollection(media.clone()));
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let (media, display) = init;

        let collection_type: CollectionType = media.clone().collection_type.into();

        let model = MediaButton;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
