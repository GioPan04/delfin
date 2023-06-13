use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    factory::{positions::GridPosition, FactoryVecDeque, Position},
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    Component, ComponentController, ComponentParts, Controller,
};
use relm4_components::web_image::WebImage;

use crate::jellyfin_api::{api::latest::LatestMedia, api_client::ApiClient};

pub struct ViewLatest {
    id: String,
    name: String,
    api_client: Arc<ApiClient>,
    media_tiles: FactoryVecDeque<MediaTile>,
}

#[derive(Debug)]
pub enum ViewLatestInput {
    MediaSelected(LatestMedia),
}

#[derive(Debug)]
pub enum ViewLatestOutput {
    MediaSelected(LatestMedia),
}

#[derive(Debug)]
pub enum ViewLatestCommandOutput {
    LatestMediaLoaded(Vec<LatestMedia>),
}

#[relm4::component(pub)]
impl Component for ViewLatest {
    type Init = (String, String, Arc<ApiClient>);
    type Input = ViewLatestInput;
    type Output = ViewLatestOutput;
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
                    set_halign: gtk::Align::Start,
                    set_margin_bottom: 12,
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

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ViewLatestInput::MediaSelected(media) => {
                sender
                    .output(ViewLatestOutput::MediaSelected(media))
                    .unwrap();
            }
        }
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
    media: LatestMedia,
    image: Controller<WebImage>,
}

#[derive(Debug)]
enum MediaTileOutput {
    Selected(LatestMedia),
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
    type Output = MediaTileOutput;
    type CommandOutput = ();
    type Root = gtk::Box;
    type Widgets = ();
    type ParentWidget = gtk::Grid;
    type ParentInput = ViewLatestInput;
    type Index = DynamicIndex;

    fn init_root(&self) -> Self::Root {
        relm4::view! {
            root = gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_width_request: 200,
                set_height_request: 256,
                add_css_class: "media-tile",
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
            .launch(media.image_tags.primary.clone())
            .detach();

        MediaTile { media, image }
    }

    fn init_widgets(
        &mut self,
        _index: &Self::Index,
        root: &Self::Root,
        _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
        sender: relm4::FactorySender<Self>,
    ) -> Self::Widgets {
        let image = self.image.widget();
        let media = &self.media;
        let played_percentage = media.user_data.played_percentage.map(|p| p / 100.0);
        relm4::view! {
            #[local_ref]
            root -> gtk::Box {
                add_controller = gtk::GestureClick {
                    connect_pressed[sender, media] => move |_, _, _, _| {
                        sender.output(MediaTileOutput::Selected(media.clone()));
                    },
                },
                add_controller = gtk::EventControllerMotion {
                    connect_enter[root] => move |_, _, _| {
                        root.add_css_class("hover");
                    },
                    connect_leave[root] => move |_| {
                        root.remove_css_class("hover");
                    },
                },

                gtk::Overlay {
                    #[local_ref]
                    image -> gtk::Box {},

                    add_overlay = &gtk::ProgressBar {
                        set_visible: self.media.user_data.played_percentage.is_some(),
                        set_fraction?: played_percentage,
                    },
                },

                gtk::Label {
                    set_ellipsize: gtk::pango::EllipsizeMode::End,
                    #[watch]
                    set_label: &self.media.name,
                },
            }
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            MediaTileOutput::Selected(media) => ViewLatestInput::MediaSelected(media),
        })
    }
}
