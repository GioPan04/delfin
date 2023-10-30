use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, Component, ComponentParts,
};

use crate::{
    jellyfin_api::api_client::ApiClient,
    library::media_list::{MediaList, MediaListInit, MediaListOutput, MediaListType},
};

pub struct HomeSectionContinueWatching {
    _media_grid: AsyncController<MediaList>,
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
        }
    }

    fn init(
        api_client: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let media_grid = MediaList::builder()
            .launch(MediaListInit {
                api_client,
                list_type: MediaListType::ContinueWatching,
                label: "Continue Watching".to_string(),
            })
            .forward(sender.input_sender(), |o| o.into());
        root.append(media_grid.widget());

        let model = HomeSectionContinueWatching {
            _media_grid: media_grid,
        };

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

impl From<MediaListOutput> for HomeSectionContinueWatchingInput {
    fn from(value: MediaListOutput) -> Self {
        match value {
            MediaListOutput::Empty(_) => HomeSectionContinueWatchingInput::Empty,
        }
    }
}
