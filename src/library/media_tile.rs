use gtk::prelude::*;
use relm4::{
    factory::{positions::GridPosition, Position},
    prelude::*,
};
use relm4_components::web_image::WebImage;

use crate::jellyfin_api::models::media::Media;

use super::media_grid::MediaGridInput;

pub struct MediaTile {
    media: Media,
    image: Controller<WebImage>,
}

#[derive(Debug)]
pub enum MediaTileOutput {
    Selected(Media),
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
    type Init = Media;
    type Input = ();
    type Output = MediaTileOutput;
    type CommandOutput = ();
    type Root = gtk::Box;
    type Widgets = ();
    type ParentWidget = gtk::Grid;
    type ParentInput = MediaGridInput;
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
            MediaTileOutput::Selected(media) => MediaGridInput::MediaSelected(media),
        })
    }
}
