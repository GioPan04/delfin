use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, Component, ComponentParts, ComponentSender,
};

use crate::jellyfin_api::{
    api::latest::GetNextUpOptions, api_client::ApiClient, models::media::Media,
};

use super::media_tile::{MediaTile, MediaTileDisplay};

#[derive(Clone, Debug)]
pub enum MediaGridType {
    ContinueWatching,
    Latest(MediaGridTypeLatestParams),
    NextUp,
}

#[derive(Clone, Debug)]
pub struct MediaGridTypeLatestParams {
    pub view_id: String,
}

pub struct MediaGrid {
    api_client: Arc<ApiClient>,
    grid_type: MediaGridType,
    media_tiles: Vec<AsyncController<MediaTile>>,
}

pub struct MediaGridInit {
    pub api_client: Arc<ApiClient>,
    pub grid_type: MediaGridType,
}

#[derive(Debug)]
pub enum MediaGridOutput {
    Empty(Option<String>),
}

#[derive(Debug)]
pub enum MediaGridCommandOutput {
    MediaLoaded(Vec<Media>),
}

#[relm4::component(pub)]
impl Component for MediaGrid {
    type Init = MediaGridInit;
    type Input = ();
    type Output = MediaGridOutput;
    type CommandOutput = MediaGridCommandOutput;

    view! {
        gtk::Box {
            #[name = "media_grid"]
            gtk::Grid {
                set_column_spacing: 32,
                set_column_homogeneous: true,
                set_halign: gtk::Align::Start,
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = MediaGrid {
            api_client: init.api_client,
            grid_type: init.grid_type,
            media_tiles: vec![],
        };

        let widgets = view_output!();

        // Initial fetch
        model.fetch_media(&sender);

        ComponentParts { model, widgets }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            MediaGridCommandOutput::MediaLoaded(media) => {
                if media.is_empty() {
                    root.set_visible(false);
                    sender
                        .output(MediaGridOutput::Empty(self.get_view_id()))
                        .unwrap();
                    return;
                }

                let media_grid = &widgets.media_grid;
                let media_tile_display = match self.grid_type {
                    MediaGridType::ContinueWatching | MediaGridType::NextUp => {
                        MediaTileDisplay::Wide
                    }
                    MediaGridType::Latest(_) => MediaTileDisplay::Cover,
                };

                for (column, media) in media.into_iter().enumerate() {
                    let media_tile = MediaTile::builder()
                        .launch((media, media_tile_display))
                        .detach();
                    media_grid.attach(media_tile.widget(), column as i32, 0, 1, 1);
                    self.media_tiles.push(media_tile);
                }
            }
        }

        self.update_view(widgets, sender);
    }
}

impl MediaGrid {
    fn fetch_media(&self, sender: &ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        let grid_type = self.grid_type.clone();
        sender.oneshot_command(async move {
            let media = match &grid_type {
                MediaGridType::ContinueWatching => api_client
                    .get_continue_watching(None)
                    .await
                    .expect("Error getting continue watching."),
                MediaGridType::Latest(params) => api_client
                    .get_latest_media(&params.view_id, None)
                    .await
                    .expect("Error getting latest media."),
                MediaGridType::NextUp => api_client
                    .get_next_up(GetNextUpOptions::default())
                    .await
                    .expect("Error getting continue watching."),
            };
            MediaGridCommandOutput::MediaLoaded(media)
        });
    }

    fn get_view_id(&self) -> Option<String> {
        match &self.grid_type {
            MediaGridType::ContinueWatching | MediaGridType::NextUp => None,
            MediaGridType::Latest(params) => Some(params.view_id.clone()),
        }
    }
}
