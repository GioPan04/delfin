use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    loading_widgets::LoadingWidgets,
    prelude::*,
    view, AsyncComponentSender,
};

use crate::{jellyfin_api::api_client::ApiClient, media_details::season_buttons::SeasonButtons};

pub struct Seasons {
    _season_buttons: Option<Controller<SeasonButtons>>,
}

pub struct SeasonsInit {
    pub api_client: Arc<ApiClient>,
    pub series_id: String,
}

#[derive(Debug)]
pub enum SeasonsInput {
    SeasonActivated(usize),
}

#[relm4::component(pub async)]
impl AsyncComponent for Seasons {
    type Init = SeasonsInit;
    type Input = SeasonsInput;
    type Output = ();
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
        } = init;

        let mut model = Seasons {
            _season_buttons: None,
        };

        let widgets = view_output!();

        let seasons = api_client.get_seasons(&series_id).await.unwrap();

        if seasons.is_empty() {
            // This probably shouldn't happen
            return AsyncComponentParts { model, widgets };
        }

        let season_buttons = SeasonButtons::builder()
            .launch(seasons)
            .forward(sender.input_sender(), |e| e);
        root.append(season_buttons.widget());
        model._season_buttons = Some(season_buttons);

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        msg: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match msg {
            SeasonsInput::SeasonActivated(index) => {
                println!("season activated {:#?}", index);
            }
        }
    }
}
