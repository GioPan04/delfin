mod collection;
mod home;
mod home_sections;
mod library_container;
mod media_carousel;
mod media_fetcher;
mod media_grid;
mod media_list;
pub mod media_page;
mod media_tile;
mod search;

use jellyfin_api::types::BaseItemDto;
use relm4::{binding::BoolBinding, ComponentController, RelmObjectExt, SharedState};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};
use uuid::Uuid;

use adw::prelude::*;
use gtk::glib;
use relm4::{adw, gtk, prelude::*, Component, Controller};

use crate::{
    app::AppPage,
    borgar::borgar_menu::{BorgarMenu, BorgarMenuAuth},
    config::{Account, Server},
    jellyfin_api::{
        api::views::UserView,
        api_client::ApiClient,
        models::{collection_type::CollectionType, display_preferences::DisplayPreferences},
    },
    media_details::MEDIA_DETAILS_REFRESH_QUEUED,
    tr,
    utils::{constants::WIDGET_NONE, message_broker::ResettableMessageBroker},
};

use self::{
    collection::{Collection, CollectionInput},
    home::{Home, HomeInit},
    search::{
        search_bar::SearchBar,
        search_results::{SearchResults, SearchResultsInput},
    },
};

pub static LIBRARY_BROKER: ResettableMessageBroker<LibraryInput> = ResettableMessageBroker::new();
pub static LIBRARY_REFRESH_QUEUED: SharedState<bool> = SharedState::new();

#[derive(Debug)]
pub enum LibraryState {
    Loading,
    Offline,
    Error,
    Ready,
}

pub struct Library {
    borgar_menu: Controller<BorgarMenu>,
    api_client: Arc<ApiClient>,
    state: LibraryState,
    search_results: Controller<SearchResults>,
    home: Option<Controller<Home>>,
    collections: HashMap<Uuid, Controller<Collection>>,
    searching: BoolBinding,
    // Store previous view stack child so we can go back from search
    previous_stack_child: Arc<RwLock<String>>,
}

#[derive(Debug)]
pub enum LibraryInput {
    MediaSelected(BaseItemDto),
    Refresh,
    Shown,
    ViewStackChildVisible(String),
    Toast(String),
    SearchChanged(String),
    SearchingChanged(bool),
}

#[derive(Debug)]
pub enum LibraryOutput {
    PlayVideo(Box<BaseItemDto>),
}

#[derive(Debug)]
pub enum LibraryCommandOutput {
    LibraryLoaded(Vec<UserView>, DisplayPreferences),
    SetLibraryState(LibraryState),
}

#[relm4::component(pub)]
impl Component for Library {
    type Init = (Server, Account, Arc<ApiClient>);
    type Input = LibraryInput;
    type Output = LibraryOutput;
    type CommandOutput = LibraryCommandOutput;

    view! {
        adw::NavigationPage {
            set_tag: Some(AppPage::Library.into()),
            set_title: tr!("library-page-title"),

            #[wrap(Some)]
            set_child = &adw::BreakpointBin {
                set_size_request: (150, 150),

                #[wrap(Some)]
                set_child = &adw::ToolbarView {
                    add_top_bar = &gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,

                        #[name = "header_bar"]
                        adw::HeaderBar {
                            #[name = "view_switcher"]
                            #[wrap(Some)]
                            set_title_widget = &adw::ViewSwitcher {
                                set_policy: adw::ViewSwitcherPolicy::Wide,
                                set_stack: Some(&view_stack),
                            },

                            pack_start = &gtk::ToggleButton {
                                set_icon_name: "loupe",
                                set_tooltip: tr!("library-search-button"),

                                add_binding: (&model.searching, "active"),
                            },

                            pack_end = model.borgar_menu.widget(),
                            pack_end = &gtk::Button::from_icon_name("refresh") {
                                set_tooltip: tr!("library-refresh-button"),
                                connect_clicked[sender] => move |_| {
                                    sender.input(LibraryInput::Refresh);
                                },
                            },
                        },

                        #[name = "search"]
                        SearchBar {
                            set_key_capture_widget: Some(root),
                            add_binding: (&model.searching, "searching"),
                            connect: ("search", false, glib::clone!(@strong sender => move |values| {
                                    let text: String = values[1].get().expect("Failed to get search text");
                                    sender.input(LibraryInput::SearchChanged(text));
                                    None
                                })
                            ),
                        },
                    },

                    #[name = "toaster"]
                    #[wrap(Some)]
                    set_content = &adw::ToastOverlay {
                        #[wrap(Some)]
                        set_child = &gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,

                            #[transition = "Crossfade"]
                            append = match model.state {
                                LibraryState::Loading => {
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
                                                    set_label: tr!("library-loading"),
                                                    add_css_class: "title-2",
                                                },
                                            }
                                        }
                                    }
                                }
                                LibraryState::Ready => {
                                    gtk::Box {
                                        set_orientation: gtk::Orientation::Vertical,

                                        #[name = "view_stack"]
                                        adw::ViewStack {
                                            set_valign: gtk::Align::Fill,

                                            add_named: (model.search_results.widget(), Some("search")),

                                            connect_visible_child_notify[sender] => move |stack| {
                                                if let Some(name) = stack.visible_child_name() {
                                                    sender.input(LibraryInput::ViewStackChildVisible(name.into()));
                                                }
                                            },
                                        },

                                        #[name = "view_switcher_bar"]
                                        adw::ViewSwitcherBar {
                                            set_stack: Some(&view_stack),
                                        },
                                    }
                                }
                                LibraryState::Offline => {
                                    adw::StatusPage {
                                        set_title: tr!("library-offline.title"),
                                        set_description: Some(tr!("library-offline.description")),

                                        set_icon_name: Some("warning"),
                                        #[wrap(Some)]
                                        set_child = &gtk::Button {
                                            set_label: "Refresh",
                                            set_halign: gtk::Align::Center,
                                            set_css_classes: &["pill", "suggested-action"],
                                            connect_clicked[sender] => move |_| {
                                                sender.input(LibraryInput::Refresh);
                                            },
                                        }
                                    }
                                }
                                LibraryState::Error => {
                                    adw::StatusPage {
                                        set_title: tr!("library-error.title"),
                                        set_description: Some(tr!("library-error.description")),

                                        set_icon_name: Some("warning"),
                                        #[wrap(Some)]
                                        set_child = &gtk::Button {
                                            set_label: "Refresh",
                                            set_halign: gtk::Align::Center,
                                            set_css_classes: &["pill", "suggested-action"],
                                            connect_clicked[sender] => move |_| {
                                                sender.input(LibraryInput::Refresh);
                                            },
                                        }
                                    }
                                }
                            },
                        },
                    },
                },

                add_breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
                    adw::BreakpointConditionLengthType::MaxWidth,
                    550.0,
                    adw::LengthUnit::Sp
                )) {
                    add_setter: (&header_bar, "title-widget", &WIDGET_NONE.into()),
                    add_setter: (&view_switcher_bar, "reveal", &true.into()),
                },
            },

            connect_shown[sender] => move |_| {
                sender.input(LibraryInput::Shown);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let (server, account, api_client) = init;

        let model = Library {
            borgar_menu: BorgarMenu::builder()
                .launch(Some(BorgarMenuAuth {
                    api_client: api_client.clone(),
                    server,
                    account,
                }))
                .detach(),
            api_client: Arc::clone(&api_client),
            state: LibraryState::Loading,
            search_results: SearchResults::builder().launch(api_client).detach(),
            home: None,
            collections: HashMap::default(),
            searching: BoolBinding::default(),
            previous_stack_child: Arc::new(RwLock::new("home".into())),
        };

        model.searching.connect_value_notify({
            let sender = sender.clone();
            move |searching| {
                sender.input(LibraryInput::SearchingChanged(searching.value()));
            }
        });

        let widgets = view_output!();
        let view_stack = &widgets.view_stack;

        view_stack.connect_visible_child_name_notify({
            let previous_stack_child = model.previous_stack_child.clone();
            move |view_stack| {
                if let Some(visible_child_name) = view_stack.visible_child_name() {
                    let visible_child_name = visible_child_name.to_string();
                    if visible_child_name != "search"
                        && visible_child_name != *previous_stack_child.read().unwrap()
                    {
                        *previous_stack_child.write().unwrap() = visible_child_name;
                    }
                }
            }
        });

        model.initial_fetch(&sender);

        relm4::ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            LibraryInput::MediaSelected(media) => {
                sender
                    .output(LibraryOutput::PlayVideo(Box::new(media)))
                    .unwrap();
            }
            LibraryInput::Refresh => {
                let view_stack = &widgets.view_stack;

                self.state = LibraryState::Loading;
                self.searching.set_value(false);

                // Clear the current set of pages before loading a new one
                if let Some(home) = self.home.take() {
                    view_stack.remove(home.widget());
                }
                for (_id, collection) in self.collections.drain() {
                    view_stack.remove(collection.widget());
                }

                self.initial_fetch(&sender);
            }
            LibraryInput::Shown => {
                if *LIBRARY_REFRESH_QUEUED.read() {
                    sender.input(LibraryInput::Refresh);
                    *MEDIA_DETAILS_REFRESH_QUEUED.write() = false;
                }
                *LIBRARY_REFRESH_QUEUED.write() = false;
            }
            LibraryInput::ViewStackChildVisible(name) => {
                if let Ok(id) = Uuid::parse_str(&name) {
                    if let Some(collection) = self.collections.get(&id) {
                        collection.emit(CollectionInput::Visible);
                    }
                }

                if name != "search" {
                    self.searching.set_value(false);
                }
            }
            LibraryInput::Toast(message) => {
                let toast = adw::Toast::new(&message);
                widgets.toaster.add_toast(toast);
            }
            LibraryInput::SearchChanged(search_text) => {
                self.search_results
                    .emit(SearchResultsInput::SearchChanged(search_text));
            }
            LibraryInput::SearchingChanged(searching) => {
                if searching {
                    widgets.view_stack.set_visible_child_name("search");
                } else {
                    let previous_stack_child = self.previous_stack_child.read().unwrap();
                    widgets
                        .view_stack
                        .set_visible_child_name(&previous_stack_child);
                }
            }
        }

        self.update_view(widgets, sender);
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
            LibraryCommandOutput::SetLibraryState(state) => {
                self.state = state;
            }
        }

        self.update_view(widgets, sender);
    }
}

impl Library {
    fn initial_fetch(&self, sender: &relm4::ComponentSender<Self>) {
        let api_client = Arc::clone(&self.api_client);
        sender.oneshot_command(async move {
            match api_client.ping().await {
                Ok(_) => {}
                Err(err) => {
                    println!("Error pinging server: {err}");
                    return LibraryCommandOutput::SetLibraryState(LibraryState::Offline);
                }
            }

            match tokio::try_join!(async { api_client.get_user_views().await }, async {
                api_client
                    // We might eventually want client-specific settings, but for
                    // now use the Jellyfin ("emby") client settings
                    .get_user_display_preferences("emby")
                    .await
            }) {
                Ok((user_views, display_preferences)) => {
                    LibraryCommandOutput::LibraryLoaded(user_views, display_preferences)
                }
                Err(err) => {
                    println!("Error loading library: {err}");
                    LibraryCommandOutput::SetLibraryState(LibraryState::Error)
                }
            }
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
        view_stack.add_titled_with_icon(
            home.widget(),
            Some("home"),
            tr!("library-page-home-title"),
            "home-filled",
        );
        self.home = Some(home);

        let user_views: Vec<&UserView> = user_views
            .iter()
            .filter(|view| {
                matches!(
                    view.collection_type,
                    CollectionType::Movies | CollectionType::TvShows
                )
            })
            .collect();

        // TODO: handle overflow when user has too many collections
        // For now we limit them to 5, user can change order in Jellyfin settings
        for &view in user_views.iter().take(5) {
            let collection = Collection::builder()
                .launch((self.api_client.clone(), view.clone()))
                .detach();

            view_stack.add_titled_with_icon(
                collection.widget(),
                Some(&view.id.to_string()),
                &view.name.clone(),
                &view.collection_type.icon(),
            );

            self.collections.insert(view.id, collection);
        }

        view_stack.set_visible_child_name("home");
    }
}
