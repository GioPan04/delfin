use std::sync::Arc;

use adw::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    loading_widgets::LoadingWidgets,
    prelude::*,
    view, AsyncComponentSender,
};
use uuid::Uuid;

use crate::{
    jellyfin_api::{api::shows::GetEpisodesOptionsBuilder, api_client::ApiClient},
    tr,
};

use super::episode::Episode;

pub(crate) struct Episodes {
    episodes: Vec<BaseItemDto>,
    episode_components: Vec<AsyncController<Episode>>,
}

pub(crate) struct EpisodesInit {
    pub(crate) api_client: Arc<ApiClient>,
    pub(crate) series_id: Uuid,
    pub(crate) season: BaseItemDto,
}

#[relm4::component(pub(crate) async)]
impl AsyncComponent for Episodes {
    type Init = EpisodesInit;
    type Input = ();
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[name = "episode_list"]
            gtk::ListBox {
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,
            },

            // Empty state
            gtk::ListBox {
                add_css_class: "boxed-list",
                set_selection_mode: gtk::SelectionMode::None,
                #[watch]
                set_visible: model.episodes.is_empty(),
                adw::ActionRow {
                    set_title: tr!("media-details-episode-list-empty"),
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let EpisodesInit {
            api_client,
            series_id,
            season,
        } = init;

        let episodes = api_client
            .get_episodes(
                &GetEpisodesOptionsBuilder::default()
                    .series_id(series_id)
                    .season_id(season.id.unwrap())
                    .build()
                    .unwrap(),
            )
            .await
            .unwrap();

        let mut model = Episodes {
            episodes,
            episode_components: vec![],
        };

        let widgets = view_output!();
        let episode_list = &widgets.episode_list;

        for episode in &model.episodes {
            let e = Episode::builder()
                .launch((episode.clone(), api_client.clone()))
                .detach();
            episode_list.append(e.widget());
            model.episode_components.push(e);
        }

        AsyncComponentParts { model, widgets }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
        view! {
            #[local_ref]
            root {
                #[name(spinner)]
                gtk::ListBox {
                    add_css_class: "boxed-list",
                    set_selection_mode: gtk::SelectionMode::None,
                    set_hexpand: true,

                    gtk::ListBoxRow {
                        gtk::Spinner {
                            set_spinning: true,
                            set_halign: gtk::Align::Center,
                            set_valign: gtk::Align::Center,
                            set_hexpand: true,
                            set_vexpand: true,
                            set_width_request: 24,
                            set_height_request: 24,
                            set_margin_top: 32,
                            set_margin_bottom: 32,
                        }
                    },
                },
            }
        }
        Some(LoadingWidgets::new(root, spinner))
    }
}
