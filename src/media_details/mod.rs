use std::sync::{Arc, RwLock};

use adw::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    prelude::*,
};

use crate::{
    borgar::borgar_menu::BorgarMenu,
    config::{Account, Config, Server},
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
    borgar_menu: Controller<BorgarMenu>,
    media_details_contents: AsyncController<MediaDetailsContents>,
}

#[relm4::component(pub)]
impl SimpleComponent for MediaDetails {
    type Init = (
        Arc<ApiClient>,
        BaseItemDto,
        Arc<RwLock<Config>>,
        Server,
        Account,
    );
    type Input = ();
    type Output = ();

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_css_class: "media-details",

                add_top_bar = &adw::HeaderBar {
                    pack_end = model.borgar_menu.widget(),
                },

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
        let (api_client, media, config, server, account) = init;

        root.set_title(
            &media
                .series_name
                .clone()
                .or(media.name.clone())
                .unwrap_or("Unnamed Item".to_string()),
        );

        let media_details_contents = MediaDetailsContents::builder()
            .launch((api_client.clone(), media))
            .detach();

        let model = MediaDetails {
            borgar_menu: BorgarMenu::builder()
                .launch((api_client, config, server, account))
                .detach(),
            media_details_contents,
        };

        let widgets = view_output!();
        let container = &widgets.container;

        container.set_child(Some(model.media_details_contents.widget()));

        ComponentParts { model, widgets }
    }
}
