use std::{cell::OnceCell, sync::Arc};

use gtk::prelude::*;
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController},
    loading_widgets::LoadingWidgets,
    prelude::*,
    view, AsyncComponentSender,
};
use uuid::Uuid;

use crate::{
    jellyfin_api::api_client::ApiClient,
    media_details::{media_details_header::MediaDetailsHeaderInit, seasons::SeasonsInit},
};

use super::{
    display_years::DisplayYears,
    media_details_header::{MediaDetailsHeader, MediaDetailsHeaderInput, MediaDetailsHeaderOutput},
    seasons::{Seasons, SeasonsOutput},
};

pub struct MediaDetailsContents {
    api_client: Arc<ApiClient>,
    item: BaseItemDto,
    series_id: Option<Uuid>,
    header: OnceCell<AsyncController<MediaDetailsHeader>>,
    seasons: Option<AsyncController<Seasons>>,
    selected_season_index: Option<usize>,
}

#[derive(Debug)]
pub enum MediaDetailsContentsInput {
    RefreshSeasons,
    SetSelectedSeasonIndex(usize),
    UpdatePlayNext,
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaDetailsContents {
    type Init = (Arc<ApiClient>, BaseItemDto);
    type Input = MediaDetailsContentsInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            adw::Clamp {
                set_maximum_size: 500,
                set_margin_bottom: 32,

                #[name = "container"]
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 16,

                    #[name = "info_box"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_halign: gtk::Align::Center,
                        set_spacing: 8,
                    },

                    gtk::Label::new(model.item.overview.as_deref()) {
                        set_halign: gtk::Align::Fill,
                        set_justify: gtk::Justification::Fill,
                        set_wrap: true,
                    },
                }
            },
        }
    }

    fn init_loading_widgets(root: Self::Root) -> Option<LoadingWidgets> {
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
                    set_width_request: 48,
                    set_height_request: 48,
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
        let (api_client, media) = init;

        let series_id = media
            .series_id
            .or(if let Some(BaseItemKind::Series) = media.type_ {
                media.id
            } else {
                None
            });

        let item = api_client
            .get_item(&media.series_id.or(media.id).unwrap())
            .await
            .unwrap();

        let mut model = MediaDetailsContents {
            api_client: api_client.clone(),
            item: item.clone(),
            series_id,
            header: OnceCell::new(),
            seasons: None,
            selected_season_index: None,
        };

        let widgets = view_output!();
        let container = &widgets.container;
        let info_box = &widgets.info_box;

        let header = MediaDetailsHeader::builder()
            .launch(MediaDetailsHeaderInit {
                api_client,
                media,
                item,
            })
            .forward(sender.input_sender(), |m| m.into());
        root.prepend(header.widget());
        model.header.set(header).unwrap();

        model.add_info(info_box);

        model.load_seasons(&sender, container);

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaDetailsContentsInput::RefreshSeasons => {
                self.load_seasons(&sender, &widgets.container);
            }
            MediaDetailsContentsInput::SetSelectedSeasonIndex(selected_season_index) => {
                self.selected_season_index = Some(selected_season_index);
            }
            MediaDetailsContentsInput::UpdatePlayNext => {
                if let Some(header) = self.header.get() {
                    header.emit(MediaDetailsHeaderInput::UpdatePlayNext);
                }
            }
        }
        self.update_view(widgets, sender);
    }
}

impl MediaDetailsContents {
    fn load_seasons(&mut self, sender: &AsyncComponentSender<Self>, container: &gtk::Box) {
        if let Some(seasons) = self.seasons.take() {
            container.remove(seasons.widget());
        }

        if let Some(series_id) = self.series_id {
            let seasons = Seasons::builder()
                .launch(SeasonsInit {
                    api_client: self.api_client.clone(),
                    series_id,
                    initial_selected_season_index: self.selected_season_index,
                })
                .forward(sender.input_sender(), |m| m.into());
            container.append(seasons.widget());
            self.seasons = Some(seasons);
        }
    }

    fn add_info(&self, info_box: &gtk::Box) {
        let item = &self.item;

        let mut first = true;

        let mut add_separator = move |skip_separator: bool| {
            if !first && !skip_separator {
                info_box.append(&gtk::Separator::new(gtk::Orientation::Vertical));
            }
            first = false;
        };

        if let Some(display_years) = item.display_years() {
            add_separator(true);
            info_box.append(&gtk::Label::new(Some(display_years.as_ref())));
        }

        if let Some(community_rating) = item.community_rating {
            add_separator(false);

            let rating_box = gtk::Box::new(gtk::Orientation::Horizontal, 4);
            rating_box.append(&gtk::Image::from_icon_name("star-large"));
            rating_box.append(&gtk::Label::new(Some(
                format!("{community_rating:.1}").as_ref(),
            )));

            info_box.append(&rating_box);
        }

        if let Some(official_rating) = &item.official_rating {
            add_separator(false);
            info_box.append(
                &gtk::Label::builder()
                    .label(official_rating)
                    .css_classes(["official-rating"])
                    .build(),
            );
        }

        if let Some(genres) = &item.genres.clone() {
            add_separator(false);

            // Only show first 2 genres to avoid overflow
            let mut genres_str: String = genres
                .iter()
                .take(2)
                .cloned()
                .collect::<Vec<String>>()
                .join(", ");
            if genres.len() > 2 {
                genres_str += ", ...";
            }

            let genres_label = &gtk::Label::new(Some(genres_str.as_ref()));
            // Show full list in tooltip in case any were truncated
            genres_label.set_tooltip_text(Some(genres.to_vec().join(", ").as_str()));
            info_box.append(genres_label);
        }
    }
}

impl From<SeasonsOutput> for MediaDetailsContentsInput {
    fn from(val: SeasonsOutput) -> Self {
        match val {
            SeasonsOutput::SeasonSelected(selected_season_index) => {
                MediaDetailsContentsInput::SetSelectedSeasonIndex(selected_season_index)
            }
        }
    }
}

impl From<MediaDetailsHeaderOutput> for MediaDetailsContentsInput {
    fn from(_: MediaDetailsHeaderOutput) -> Self {
        Self::RefreshSeasons
    }
}
