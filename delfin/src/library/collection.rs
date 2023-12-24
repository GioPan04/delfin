use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;
use tokio::sync::mpsc;

use crate::{
    jellyfin_api::{api::views::UserView, api_client::ApiClient},
    library::{media_grid::MediaGridInit, media_tile::MediaTileDisplay},
    utils::constants::PAGE_MARGIN,
};

use super::{
    library_container::LibraryContainer,
    media_fetcher::{Fetcher, FetcherCount, FetcherDisplay, FetcherState, MediaFetcher},
    media_grid::MediaGrid,
};

const ITEMS_PER_PAGE: usize = 24;

pub struct Collection {
    api_client: Arc<ApiClient>,
    view: UserView,
    fetcher: MediaFetcher<ViewItemsFetcher>,
    grid: Option<Controller<MediaGrid>>,
    loading: bool,
    count: FetcherCount,
}

#[derive(Debug)]
pub enum CollectionInput {
    Visible,
    NextPage,
    PrevPage,
    FetcherStateUpdate(FetcherState),
}

#[relm4::component(pub async)]
impl AsyncComponent for Collection {
    type Init = (Arc<ApiClient>, UserView);
    type Input = CollectionInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[template]
            LibraryContainer {
                set_margin_top: 0,
                set_margin_bottom: 0,

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: PAGE_MARGIN,
                    set_margin_bottom: PAGE_MARGIN,

                    gtk::Label {
                        set_label: &model.view.name,
                        add_css_class: "title-1",
                        set_margin_end: 8,
                    },

                    gtk::Label {
                        set_halign: gtk::Align::End,
                        set_hexpand: true,
                        set_margin_end: 8,
                        #[watch]
                        set_label: &model.count.label(),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        add_css_class: "linked",

                        gtk::Button {
                            set_icon_name: "left",
                            add_css_class: "flat",
                            #[watch]
                            set_sensitive: model.fetcher.has_prev(),
                            connect_clicked[sender] => move |_| {
                                sender.input(CollectionInput::PrevPage);
                            },
                        },

                        gtk::Button {
                            set_icon_name: "right",
                            add_css_class: "flat",
                            #[watch]
                            set_sensitive: model.fetcher.has_next(),
                            connect_clicked[sender] => move |_| {
                                sender.input(CollectionInput::NextPage);
                            },
                        },
                    },
                },
            },

            gtk::Spinner {
                #[watch]
                set_visible: model.loading,
                set_spinning: true,
                set_width_request: 32,
                set_height_request: 32,
            },


            #[name = "scroll"]
            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,
                #[watch]
                set_visible: !model.loading,

                #[template]
                LibraryContainer {
                    #[name = "container"]
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 32,
                        #[watch]
                        set_visible: model.count.total > 0,
                    },
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (api_client, view) = init;

        let (tx, mut rx) = mpsc::unbounded_channel();

        let fetcher = MediaFetcher::new(
            Arc::new(ViewItemsFetcher {
                api_client: api_client.clone(),
                view: view.clone(),
            }),
            tx,
            ITEMS_PER_PAGE,
        );

        relm4::spawn({
            let sender = sender.clone();
            async move {
                while let Some(state) = rx.recv().await {
                    sender.input(CollectionInput::FetcherStateUpdate(state));
                }
            }
        });

        let model = Collection {
            api_client,
            view,
            fetcher,
            grid: None,
            loading: true,
            count: FetcherCount::default(),
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let container = &widgets.container;
        let scroll = &widgets.scroll;

        match message {
            CollectionInput::Visible => {
                if self.count.total == 0 {
                    self.fetcher.next_page();
                }
            }
            CollectionInput::NextPage => {
                self.fetcher.next_page();
            }
            CollectionInput::PrevPage => {
                self.fetcher.prev_page();
            }
            CollectionInput::FetcherStateUpdate(state) => match state {
                FetcherState::Empty => {
                    self.loading = true;
                }
                FetcherState::Loading(count) => {
                    self.loading = true;
                    self.count = count;
                }
                FetcherState::Ready(FetcherDisplay { items, count }) => {
                    self.loading = false;
                    self.display_items(container, scroll, items);
                    self.count = count;
                }
                FetcherState::Error(err) => {
                    // TODO
                    println!("Error loading view: {err:#?}");
                }
            },
        }

        self.update_view(widgets, sender);
    }
}

impl Collection {
    fn display_items(
        &mut self,
        container: &gtk::Box,
        scroll: &gtk::ScrolledWindow,
        items: Vec<BaseItemDto>,
    ) {
        if let Some(grid) = self.grid.take() {
            container.remove(grid.widget());
        }

        let grid = MediaGrid::builder()
            .launch(MediaGridInit {
                media: items,
                media_tile_display: MediaTileDisplay::CoverLarge,
                api_client: self.api_client.clone(),
            })
            .detach();
        container.append(grid.widget());

        self.grid = Some(grid);
        self.loading = false;
        scroll.set_vadjustment(gtk::Adjustment::NONE);
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
}
