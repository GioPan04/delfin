use std::{cmp, sync::Arc};

use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    jellyfin_api::{api::views::UserView, api_client::ApiClient},
    library::{media_grid::MediaGridInit, media_tile::MediaTileDisplay},
    tr,
    utils::constants::PAGE_MARGIN,
};

use super::{library_container::LibraryContainer, media_grid::MediaGrid};

const ITEMS_PER_PAGE: usize = 24;

pub struct Collection {
    api_client: Arc<ApiClient>,
    view: UserView,
    grid: Option<Controller<MediaGrid>>,
    cur_page: usize,
    total_item_count: usize,
    loading: bool,
}

#[derive(Debug)]
pub enum CollectionInput {
    Visible,
    NextPage,
    PrevPage,
}

#[derive(Debug)]
pub enum CollectionCommandOutput {
    PageLoaded(usize, Vec<BaseItemDto>),
}

#[relm4::component(pub async)]
impl AsyncComponent for Collection {
    type Init = (Arc<ApiClient>, UserView);
    type Input = CollectionInput;
    type Output = ();
    type CommandOutput = CollectionCommandOutput;

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
                        set_label: tr!("library-item-count", {
                            "start" => model.cur_page * ITEMS_PER_PAGE + 1,
                            "end" => cmp::min(model.total_item_count, (model.cur_page + 1) * ITEMS_PER_PAGE),
                            "total" => model.total_item_count,
                        }),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        add_css_class: "linked",

                        gtk::Button {
                            set_icon_name: "left",
                            add_css_class: "flat",
                            #[watch]
                            set_sensitive: model.cur_page > 0,
                            connect_clicked[sender] => move |_| {
                                sender.input(CollectionInput::PrevPage);
                            },
                        },

                        gtk::Button {
                            set_icon_name: "right",
                            add_css_class: "flat",
                            #[watch]
                            set_sensitive: (model.cur_page as f32) < (model.total_item_count as f32 / ITEMS_PER_PAGE as f32) - 1.0,
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
                        set_visible: model.total_item_count > 0,

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

        let model = Collection {
            api_client,
            view,
            grid: None,
            cur_page: 0,
            total_item_count: 0,
            loading: true,
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
                if self.total_item_count == 0 {
                    let (items, total_item_count) = self
                        .api_client
                        .get_view_items(&self.view, 0, ITEMS_PER_PAGE)
                        .await
                        .expect("Error getting view items");
                    self.total_item_count = total_item_count;
                    self.display_items(container, scroll, items);
                }
            }
            CollectionInput::NextPage => {
                self.cur_page += 1;
                self.load_cur_page(&sender);
            }
            CollectionInput::PrevPage => {
                self.cur_page -= 1;
                self.load_cur_page(&sender);
            }
        }

        self.update_view(widgets, sender);
    }

    async fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let container = &widgets.container;
        let scroll = &widgets.scroll;

        match message {
            CollectionCommandOutput::PageLoaded(page, items) => 'msg_block: {
                if self.cur_page != page {
                    break 'msg_block;
                }

                self.display_items(container, scroll, items);
            }
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
                media_tile_display: MediaTileDisplay::Cover,
                api_client: self.api_client.clone(),
            })
            .detach();
        container.append(grid.widget());

        self.grid = Some(grid);
        self.loading = false;
        scroll.set_vadjustment(gtk::Adjustment::NONE);
    }

    fn load_cur_page(&mut self, sender: &AsyncComponentSender<Self>) {
        self.loading = true;
        sender.oneshot_command({
            let api_client = self.api_client.clone();
            let view = self.view.clone();
            let page = self.cur_page;
            async move {
                let (items, _) = api_client
                    .get_view_items(&view, page * ITEMS_PER_PAGE, ITEMS_PER_PAGE)
                    .await
                    .expect("Error getting view items");
                CollectionCommandOutput::PageLoaded(page, items)
            }
        });
    }
}
