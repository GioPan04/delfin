use std::sync::Arc;

use adw::prelude::BoxExt;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    prelude::*,
    AsyncComponentSender, Controller,
};
use uuid::Uuid;

use crate::jellyfin_api::{
    api::latest::GetNextUpOptions, api_client::ApiClient, models::user_view::UserView,
};

use super::{
    media_carousel::{MediaCarousel, MediaCarouselInit, MediaCarouselOutput, MediaCarouselType},
    media_tile::MediaTileDisplay,
};

enum MediaListContents {
    // Grid(Controller<MediaGrid>),
    Carousel(Controller<MediaCarousel>),
}

#[derive(Clone, Debug)]
pub enum MediaListType {
    ContinueWatching,
    Latest(MediaListTypeLatestParams),
    NextUp,
    MyMedia {
        user_views: Vec<UserView>,
        small: bool,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct MediaListTypeLatestParams {
    pub view_id: Uuid,
}

pub struct MediaList {
    _contents: MediaListContents,
}

pub struct MediaListInit {
    pub list_type: MediaListType,
    pub api_client: Arc<ApiClient>,
    pub label: String,
    pub label_clickable: bool,
}

#[derive(Debug)]
pub enum MediaListOutput {
    Empty(Option<Uuid>),
    LabelClicked(Option<Uuid>),
}

#[relm4::component(pub async)]
impl AsyncComponent for MediaList {
    type Init = MediaListInit;
    type Input = ();
    type Output = MediaListOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {}
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> where {
        let model = MediaList::new(&init, &root, &sender).await;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }
}

impl MediaList {
    async fn new(
        init: &MediaListInit,
        root: &gtk::Box,
        sender: &AsyncComponentSender<Self>,
    ) -> Self {
        let api_client = Arc::clone(&init.api_client);
        let list_type = &init.list_type;
        let label = init.label.clone();
        let label_clickable = init.label_clickable;

        let view_id = list_type.view_id();

        let media = match list_type {
            MediaListType::ContinueWatching => api_client
                .get_continue_watching(GetNextUpOptions::default())
                .await
                .expect("Error getting continue watching."),
            MediaListType::Latest(params) => api_client
                .get_latest_media(&params.view_id, None)
                .await
                .expect("Error getting latest media."),
            MediaListType::NextUp => api_client
                .get_next_up(GetNextUpOptions::default())
                .await
                .expect("Error getting continue watching."),

            MediaListType::MyMedia {
                user_views,
                small: _,
            } => user_views
                .clone()
                .into_iter()
                .map(|view| view.into())
                .collect(),
        };
        if media.is_empty() {
            sender.output(MediaListOutput::Empty(view_id)).unwrap();
        }

        let media_tile_display = match list_type {
            MediaListType::ContinueWatching | MediaListType::NextUp => MediaTileDisplay::Wide,
            MediaListType::Latest(_) => MediaTileDisplay::Cover,
            MediaListType::MyMedia { small, .. } if *small => MediaTileDisplay::Buttons,
            MediaListType::MyMedia { .. } => MediaTileDisplay::CollectionWide,
        };

        let carousel_type = match list_type {
            MediaListType::MyMedia { small, .. } if *small => MediaCarouselType::Buttons,
            _ => MediaCarouselType::Tiles,
        };

        let contents = {
            let carousel = MediaCarousel::builder()
                .launch(MediaCarouselInit {
                    media,
                    media_tile_display,
                    carousel_type,
                    api_client,
                    label,
                    label_clickable,
                })
                .forward(sender.output_sender(), move |msg| match msg {
                    MediaCarouselOutput::LabelClicked => MediaListOutput::LabelClicked(view_id),
                });
            root.append(carousel.widget());
            MediaListContents::Carousel(carousel)
        };

        Self {
            _contents: contents,
        }
    }
}

impl MediaListType {
    fn view_id(&self) -> Option<Uuid> {
        match self {
            MediaListType::ContinueWatching
            | MediaListType::NextUp
            | MediaListType::MyMedia { .. } => None,
            MediaListType::Latest(params) => Some(params.view_id),
        }
    }
}
