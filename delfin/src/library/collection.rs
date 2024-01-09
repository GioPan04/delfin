use std::sync::Arc;

use adw::prelude::*;
use anyhow::Result;
use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

use crate::{
    borgar::borgar_menu::{BorgarMenu, BorgarMenuAuth},
    config::{Account, Server},
    jellyfin_api::api_client::ApiClient,
    library::{
        media_page::{MediaPage, MediaPageInput},
        media_tile::MediaTileDisplay,
    },
    tr,
    utils::empty_component::EmptyComponent,
};

use super::{media_fetcher::Fetcher, media_page::MediaPageInit};

pub struct Collection {
    api_client: Arc<ApiClient>,
    collection: BaseItemDto,
    borgar_menu: Controller<BorgarMenu>,
    media_page: Controller<MediaPage<CollectionItemsFetcher, EmptyComponent>>,
}

#[derive(Debug)]
pub enum CollectionInput {
    Refresh,
}

#[relm4::component(pub)]
impl Component for Collection {
    type Init = (Arc<ApiClient>, BaseItemDto, Server, Account);
    type Input = CollectionInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            #[name = "toolbar_view"]
            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = model.borgar_menu.widget(),
                    pack_end = &gtk::Button::from_icon_name("refresh") {
                        set_tooltip: tr!("library-collection-refresh-button"),
                        connect_clicked[sender] => move |_| {
                            sender.input(CollectionInput::Refresh);
                        },
                    },
                },

                #[wrap(Some)]
                set_content = model.media_page.widget(),
            },

            set_title: &model.collection
                .name
                .as_ref()
                .unwrap_or(tr!("library-unnamed-collection"))
                .clone(),
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, collection, server, account) = init;

        let model = Collection {
            api_client: api_client.clone(),
            collection: collection.clone(),
            borgar_menu: BorgarMenu::builder()
                .launch(Some(BorgarMenuAuth {
                    api_client: api_client.clone(),
                    server,
                    account,
                }))
                .detach(),
            media_page: new_media_page(&api_client, collection),
        };

        model.media_page.emit(MediaPageInput::NextPage);

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
            CollectionInput::Refresh => {
                let toolbar_view = &widgets.toolbar_view;
                let media_page = new_media_page(&self.api_client, self.collection.clone());
                toolbar_view.set_content(Some(media_page.widget()));
                media_page.emit(MediaPageInput::NextPage);
                self.media_page = media_page;
            }
        }
        self.update_view(widgets, sender);
    }
}

fn new_media_page(
    api_client: &Arc<ApiClient>,
    collection: BaseItemDto,
) -> Controller<MediaPage<CollectionItemsFetcher, EmptyComponent>> {
    let fetcher = CollectionItemsFetcher {
        api_client: api_client.clone(),
        collection,
    };
    MediaPage::builder()
        .launch(MediaPageInit {
            api_client: api_client.clone(),
            fetcher,
            empty_component: None,
            media_tile_display: Some(MediaTileDisplay::CoverLarge),
        })
        .detach()
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
            .unwrap_or(tr!("library-unnamed-collection"))
            .clone()
    }
}
