use std::sync::Arc;

use anyhow::Result;
use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    jellyfin_api::api_client::ApiClient,
    library::{
        media_page::{MediaPage, MediaPageInput},
        media_tile::MediaTileDisplay,
    },
    utils::empty_component::EmptyComponent,
};

use super::{media_fetcher::Fetcher, media_page::MediaPageInit};

pub struct Collection {
    media_page: Controller<MediaPage<CollectionItemsFetcher, EmptyComponent>>,
}

#[relm4::component(pub)]
impl SimpleComponent for Collection {
    type Init = (Arc<ApiClient>, BaseItemDto);
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {}
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, collection) = init;

        let fetcher = CollectionItemsFetcher {
            api_client: api_client.clone(),
            collection,
        };

        let model = Collection {
            media_page: MediaPage::builder()
                .launch(MediaPageInit {
                    api_client,
                    fetcher,
                    empty_component: None,
                    media_tile_display: Some(MediaTileDisplay::CoverLarge),
                })
                .detach(),
        };
        root.append(model.media_page.widget());

        model.media_page.emit(MediaPageInput::NextPage);

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}

struct CollectionItemsFetcher {
    api_client: Arc<ApiClient>,
    collection: BaseItemDto,
}

impl Fetcher for CollectionItemsFetcher {
    async fn fetch(&self, start_index: usize, limit: usize) -> Result<(Vec<BaseItemDto>, usize)> {
        self.api_client
            .get_collection_items(&self.collection, start_index, limit)
            .await
    }

    fn title(&self) -> String {
        self.collection
            .name
            .as_ref()
            .unwrap_or(&String::default())
            .clone()
    }
}
