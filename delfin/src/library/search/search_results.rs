use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use jellyfin_api::types::BaseItemDto;
use relm4::{gtk::traits::BoxExt, prelude::*};

use crate::{
    jellyfin_api::api_client::ApiClient,
    library::{
        media_fetcher::Fetcher,
        media_page::{MediaPage, MediaPageInit, MediaPageInput},
    },
    tr,
};

pub struct SearchResults {
    api_client: Arc<ApiClient>,
    media_page: Option<Controller<MediaPage<SearchResultsFetcher, SearchResultsEmpty>>>,
}

#[derive(Debug)]
pub enum SearchResultsInput {
    SearchChanged(String),
}

#[relm4::component(pub)]
impl Component for SearchResults {
    type Init = Arc<ApiClient>;
    type Input = SearchResultsInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {}
    }

    fn init(
        api_client: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResults {
            api_client,
            media_page: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            SearchResultsInput::SearchChanged(search_text) => {
                if let Some(media_page) = self.media_page.take() {
                    root.remove(media_page.widget());
                }

                let fetcher = SearchResultsFetcher {
                    api_client: self.api_client.clone(),
                    search_text: search_text.clone(),
                };

                let media_page = MediaPage::builder()
                    .launch(MediaPageInit {
                        api_client: self.api_client.clone(),
                        fetcher,
                        empty_component: Some(SearchResultsEmpty::builder().launch(()).detach()),
                    })
                    .detach();

                if !search_text.is_empty() {
                    media_page.emit(MediaPageInput::NextPage);
                }

                root.append(media_page.widget());

                self.media_page = Some(media_page);
            }
        }

        self.update_view(widgets, sender);
    }
}

struct SearchResultsFetcher {
    api_client: Arc<ApiClient>,
    search_text: String,
}

#[async_trait]
impl Fetcher for SearchResultsFetcher {
    async fn fetch(&self, start_index: usize, limit: usize) -> Result<(Vec<BaseItemDto>, usize)> {
        self.api_client
            .search_items(&self.search_text, start_index, limit)
            .await
    }

    fn title(&self) -> String {
        tr!("library-search-title", {
             "searchText" => self.search_text.clone(),
        })
        .to_string()
    }
}

struct SearchResultsEmpty;

#[relm4::component]
impl SimpleComponent for SearchResultsEmpty {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::StatusPage {
            set_icon_name: Some("loupe"),
            set_title: tr!("library-search-empty.title"),
            set_description: Some(tr!("library-search-empty.description")),
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResultsEmpty;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
