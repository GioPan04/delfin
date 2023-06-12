use relm4::ComponentController;
use std::sync::Arc;

use adw::prelude::*;
use relm4::{adw, gtk, prelude::*, Component, Controller};

use crate::api::{api_client::ApiClient, latest::LatestMedia, views::UserViews};

use super::view_latest::{ViewLatest, ViewLatestOutput};

enum LibraryState {
    Loading,
    Ready,
}

// TODO: remove this
#[allow(dead_code)]
pub struct Library {
    api_client: Arc<ApiClient>,
    state: LibraryState,
    views: Option<Vec<Controller<ViewLatest>>>,
}

#[derive(Debug)]
pub enum LibraryInput {
    MediaSelected(LatestMedia),
}

#[derive(Debug)]
pub enum LibraryOutput {
    NavigateBack,
    PlayVideo(LatestMedia),
}

#[derive(Debug)]
pub enum LibraryCommandOutput {
    LibraryLoaded(UserViews),
}

#[relm4::component(pub)]
impl Component for Library {
    type Init = Arc<ApiClient>;
    type Input = LibraryInput;
    type Output = LibraryOutput;
    type CommandOutput = LibraryCommandOutput;

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,

            #[transition = "Crossfade"]
            append = if matches!(model.state, LibraryState::Loading) {
                adw::Clamp {
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_valign: gtk::Align::Center,
                        set_spacing: 20,

                        gtk::Spinner {
                            set_spinning: true,
                            set_size_request: (64, 64),
                        },

                        gtk::Label {
                            set_label: "Loading your library...",
                            add_css_class: "title-2",
                        },
                    }
                }
            } else {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                            set_centering_policy: adw::CenteringPolicy::Strict,

                            #[wrap(Some)]
                            #[name = "view_switcher_title"]
                            set_title_widget = &adw::ViewSwitcherTitle {
                                set_title: "Jellything",
                                set_stack: Some(&view_stack),
                            },

                            pack_start = &gtk::Button {
                                set_icon_name: "go-previous",
                                #[watch]
                                set_visible: true,
                                connect_clicked[sender] => move |_| {
                                    sender.output(LibraryOutput::NavigateBack).unwrap();
                                },
                            },
                    },

                    gtk::ScrolledWindow {
                        #[local_ref]
                        view_stack -> adw::ViewStack {
                            set_margin_all: 20,
                            set_valign: gtk::Align::Fill,

                            #[name = "home"]
                            add_titled_with_icon[Some("home"), "Home", "home-filled"] = &gtk::Box {
                                set_valign: gtk::Align::Start,
                                set_vexpand: true,
                                set_orientation: gtk::Orientation::Vertical,
                                set_spacing: 20,
                            },
                        },
                    },

                    #[name = "view_switcher_bar"]
                    adw::ViewSwitcherBar {
                        set_stack: Some(&view_stack),
                    },
                }
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let api_client = init;

        let model = Library {
            api_client: Arc::clone(&api_client),
            state: LibraryState::Loading,
            views: None,
        };

        let view_stack = adw::ViewStack::new();

        let widgets = view_output!();

        // Always show view switcher at either top or bottom of screen
        let view_switcher_title = &widgets.view_switcher_title;
        let view_switcher_bar = &widgets.view_switcher_bar;
        view_switcher_title
            .bind_property("title-visible", view_switcher_bar, "reveal")
            .build();

        // Initial fetch
        model.fetch_user_views(&sender);

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            LibraryInput::MediaSelected(media) => {
                sender.output(LibraryOutput::PlayVideo(media)).unwrap();
            }
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            LibraryCommandOutput::LibraryLoaded(user_views) => {
                self.display_user_views(widgets, &sender, user_views)
            }
        }

        self.update_view(widgets, sender);
    }
}

impl Library {
    fn fetch_user_views(&self, sender: &relm4::ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        sender.oneshot_command(async move {
            let user_views = api_client
                .get_user_views()
                .await
                .unwrap_or_else(|err| panic!("Error getting library: {}", err));
            LibraryCommandOutput::LibraryLoaded(user_views)
        });
    }

    fn display_user_views(
        &mut self,
        widgets: &mut LibraryWidgets,
        sender: &relm4::ComponentSender<Self>,
        user_views: UserViews,
    ) {
        self.state = LibraryState::Ready;

        let home = &widgets.home;
        let view_stack = &widgets.view_stack;

        let mut views = Vec::new();
        for view in user_views {
            let new_view = ViewLatest::builder()
                .launch((
                    view.id.clone(),
                    view.name.clone(),
                    Arc::clone(&self.api_client),
                ))
                .forward(sender.input_sender(), convert_view_latest_output);
            let widget = new_view.widget();
            home.append(widget);
            views.push(new_view);

            let icon = match view.collection_type.as_str() {
                "movies" => "video-clip-multiple-filled",
                "shows" => "video-clip-multiple-filled",
                "music" => "play-multiple-filled",
                "playlists" => "tag-multiple-filled",
                _ => "video-clip-multiple-filled",
            };

            view_stack.add_titled_with_icon(
                &gtk::Box::default(),
                Some(&view.id.clone()),
                &view.name.clone(),
                icon,
            );
        }

        self.views = Some(views);
    }
}

fn convert_view_latest_output(output: ViewLatestOutput) -> LibraryInput {
    match output {
        ViewLatestOutput::MediaSelected(media) => LibraryInput::MediaSelected(media),
    }
}
