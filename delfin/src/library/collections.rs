use std::sync::Arc;

use anyhow::Result;
use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    jellyfin_api::{api_client::ApiClient, models::user_view::FilterSupported},
    library::{
        media_page::{MediaPageInit, MediaPageInput},
        media_tile::MediaTileDisplay,
    },
    tr,
    utils::empty_component::EmptyComponent,
};

use super::{media_fetcher::Fetcher, media_page::MediaPage};

pub struct Collections {
    media_page: Controller<MediaPage<CollectionsFetcher, EmptyComponent>>,
}

#[relm4::component(pub)]
impl SimpleComponent for Collections {
    type Init = Arc<ApiClient>;
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            append = model.media_page.widget(),
        }
    }

    fn init(
        api_client: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let fetcher = CollectionsFetcher {
            api_client: api_client.clone(),
        };

        let model = Collections {
            media_page: MediaPage::builder()
                .launch(MediaPageInit {
                    api_client,
                    fetcher,
                    empty_component: None,
                    media_tile_display: Some(MediaTileDisplay::CollectionWide),
                })
                .detach(),
        };

        model.media_page.emit(MediaPageInput::NextPage);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

struct CollectionsFetcher {
    api_client: Arc<ApiClient>,
}

impl Fetcher for CollectionsFetcher {
    async fn fetch(&self, start_index: usize, limit: usize) -> Result<(Vec<BaseItemDto>, usize)> {
        let (collections, _) = self
            .api_client
            .get_user_views(Some(start_index), Some(limit))
            .await?;
        let collections: Vec<BaseItemDto> = collections
            .filter_supported()
            .into_iter()
            .map(|view| view.into())
            .collect();
        let total = collections.len();
        Ok((collections, total))
    }

    fn title(&self) -> String {
        tr!("library-page-collections-title").to_owned()
    }
}
