use std::{collections::HashMap, convert::identity, sync::Arc};

use gtk::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};
use uuid::Uuid;

use crate::{
    jellyfin_api::{api::views::UserView, api_client::ApiClient},
    library::media_grid::{
        MediaGrid, MediaGridInit, MediaGridOutput, MediaGridType, MediaGridTypeLatestParams,
    },
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
            .filter(|view| matches!(view.collection_type.as_ref(), "movies" | "tvshows"))
            .collect();

        for view in user_views {
            let row = LatestRow::builder()
                .launch((api_client.clone(), view.clone()))
                .forward(sender.input_sender(), identity);
            root.append(row.widget());
            model.rows.insert(view.id.to_string(), row);
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

impl From<MediaGridOutput> for HomeSectionLatestInput {
    fn from(value: MediaGridOutput) -> Self {
        match value {
            MediaGridOutput::Empty(Some(id)) => HomeSectionLatestInput::Empty(id),
            _ => HomeSectionLatestInput::None,
        }
    }
}

pub struct LatestRow {
    _media_grid: Controller<MediaGrid>,
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

            #[name = "title"]
            gtk::Label {
                set_label: "Latest ...",
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
        _root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, view) = init;

        let widgets = view_output!();
        let title = &widgets.title;
        let container = &widgets.container;

        let title_text = match view.collection_type.as_str() {
            "movies" => "Latest Movies",
            "tvshows" => "Latest Shows",
            "music" => "Latest Music",
            s => {
                println!("Unknown collection type: {s}");
                s
            }
        };
        title.set_label(title_text);

        let media_grid = MediaGrid::builder()
            .launch(MediaGridInit {
                api_client,
                grid_type: MediaGridType::Latest(MediaGridTypeLatestParams { view_id: view.id }),
            })
            .forward(sender.input_sender(), |o| o.into());
        container.set_child(Some(media_grid.widget()));

        let model = LatestRow {
            _media_grid: media_grid,
        };

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        sender.output(message).unwrap();
    }
}
