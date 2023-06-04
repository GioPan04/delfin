use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    factory::{positions::GridPosition, FactoryVecDeque, Position},
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    Component, ComponentController, ComponentParts, Controller,
};
use relm4_components::web_image::WebImage;

use crate::api::{api_client::ApiClient, latest::LatestMedia};

pub struct ViewLatest {
    id: String,
    name: String,
    api_client: Arc<ApiClient>,
    media_tiles: FactoryVecDeque<MediaTile>,
}

#[derive(Debug)]
pub enum ViewLatestCommandOutput {
    LatestMediaLoaded(Vec<LatestMedia>),
}

#[relm4::component(pub)]
impl Component for ViewLatest {
    type Init = (String, String, Arc<ApiClient>);
    type Input = ();
    type Output = ();
    type CommandOutput = ViewLatestCommandOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            gtk::Label {
                #[watch]
                set_label: &format!("Latest {}", model.name),
                add_css_class: "title-2",
                set_halign: gtk::Align::Start,
            },

            gtk::ScrolledWindow {
                set_vscrollbar_policy: gtk::PolicyType::Never,
                #[local_ref]
                media_box -> gtk::Grid {
                    set_column_spacing: 16,
                    set_column_homogeneous: true,
                },
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let media_tiles = FactoryVecDeque::new(gtk::Grid::default(), sender.input_sender());

        let model = ViewLatest {
            id: init.0,
            name: init.1,
            api_client: init.2,
            media_tiles,
        };

        let media_box = model.media_tiles.widget();
        let widgets = view_output!();

        // Initial fetch
        model.fetch_latest_media(&sender);

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            ViewLatestCommandOutput::LatestMediaLoaded(latest_media) => {
                if latest_media.is_empty() {
                    root.set_visible(false);
                    return;
                }
                for media in latest_media {
                    self.media_tiles.guard().push_back(media);
                }
            }
        }
    }
}

impl ViewLatest {
    fn fetch_latest_media(&self, sender: &relm4::ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        let id = self.id.clone();
        sender.oneshot_command(async move {
            let latest_media = api_client
                .get_latest_media(&id, None)
                .await
                .expect("Error getting latest media.");
            ViewLatestCommandOutput::LatestMediaLoaded(latest_media)
        });
    }
}

struct MediaTile {
    name: String,
    image: Controller<WebImage>,
}

impl Position<GridPosition, DynamicIndex> for MediaTile {
    fn position(&self, index: &DynamicIndex) -> GridPosition {
        let index = index.current_index();
        GridPosition {
            column: index as i32,
            row: 0,
            width: 1,
            height: 1,
        }
    }
}

impl FactoryComponent for MediaTile {
    type Init = LatestMedia;
    type Input = ();
    type Output = ();
    type CommandOutput = ();
    type Root = gtk::Box;
    type Widgets = ();
    type ParentWidget = gtk::Grid;
    type ParentInput = ();
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        relm4::view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 8,
                set_width_request: 200,
                set_height_request: 256,
            }
        }
        root
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        let media = init;

        let image = WebImage::builder()
            .launch(media.image_tags.primary)
            .detach();

        MediaTile {
            name: media.name,
            image,
        }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        _sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        let image = self.image.widget();
        relm4::view! {
            #[local_ref]
            root -> gtk::Box {
                #[local_ref]
                image -> gtk::Box {},
                gtk::Label {
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.name,
                },
            }
        }
    }
}
