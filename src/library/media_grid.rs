use std::sync::Arc;

use gtk::prelude::*;
use relm4::{factory::FactoryVecDeque, gtk, Component, ComponentParts, ComponentSender};

use crate::jellyfin_api::{api_client::ApiClient, models::media::Media};

use super::media_tile::MediaTile;

pub enum MediaGridType {
    Latest,
}

pub struct MediaGrid {
    api_client: Arc<ApiClient>,
    id: String,
    _grid_type: MediaGridType,
    media_tiles: FactoryVecDeque<MediaTile>,
}

#[derive(Debug)]
pub enum MediaGridInput {
    MediaSelected(Media),
}

#[derive(Debug)]
pub enum MediaGridOutput {
    MediaSelected(Media),
    Empty,
}

#[derive(Debug)]
pub enum MediaGridCommandOutput {
    MediaLoaded(Vec<Media>),
}

#[relm4::component(pub)]
impl Component for MediaGrid {
    type Init = (Arc<ApiClient>, String, MediaGridType);
    type Input = MediaGridInput;
    type Output = MediaGridOutput;
    type CommandOutput = MediaGridCommandOutput;

    view! {
        gtk::Box {
            #[local_ref]
            media_grid -> gtk::Grid {
                set_column_spacing: 16,
                set_column_homogeneous: true,
                set_halign: gtk::Align::Start,
                set_margin_bottom: 12,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let media_tiles = FactoryVecDeque::new(gtk::Grid::default(), sender.input_sender());

        let model = MediaGrid {
            api_client: init.0,
            id: init.1,
            _grid_type: init.2,
            media_tiles,
        };

        let media_grid = model.media_tiles.widget();
        let widgets = view_output!();

        // Initial fetch
        model.fetch_media(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            MediaGridInput::MediaSelected(media) => sender
                .output(MediaGridOutput::MediaSelected(media))
                .unwrap(),
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            MediaGridCommandOutput::MediaLoaded(latest_media) => {
                if latest_media.is_empty() {
                    root.set_visible(false);
                    sender.output(MediaGridOutput::Empty).unwrap();
                    return;
                }
                for media in latest_media {
                    self.media_tiles.guard().push_back(media);
                }
            }
        }
    }
}

impl MediaGrid {
    fn fetch_media(&self, sender: &ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        let id = self.id.clone();
        sender.oneshot_command(async move {
            let latest_media = api_client
                .get_latest_media(&id, None)
                .await
                .expect("Error getting latest media.");
            MediaGridCommandOutput::MediaLoaded(latest_media)
        });
    }
}
