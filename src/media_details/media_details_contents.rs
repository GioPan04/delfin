use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncComponentParts, AsyncController},
    loading_widgets::LoadingWidgets,
    prelude::*,
    view, AsyncComponentSender,
};

use crate::{
    jellyfin_api::{api::item::GetItemRes, api_client::ApiClient, models::media::Media},
    media_details::{display_years::DisplayYears, seasons::SeasonsInit},
};

use super::{seasons::Seasons, MediaDetailsOutput};

pub struct MediaDetailsContents {
    item: GetItemRes,
    seasons: Option<AsyncController<Seasons>>,
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaDetailsContents {
    type Init = (Arc<ApiClient>, Media);
    type Input = ();
    type Output = MediaDetailsOutput;
    type CommandOutput = ();

    view! {
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
        _sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (api_client, media) = init;

        let mut seasons = None;

        let mut id = &media.id;
        if let Some(series_id) = &media.series_id {
            id = series_id;
            seasons = Some(
                Seasons::builder()
                    .launch(SeasonsInit {
                        api_client: api_client.clone(),
                        series_id: series_id.clone(),
                    })
                    .detach(),
            );
        }

        let item = api_client.get_item(id).await.unwrap();

        let model = MediaDetailsContents { item, seasons };

        let widgets = view_output!();
        let info_box = &widgets.info_box;

        add_info(info_box, &model.item);

        if let Some(seasons) = &model.seasons {
            root.append(seasons.widget());
        }

        AsyncComponentParts { model, widgets }
    }
}

fn add_info(info_box: &gtk::Box, item: &GetItemRes) {
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
