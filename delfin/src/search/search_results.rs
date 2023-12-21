use std::sync::Arc;

use relm4::prelude::*;

use crate::{jellyfin_api::api_client::ApiClient, tr};

pub struct SearchResults {
    _api_client: Arc<ApiClient>,
    search_text: String,
}

#[derive(Debug)]
pub enum SearchResultsInput {
    SearchChanged(String),
}

#[relm4::component(pub)]
impl SimpleComponent for SearchResults {
    type Init = Arc<ApiClient>;
    type Input = SearchResultsInput;
    type Output = ();

    view! {
        adw::StatusPage {
            set_icon_name: Some("loupe"),
            set_title: tr!("library-search-empty.title"),
            set_description: Some(tr!("library-search-empty.description")),
        }
    }

    fn init(
        api_client: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResults {
            _api_client: api_client,
            search_text: String::default(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SearchResultsInput::SearchChanged(search_text) => {
                self.search_text = search_text;
            }
        }
    }
}
