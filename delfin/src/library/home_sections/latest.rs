use std::{collections::HashMap, convert::identity, sync::Arc, unreachable};

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use uuid::Uuid;

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::{
        api_client::ApiClient,
        models::user_view::{FilterSupported, UserView},
    },
    library::media_list::{
        MediaList, MediaListInit, MediaListOutput, MediaListType, MediaListTypeLatestParams,
    },
    tr,
};

pub struct HomeSectionLatest {
    rows: HashMap<Uuid, (UserView, Controller<LatestRow>)>,
}

#[derive(Debug)]
pub enum HomeSectionLatestInput {
    Empty(Uuid),
    ShowCollection(Uuid),
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
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let api_client = init.0;
        let user_views = init.1;

        let mut model = HomeSectionLatest {
            rows: HashMap::new(),
        };

        let widgets = view_output!();

        let user_views = user_views.filter_supported();

        for view in user_views {
            let row = LatestRow::builder()
                .launch((api_client.clone(), view.clone()))
                .forward(sender.input_sender(), identity);
            root.append(row.widget());
            model.rows.insert(view.id(), (view, row));
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            HomeSectionLatestInput::Empty(id) => {
                if let Some(row) = self.rows.get(&id) {
                    row.1.widget().set_visible(false);
                }
            }
            HomeSectionLatestInput::ShowCollection(id) => {
                if let Some(collection) = self.rows.get(&id) {
                    APP_BROKER.send(AppInput::ShowCollection(collection.0.clone().into()));
                }
            }
        }
    }
}

impl From<MediaListOutput> for HomeSectionLatestInput {
    fn from(value: MediaListOutput) -> Self {
        match value {
            MediaListOutput::Empty(Some(id)) => HomeSectionLatestInput::Empty(id),
            MediaListOutput::LabelClicked(Some(id)) => HomeSectionLatestInput::ShowCollection(id),
            _ => unreachable!(),
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
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, view) = init;

        let widgets = view_output!();

        let media_list = MediaList::builder()
            .launch(MediaListInit {
                api_client,
                list_type: MediaListType::Latest(MediaListTypeLatestParams { view_id: view.id() }),
                label: tr!("library-section-title.latest-in", { "name" => view.name() })
                    .to_string(),
                label_clickable: true,
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
