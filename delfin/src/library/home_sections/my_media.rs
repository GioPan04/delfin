use std::sync::Arc;

use gtk::prelude::*;
use relm4::prelude::*;

use crate::{
    jellyfin_api::{
        api_client::ApiClient,
        models::user_view::{FilterSupported, UserView},
    },
    library::{
        media_list::{MediaList, MediaListInit, MediaListOutput, MediaListType},
        LibraryInput, LIBRARY_BROKER,
    },
    tr,
};

pub struct HomeSectionMyMedia {
    media_list: AsyncController<MediaList>,
}

pub struct HomeSectionMyMediaInit {
    pub api_client: Arc<ApiClient>,
    pub user_views: Vec<UserView>,
    pub small: bool,
}

#[relm4::component(pub)]
impl Component for HomeSectionMyMedia {
    type Init = HomeSectionMyMediaInit;
    type Input = MediaListOutput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            model.media_list.widget(),
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let HomeSectionMyMediaInit {
            api_client,
            user_views,
            small,
        } = init;

        let user_views = user_views.filter_supported();

        let media_list = MediaList::builder()
            .launch(MediaListInit {
                api_client,
                list_type: MediaListType::MyMedia { user_views, small },
                label: tr!("library-section-title.my-media").to_string(),
                label_clickable: true,
            })
            .forward(sender.input_sender(), |m| m);

        let model = HomeSectionMyMedia { media_list };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            MediaListOutput::Empty(_) => {
                root.set_visible(false);
            }
            MediaListOutput::LabelClicked(_) => {
                LIBRARY_BROKER.send(LibraryInput::ShowCollections);
            }
        }
    }
}
