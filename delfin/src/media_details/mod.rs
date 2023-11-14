use std::sync::{Arc, RwLock};

use adw::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    prelude::*,
    SharedState,
};

use crate::{
    borgar::borgar_menu::BorgarMenu,
    config::{Account, Config, Server},
    jellyfin_api::api_client::ApiClient,
    media_details::media_details_contents::MediaDetailsContents,
    tr,
};

use self::media_details_contents::MediaDetailsContentsInput;

mod display_years;
pub mod episode;
mod episodes;
mod media_details_contents;
mod media_details_header;
mod season_buttons;
mod seasons;

pub static MEDIA_DETAILS_REFRESH_QUEUED: SharedState<bool> = SharedState::new();

pub struct MediaDetails {
    borgar_menu: Controller<BorgarMenu>,
    media_details_contents: AsyncController<MediaDetailsContents>,
}

#[derive(Debug)]
pub enum MediaDetailsInput {
    Shown,
    Refresh,
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
    type Input = MediaDetailsInput;
    type Output = ();

    view! {
        adw::NavigationPage {
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_css_class: "media-details",

                add_top_bar = &adw::HeaderBar {
                    pack_end = model.borgar_menu.widget(),
                    pack_end = &gtk::Button::from_icon_name("refresh") {
                        set_tooltip: tr!("media-details-refresh-button"),
                        connect_clicked[sender] => move |_| {
                            sender.input(MediaDetailsInput::Refresh);
                        },
                    },
                },

                #[name = "container"]
                #[wrap(Some)]
                set_content = &gtk::ScrolledWindow {
                    set_hexpand: true,
                    set_vexpand: true,
                },
            },

            connect_shown[sender] => move |_| {
                sender.input(MediaDetailsInput::Shown);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
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

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            MediaDetailsInput::Refresh => {
                self.media_details_contents
                    .emit(MediaDetailsContentsInput::RefreshSeasons);
            }
            MediaDetailsInput::Shown => {
                if *MEDIA_DETAILS_REFRESH_QUEUED.read() {
                    sender.input(MediaDetailsInput::Refresh);
                }
                *MEDIA_DETAILS_REFRESH_QUEUED.write() = false;
            }
        }
    }
}
