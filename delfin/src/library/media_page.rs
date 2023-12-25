use std::sync::Arc;

use gtk::{pango, prelude::*};
use relm4::{gtk, prelude::*};
use tokio::sync::mpsc;

use crate::{
    jellyfin_api::api_client::ApiClient, library::library_container::LibraryContainer,
    utils::constants::PAGE_MARGIN,
};

use super::{
    media_fetcher::{Fetcher, FetcherCount, FetcherDisplay, FetcherState, MediaFetcher},
    media_grid::{MediaGrid, MediaGridInit},
    media_tile::MediaTileDisplay,
};

const ITEMS_PER_PAGE: usize = 24;

pub struct MediaPage<F: Fetcher + Send + Sync + 'static, EmptyComponent: Component>
where
    <EmptyComponent as Component>::Root: IsA<gtk::Widget>,
{
    api_client: Arc<ApiClient>,
    fetcher: MediaFetcher<F>,
    empty_component: Option<Controller<EmptyComponent>>,
    media_grid: Option<Controller<MediaGrid>>,
    state: FetcherState,
    count: Option<FetcherCount>,
}

pub struct MediaPageInit<F, C: Component> {
    pub api_client: Arc<ApiClient>,
    pub fetcher: F,
    pub empty_component: Option<Controller<C>>,
}

#[derive(Debug)]
pub enum MediaPageInput {
    FetcherState(FetcherState),
    NextPage,
    PrevPage,
}

#[relm4::component(pub)]
impl<F: Fetcher + Send + Sync + 'static, EmptyComponent: Component> Component
    for MediaPage<F, EmptyComponent>
where
    <EmptyComponent as Component>::Root: IsA<gtk::Widget>,
{
    type Init = MediaPageInit<F, EmptyComponent>;
    type Input = MediaPageInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_hexpand: true,
            set_vexpand: true,

            #[template]
            LibraryContainer {
                set_margin_top: 0,
                set_margin_bottom: 0,
                #[watch]
                set_visible: matches!(model.state, FetcherState::Loading(_) | FetcherState::Ready(_)),

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_margin_top: PAGE_MARGIN,
                    set_margin_bottom: PAGE_MARGIN,

                    gtk::Label {
                        #[watch]
                        set_label: &model.fetcher.title(),
                        add_css_class: "title-1",
                        set_margin_end: 8,
                        set_ellipsize: pango::EllipsizeMode::End,
                    },

                    gtk::Label {
                        set_halign: gtk::Align::End,
                        set_hexpand: true,
                        set_margin_end: 8,
                        #[watch]
                        set_label?: &model.count.as_ref().map(|count| count.label()),
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
                                sender.input(MediaPageInput::PrevPage);
                            },
                        },

                        gtk::Button {
                            set_icon_name: "right",
                            add_css_class: "flat",
                            #[watch]
                            set_sensitive: model.fetcher.has_next(),
                            connect_clicked[sender] => move |_| {
                                sender.input(MediaPageInput::NextPage);
                            },
                        },
                    },
                },
            },

            gtk::Spinner {
                #[watch]
                set_visible: matches!(model.state, FetcherState::Loading(_)),
                set_spinning: true,
                set_width_request: 32,
                set_height_request: 32,
            },

            #[template]
            LibraryContainer {
                #[watch]
                set_visible: matches!(model.state, FetcherState::Empty),

                #[name = "empty_container"]
                gtk::Box {
                    set_widget_name: "empty_container",
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 32,


                    append?: model.empty_component.as_ref().map(|c| c.widget()),
                },
            },

            #[name = "scroll"]
            gtk::ScrolledWindow {
                set_hexpand: true,
                set_vexpand: true,
                #[watch]
                set_visible: matches!(model.state, FetcherState::Ready(_)),

                #[template]
                LibraryContainer {
                    #[name = "media_grid_container"]
                    gtk::Box {
                        set_widget_name: "media_grid_container",
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 32,
                    },
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let MediaPageInit {
            api_client,
            fetcher,
            empty_component,
        } = init;

        let (tx, mut rx) = mpsc::unbounded_channel();

        let fetcher = MediaFetcher::new(Arc::new(fetcher), tx, ITEMS_PER_PAGE);

        relm4::spawn({
            let sender = sender.clone();
            async move {
                while let Some(state) = rx.recv().await {
                    sender.input(MediaPageInput::FetcherState(state));
                }
            }
        });

        let model = MediaPage {
            api_client,
            fetcher,
            empty_component,
            media_grid: None,
            state: FetcherState::Empty,
            count: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            MediaPageInput::FetcherState(state) => {
                self.count = match state {
                    FetcherState::Loading(count) => Some(count),
                    FetcherState::Ready(FetcherDisplay { items: _, count }) => Some(count),
                    _ => None,
                };

                if let FetcherState::Ready(display) = &state {
                    let media_grid_container = &widgets.media_grid_container;

                    if let Some(media_grid) = self.media_grid.take() {
                        media_grid_container.remove(media_grid.widget());
                    }

                    let media_grid = MediaGrid::builder()
                        .launch(MediaGridInit {
                            media: display.items.clone(),
                            media_tile_display: MediaTileDisplay::CoverLarge,
                            api_client: self.api_client.clone(),
                        })
                        .detach();
                    media_grid_container.append(media_grid.widget());
                    self.media_grid = Some(media_grid);

                    widgets.scroll.set_vadjustment(gtk::Adjustment::NONE);
                };

                self.state = state;
            }
            MediaPageInput::NextPage => {
                self.fetcher.next_page();
            }
            MediaPageInput::PrevPage => {
                self.fetcher.prev_page();
            }
        }

        self.update_view(widgets, sender);
    }
}
