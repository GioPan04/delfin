use std::sync::Arc;

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, Component, ComponentParts,
};

use crate::{
    jellyfin_api::api_client::ApiClient,
    library::media_list::{MediaList, MediaListInit, MediaListOutput, MediaListType},
    tr,
};

pub struct HomeSectionContinueWatching {
    _media_grid: AsyncController<MediaList>,
}

#[relm4::component(pub)]
impl Component for HomeSectionContinueWatching {
    type Init = Arc<ApiClient>;
    type Input = MediaListOutput;
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
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let widgets = view_output!();

        let media_grid = MediaList::builder()
            .launch(MediaListInit {
                api_client,
                list_type: MediaListType::ContinueWatching,
                label: tr!("library-section-title.continue-watching").to_string(),
                label_clickable: false,
            })
            .forward(sender.input_sender(), |o| o);
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
            MediaListOutput::Empty(_) => root.set_visible(false),
            MediaListOutput::LabelClicked(_) => {}
        }
    }
}
