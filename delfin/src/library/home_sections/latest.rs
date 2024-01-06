use std::{collections::HashMap, convert::identity, sync::Arc};

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use uuid::Uuid;

use crate::{
    jellyfin_api::{
        api_client::ApiClient,
        models::{collection_type::CollectionType, user_view::UserView},
    },
    library::media_list::{
        MediaList, MediaListInit, MediaListOutput, MediaListType, MediaListTypeLatestParams,
    },
    tr,
};

pub struct HomeSectionLatest {
    rows: HashMap<String, Controller<LatestRow>>,
}

#[derive(Debug)]
pub enum HomeSectionLatestInput {
    Empty(Uuid),
    None,
}

#[relm4::component(pub)]
impl Component for HomeSectionLatest {
    type Init = (Arc<ApiClient>, Vec<UserView>);
    type Input = HomeSectionLatestInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 20,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let api_client = init.0;
        let user_views = init.1;

        let mut model = HomeSectionLatest {
            rows: HashMap::new(),
        };

        let widgets = view_output!();

        let user_views: Vec<&UserView> = user_views
            .iter()
            .filter(|view| {
                matches!(
                    view.collection_type(),
                    CollectionType::Movies | CollectionType::TvShows
                )
            })
            .collect();

        for view in user_views {
            let row = LatestRow::builder()
                .launch((api_client.clone(), view.clone()))
                .forward(sender.input_sender(), identity);
            root.append(row.widget());
            model.rows.insert(view.id().to_string(), row);
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            HomeSectionLatestInput::Empty(id) => {
                if let Some(row) = self.rows.get(&id.to_string()) {
                    row.widget().set_visible(false);
                }
            }
            HomeSectionLatestInput::None => {}
        }
    }
}

impl From<MediaListOutput> for HomeSectionLatestInput {
    fn from(value: MediaListOutput) -> Self {
        match value {
            MediaListOutput::Empty(Some(id)) => HomeSectionLatestInput::Empty(id),
            _ => HomeSectionLatestInput::None,
        }
    }
}

pub struct LatestRow {
    _media_list: AsyncController<MediaList>,
}

#[relm4::component(pub)]
impl SimpleComponent for LatestRow {
    type Init = (Arc<ApiClient>, UserView);
    type Input = HomeSectionLatestInput;
    type Output = HomeSectionLatestInput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, view) = init;

        let widgets = view_output!();

        let title_text = match view.collection_type() {
            CollectionType::Movies => tr!("library-section-title.latest-movies").to_string(),
            CollectionType::TvShows => tr!("library-section-title.latest-shows").to_string(),
            CollectionType::Music => tr!("library-section-title.latest-music").to_string(),
            s => {
                println!("Unknown collection type: {s}");
                s.to_string()
            }
        };

        let media_list = MediaList::builder()
            .launch(MediaListInit {
                api_client,
                list_type: MediaListType::Latest(MediaListTypeLatestParams { view_id: view.id() }),
                label: title_text.to_string(),
            })
            .forward(sender.input_sender(), |o| o.into());
        root.append(media_list.widget());

        let model = LatestRow {
            _media_list: media_list,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        sender.output(message).unwrap();
    }
}
