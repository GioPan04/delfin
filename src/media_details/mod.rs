use std::sync::Arc;

use adw::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    prelude::*,
};

use crate::{
    jellyfin_api::api_client::ApiClient,
    media_details::media_details_contents::MediaDetailsContents,
};

mod display_years;
pub mod episode;
mod episodes;
mod media_details_contents;
mod media_details_header;
mod season_buttons;
mod seasons;

pub struct MediaDetails {
    _media_details_contents: AsyncController<MediaDetailsContents>,
}

#[relm4::component(pub)]
impl SimpleComponent for MediaDetails {
    type Init = (Arc<ApiClient>, BaseItemDto);
    type Input = ();
    type Output = ();

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_css_class: "media-details",

                add_top_bar = &adw::HeaderBar {},

                #[name = "container"]
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hexpand: true,
                    set_vexpand: true,
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

        root.set_title(
            &media
                .series_name
                .clone()
                .or(media.name.clone())
                .unwrap_or("Unnamed Item".to_string()),
        );

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
