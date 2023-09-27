use std::sync::Arc;

use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, ComponentParts, ComponentSender, SimpleComponent,
};

use crate::jellyfin_api::api_client::ApiClient;

use super::media_tile::{MediaTile, MediaTileDisplay};

pub(crate) struct MediaGrid {
    media_tiles: Vec<AsyncController<MediaTile>>,
}

pub(crate) struct MediaGridInit {
    pub(crate) media: Vec<BaseItemDto>,
    pub(crate) media_tile_display: MediaTileDisplay,
    pub(crate) api_client: Arc<ApiClient>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for MediaGrid {
    type Init = MediaGridInit;
    type Input = ();
    type Output = ();

    view! {
        gtk::Grid {
            set_column_spacing: 32,
            set_column_homogeneous: true,
            set_halign: gtk::Align::Start,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let MediaGridInit {
            api_client,
            media,
            media_tile_display,
        } = init;

        let mut model = MediaGrid {
            media_tiles: vec![],
        };

        for (column, media) in media.into_iter().enumerate() {
            let media_tile = MediaTile::builder()
                .launch((media, media_tile_display, api_client.clone()))
                .detach();
            root.attach(media_tile.widget(), column as i32, 0, 1, 1);
            model.media_tiles.push(media_tile);
        }

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
