use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    prelude::*,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::{api_client::ApiClient, models::media::Media},
    media_details::media_details_contents::MediaDetailsContents,
};

mod display_years;
pub mod episode;
mod episodes;
mod media_details_contents;
mod season_buttons;
mod seasons;

pub struct MediaDetails {
    _media_details_contents: AsyncController<MediaDetailsContents>,
}

#[derive(Debug)]
pub enum MediaDetailsOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl SimpleComponent for MediaDetails {
    type Init = (Arc<ApiClient>, Media);
    type Input = ();
    type Output = MediaDetailsOutput;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "media-details",

            adw::HeaderBar {
                set_valign: gtk::Align::Start,
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    set_title: title,
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous",
                    connect_clicked => move |_| {
                        APP_BROKER.send(AppInput::NavigateBack);
                    },
                },
            },

            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,

                #[name = "container"]
                adw::Clamp {
                    set_maximum_size: 500,
                    set_margin_bottom: 32,
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, media) = init;

        let title = &media.series_name.as_ref().unwrap_or(&media.name);

        let widgets = view_output!();
        let container = &widgets.container;

        let media_details_contents = MediaDetailsContents::builder()
            .launch((api_client, media))
            .detach();
        container.set_child(Some(media_details_contents.widget()));

        let model = MediaDetails {
            _media_details_contents: media_details_contents,
        };

        ComponentParts { model, widgets }
    }
}
