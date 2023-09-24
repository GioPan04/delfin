use jellyfin_api::types::BaseItemDto;
use relm4::ComponentController;
use std::sync::Arc;

use adw::prelude::*;
use relm4::{adw, gtk, prelude::*, Component, Controller};

use crate::jellyfin_api::{
    api::views::UserView, api_client::ApiClient, models::display_preferences::DisplayPreferences,
};

use super::home::{Home, HomeInit};

enum LibraryState {
    Loading,
    Ready,
}

pub struct Library {
    api_client: Arc<ApiClient>,
    state: LibraryState,
    home: Option<Controller<Home>>,
}

#[derive(Debug)]
pub enum LibraryInput {
    MediaSelected(BaseItemDto),
}

#[derive(Debug)]
pub enum LibraryOutput {
    NavigateBack,
    PlayVideo(Box<BaseItemDto>),
}

#[derive(Debug)]
pub enum LibraryCommandOutput {
    LibraryLoaded(Vec<UserView>, DisplayPreferences),
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

            adw::HeaderBar {
                    set_centering_policy: adw::CenteringPolicy::Strict,

                    #[wrap(Some)]
                    #[transition = "Crossfade"]
                    set_title_widget = if matches!(model.state, LibraryState::Loading) {
                        &adw::WindowTitle {}
                    } else {
                        #[name = "view_switcher_title"]
                        &adw::ViewSwitcherTitle {
                            set_title: "Jellything",
                            set_stack: Some(&view_stack),
                        }
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

            #[transition = "Crossfade"]
            append = if matches!(model.state, LibraryState::Loading) {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::Clamp {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_hexpand: true,
                            set_vexpand: true,
                            set_halign: gtk::Align::Center,
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
                }
            } else {
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::ScrolledWindow {
                        #[local_ref]
                        view_stack -> adw::ViewStack {
                            set_margin_all: 20,
                            set_valign: gtk::Align::Fill,
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
            home: None,
        };

        let view_stack = adw::ViewStack::new();

        let widgets = view_output!();

        // Always show view switcher at either top or bottom of screen
        let view_switcher_title = &widgets.view_switcher_title;
        let view_switcher_bar = &widgets.view_switcher_bar;
        view_switcher_title
            .bind_property("title-visible", view_switcher_bar, "reveal")
            .build();

        model.initial_fetch(&sender);

        relm4::ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            LibraryInput::MediaSelected(media) => {
                sender
                    .output(LibraryOutput::PlayVideo(Box::new(media)))
                    .unwrap();
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
            LibraryCommandOutput::LibraryLoaded(user_views, display_preferences) => {
                self.display_user_views(widgets, &sender, user_views, display_preferences)
            }
        }

        self.update_view(widgets, sender);
    }
}

impl Library {
    fn initial_fetch(&self, sender: &relm4::ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        sender.oneshot_command(async move {
            let (user_views, display_preferences) = tokio::join!(
                async {
                    api_client
                        .get_user_views()
                        .await
                        .unwrap_or_else(|err| panic!("Error getting library: {}", err))
                },
                async {
                    api_client
                        // We might eventually want client-specific settings, but for
                        // now use the Jellyfin ("emby") client settings
                        .get_user_display_preferences("emby")
                        .await
                        .unwrap()
                }
            );

            LibraryCommandOutput::LibraryLoaded(user_views, display_preferences)
        });
    }

    fn display_user_views(
        &mut self,
        widgets: &mut LibraryWidgets,
        _sender: &relm4::ComponentSender<Self>,
        user_views: Vec<UserView>,
        display_preferences: DisplayPreferences,
    ) {
        let view_stack = &widgets.view_stack;

        self.state = LibraryState::Ready;

        let home = Home::builder()
            .launch(HomeInit {
                api_client: self.api_client.clone(),
                display_preferences,
                user_views: user_views.clone(),
            })
            .detach();
        view_stack.add_titled_with_icon(home.widget(), Some("home"), "Home", "home-filled");
        self.home = Some(home);

        for view in user_views {
            let icon = match view.collection_type.as_str() {
                "movies" => "video-clip-multiple-filled",
                "tvshows" => "video-clip-multiple-filled",
                "music" => "play-multiple-filled",
                "playlists" => "tag-multiple-filled",
                _ => "video-clip-multiple-filled",
            };

            view_stack.add_titled_with_icon(
                &gtk::Box::default(),
                Some(&view.id.to_string()),
                &view.name.clone(),
                icon,
            );
        }
    }
}
