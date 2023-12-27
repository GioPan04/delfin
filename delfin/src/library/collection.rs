use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    jellyfin_api::{api::views::UserView, api_client::ApiClient},
    library::media_page::{MediaPage, MediaPageInput},
    utils::empty_component::EmptyComponent,
};

use super::{media_fetcher::Fetcher, media_page::MediaPageInit};

pub struct Collection {
    media_page: Controller<MediaPage<ViewItemsFetcher, EmptyComponent>>,
    initialized: bool,
}

#[derive(Debug)]
pub enum CollectionInput {
    Visible,
}

#[relm4::component(pub)]
impl SimpleComponent for Collection {
    type Init = (Arc<ApiClient>, UserView);
    type Input = CollectionInput;
    type Output = ();

    view! {
        gtk::Box {}
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, view) = init;

        let fetcher = ViewItemsFetcher {
            api_client: api_client.clone(),
            view,
        };

        let model = Collection {
            media_page: MediaPage::builder()
                .launch(MediaPageInit {
                    api_client,
                    fetcher,
                    empty_component: None,
                })
                .detach(),
            initialized: false,
        };
        root.append(model.media_page.widget());

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            CollectionInput::Visible if !self.initialized => {
                self.initialized = true;
                self.media_page.emit(MediaPageInput::NextPage);
            }
            _ => {}
        }
    }
}

struct ViewItemsFetcher {
    api_client: Arc<ApiClient>,
    view: UserView,
}

#[async_trait]
impl Fetcher for ViewItemsFetcher {
    async fn fetch(&self, start_index: usize, limit: usize) -> Result<(Vec<BaseItemDto>, usize)> {
        self.api_client
            .get_view_items(&self.view, start_index, limit)
            .await
    }

    fn title(&self) -> String {
        self.view.name.clone()
    }
}
