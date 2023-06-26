use std::sync::Arc;

use gtk::prelude::*;
use relm4::{gtk, prelude::*, Component, ComponentParts};

use crate::{
    jellyfin_api::api_client::ApiClient,
    library::media_grid::{MediaGrid, MediaGridInit, MediaGridOutput, MediaGridType},
};

pub struct HomeSectionContinueWatching {
    _media_grid: Controller<MediaGrid>,
}

#[derive(Debug)]
pub enum HomeSectionContinueWatchingInput {
    Empty,
}

#[relm4::component(pub)]
impl Component for HomeSectionContinueWatching {
    type Init = Arc<ApiClient>;
    type Input = HomeSectionContinueWatchingInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            gtk::Label {
                set_label: "Continue Watching",
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
        api_client: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let media_grid = MediaGrid::builder()
            .launch(MediaGridInit {
                api_client,
                grid_type: MediaGridType::ContinueWatching,
            })
            .forward(sender.input_sender(), |o| o.into());

        let model = HomeSectionContinueWatching {
            _media_grid: media_grid,
        };

        let widgets = view_output!();
        let container = &widgets.container;

        container.set_child(Some(model._media_grid.widget()));

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        _widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            HomeSectionContinueWatchingInput::Empty => root.set_visible(false),
        }
    }
}

impl From<MediaGridOutput> for HomeSectionContinueWatchingInput {
    fn from(value: MediaGridOutput) -> Self {
        match value {
            MediaGridOutput::Empty(_) => HomeSectionContinueWatchingInput::Empty,
        }
    }
}
