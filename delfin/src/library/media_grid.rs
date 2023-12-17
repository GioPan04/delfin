use std::sync::Arc;

use jellyfin_api::types::BaseItemDto;
use relm4::prelude::*;

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
        gtk::FlowBox {
            set_column_spacing: 16,
            set_row_spacing: 32,
            set_homogeneous: true,
            set_selection_mode: gtk::SelectionMode::None,
            set_max_children_per_line: 6,
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

        for media in media {
            let media_tile = MediaTile::builder()
                .launch((media, media_tile_display, api_client.clone()))
                .detach();
            root.append(media_tile.widget());
            model.media_tiles.push(media_tile);
        }

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
