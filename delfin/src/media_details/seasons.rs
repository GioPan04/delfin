use jellyfin_api::types::BaseItemDto;
use relm4::component::AsyncComponentController;
use std::sync::Arc;
use uuid::Uuid;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts, AsyncController},
    loading_widgets::LoadingWidgets,
    prelude::*,
    view, AsyncComponentSender,
};

use crate::{jellyfin_api::api_client::ApiClient, media_details::season_buttons::SeasonButtons};

use super::episodes::{Episodes, EpisodesInit};

pub struct Seasons {
    api_client: Arc<ApiClient>,
    series_id: Uuid,
    seasons: Vec<BaseItemDto>,
    season_buttons: Option<Controller<SeasonButtons>>,
    episodes: Option<AsyncController<Episodes>>,
}

pub struct SeasonsInit {
    pub api_client: Arc<ApiClient>,
    pub series_id: Uuid,
    pub initial_selected_season_index: Option<usize>,
}

#[derive(Debug)]
pub enum SeasonsInput {
    SeasonActivated(usize),
}

#[derive(Debug)]
pub enum SeasonsOutput {
    SeasonSelected(usize),
}

#[relm4::component(pub async)]
impl AsyncComponent for Seasons {
    type Init = SeasonsInit;
    type Input = SeasonsInput;
    type Output = SeasonsOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,
        }
    }

    fn init_loading_widgets(root: &mut Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                #[name(spinner)]
                gtk::Spinner {
                    set_spinning: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_hexpand: true,
                    set_vexpand: true,
                    set_width_request: 24,
                    set_height_request: 24,
                }
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let SeasonsInit {
            api_client,
            series_id,
            initial_selected_season_index,
        } = init;

        let widgets = view_output!();

        let seasons = api_client.get_seasons(&series_id).await.unwrap();

        let mut model = Seasons {
            api_client,
            series_id,
            seasons,
            season_buttons: None,
            episodes: None,
        };

        if model.seasons.is_empty() {
            // This probably shouldn't happen
            return AsyncComponentParts { model, widgets };
        }

        let initial_selected_season_index = match initial_selected_season_index {
            Some(season) if season < model.seasons.len() => season,
            _ => 0,
        };

        let season_buttons = SeasonButtons::builder()
            .launch((model.seasons.clone(), initial_selected_season_index))
            .forward(sender.input_sender(), |e| e);
        root.append(season_buttons.widget());
        model.season_buttons = Some(season_buttons);

        sender.input(SeasonsInput::SeasonActivated(initial_selected_season_index));

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        sender: AsyncComponentSender<Self>,
        root: &Self::Root,
    ) {
        match msg {
            SeasonsInput::SeasonActivated(index) => {
                if let Some(episodes) = &self.episodes {
                    root.remove(episodes.widget());
                }

                let season = self.seasons[index].clone();

                let episodes = Episodes::builder()
                    .launch(EpisodesInit {
                        api_client: self.api_client.clone(),
                        series_id: self.series_id,
                        season,
                    })
                    .detach();
                root.append(episodes.widget());
                self.episodes = Some(episodes);

                sender.output(SeasonsOutput::SeasonSelected(index)).unwrap();
            }
        }
    }
}
