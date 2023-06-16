use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, Component, ComponentController, ComponentParts, Controller};

use crate::{
    jellyfin_api::{api_client::ApiClient, models::media::Media},
    library::media_grid::{MediaGrid, MediaGridType},
};

use super::media_grid::MediaGridOutput;

pub struct ViewLatest {
    _id: String,
    name: String,
    media_grid: Controller<MediaGrid>,
}

#[derive(Debug)]
pub enum ViewLatestInput {
    MediaSelected(Media),
    Empty,
}

#[derive(Debug)]
pub enum ViewLatestOutput {
    MediaSelected(Media),
}

#[relm4::component(pub)]
impl Component for ViewLatest {
    type Init = (String, String, Arc<ApiClient>);
    type Input = ViewLatestInput;
    type Output = ViewLatestOutput;
    type CommandOutput = ();

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

            #[name = "container"]
            gtk::ScrolledWindow {
                set_vscrollbar_policy: gtk::PolicyType::Never,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let media_grid = MediaGrid::builder()
            .launch((init.2, init.0.clone(), MediaGridType::Latest))
            .forward(sender.input_sender(), convert_media_grid_output);

        let model = ViewLatest {
            _id: init.0,
            name: init.1,
            media_grid,
        };

        let widgets = view_output!();
        let container = &widgets.container;

        container.set_child(Some(model.media_grid.widget()));

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            ViewLatestInput::MediaSelected(media) => {
                sender
                    .output(ViewLatestOutput::MediaSelected(media))
                    .unwrap();
            }
            ViewLatestInput::Empty => {
                root.set_visible(false);
            }
        }
    }
}

fn convert_media_grid_output(output: MediaGridOutput) -> ViewLatestInput {
    match output {
        MediaGridOutput::MediaSelected(media) => ViewLatestInput::MediaSelected(media),
        MediaGridOutput::Empty => ViewLatestInput::Empty,
    }
}
