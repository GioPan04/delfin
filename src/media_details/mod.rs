use std::sync::{Arc, RwLock};

use gtk::prelude::*;
use relm4::prelude::*;

use crate::{
    app::{AppInput, APP_BROKER},
    config::Config,
    jellyfin_api::{api_client::ApiClient, models::media::Media},
};

pub struct MediaDetails {
    #[allow(dead_code)]
    config: Arc<RwLock<Config>>,
    media: Media,
    #[allow(dead_code)]
    api_client: Arc<ApiClient>,
}

#[derive(Debug)]
pub enum MediaDetailsInput {
    ShowDetails(Arc<ApiClient>, Box<Media>),
}

#[derive(Debug)]
pub enum MediaDetailsOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl Component for MediaDetails {
    type Init = (Arc<RwLock<Config>>, Arc<ApiClient>, Media);
    type Input = MediaDetailsInput;
    type Output = MediaDetailsOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            adw::HeaderBar {
                set_valign: gtk::Align::Start,
                #[wrap(Some)]
                set_title_widget = &adw::WindowTitle {
                    #[watch]
                    set_title: if let Some(series_name) = &model.media.series_name {
                        series_name
                    } else {
                        &model.media.name
                    },
                },
                pack_start = &gtk::Button {
                    set_icon_name: "go-previous",
                    connect_clicked => move |_| {
                        APP_BROKER.send(AppInput::NavigateBack);
                    },
                },
            },

            gtk::Label::new(Some("media details")),
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (config, api_client, media) = init;

        let model = MediaDetails {
            config,
            media,
            api_client,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
