use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{adw::traits::ActionRowExt, gtk::traits::BoxExt, prelude::*};

use crate::{
    app::{AppInput, APP_BROKER},
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
                        empty_component: Some(
                            SearchResultsEmpty::builder()
                                .launch(self.api_client.clone())
                                .detach(),
                        ),
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

#[derive(Debug)]
enum SearchResultsEmptyCommandOutput {
    Suggestions(Vec<BaseItemDto>),
}

#[relm4::component]
impl Component for SearchResultsEmpty {
    type Init = Arc<ApiClient>;
    type Input = ();
    type Output = ();
    type CommandOutput = SearchResultsEmptyCommandOutput;

    view! {
        adw::Clamp {
            set_maximum_size: 400,
            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::StatusPage {
                    set_icon_name: Some("loupe"),
                    set_title: tr!("library-search-empty.title"),
                    set_description: Some(tr!("library-search-empty.description")),

                    #[name = "suggestions"]
                    #[wrap(Some)]
                    set_child = &gtk::ListBox {
                        set_visible: false,
                        add_css_class: "boxed-list",
                        set_selection_mode: gtk::SelectionMode::None,
                    },
                },

            },
        }
    }

    fn init(
        api_client: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SearchResultsEmpty;

        sender.oneshot_command(async move {
            match api_client.get_search_suggestions(3).await {
                Ok((items, _)) => SearchResultsEmptyCommandOutput::Suggestions(items),
                Err(err) => {
                    println!("Error getting search suggestions: {err}");
                    SearchResultsEmptyCommandOutput::Suggestions(vec![])
                }
            }
        });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            SearchResultsEmptyCommandOutput::Suggestions(items) if !items.is_empty() => {
                let suggestions = &widgets.suggestions;

                for item in items {
                    if let Some(name) = &item.name {
                        let suffix = gtk::Image::from_icon_name("go-next-symbolic");
                        let row = adw::ActionRow::builder()
                            .title(name)
                            .activatable(true)
                            .build();
                        row.add_suffix(&suffix);
                        row.connect_activated(move |_| {
                            APP_BROKER.send(AppInput::ShowDetails(item.clone()));
                        });
                        suggestions.append(&row);
                    }
                }

                suggestions.set_visible(true);
            }
            _ => {}
        }

        self.update_view(widgets, sender);
    }
}
