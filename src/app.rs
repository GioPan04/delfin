use adw::prelude::*;
use core::fmt;
use relm4::prelude::*;
use std::sync::{Arc, RwLock};

use crate::{
    accounts::account_list::{AccountList, AccountListInput, AccountListOutput},
    config::{self, Config},
    jellyfin_api::{api::latest::LatestMedia, api_client::ApiClient},
    library::library_component::{Library, LibraryOutput},
    main_window::MAIN_APP_WINDOW_NAME,
    servers::server_list::{ServerList, ServerListOutput},
    video_player::video_player_component::{VideoPlayer, VideoPlayerInput, VideoPlayerOutput},
};

#[derive(Clone, Copy, Debug)]
pub enum AppPage {
    Servers,
    Accounts,
    Library,
    VideoPlayer,
}

impl AppPage {
    fn back(&self) -> Self {
        match self {
            AppPage::Servers => AppPage::Servers,
            AppPage::Accounts => AppPage::Servers,
            AppPage::Library => AppPage::Accounts,
            AppPage::VideoPlayer => AppPage::Library,
        }
    }
}

impl fmt::Display for AppPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppPage::Servers => write!(f, "servers"),
            AppPage::Accounts => write!(f, "accounts"),
            AppPage::Library => write!(f, "library"),
            AppPage::VideoPlayer => write!(f, "video_player"),
        }
    }
}

pub struct App {
    config: Arc<RwLock<Config>>,
    api_client: Option<Arc<ApiClient>>,
    page: AppPage,
    servers: Controller<ServerList>,
    account_list: Controller<AccountList>,
    library: Option<Controller<Library>>,
    video_player: Controller<VideoPlayer>,
    server: Option<config::Server>,
}

#[derive(Debug)]
pub enum AppInput {
    SetPage(AppPage),
    NavigateBack,
    ServerSelected(config::Server),
    AccountSelected(config::Server, config::Account),
    PlayVideo(LatestMedia),
}

#[relm4::component(pub)]
impl Component for App {
    type Init = Arc<RwLock<Config>>;
    type Input = AppInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::ApplicationWindow {
            set_widget_name: MAIN_APP_WINDOW_NAME,
            set_title: Some("Jellything"),
            set_default_width: 960,
            set_default_height: 540,

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                adw::HeaderBar {
                    #[watch]
                    set_visible: matches!(model.page, AppPage::Servers) || matches!(model.page, AppPage::Accounts),
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Jellything",
                    },
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous",
                        #[watch]
                        set_visible: !matches!(model.page, AppPage::Servers),
                        connect_clicked[sender] => move |_| {
                            sender.input(AppInput::NavigateBack);
                        },
                    },
                },

                #[name = "stack"]
                gtk::Stack {
                    add_child = model.servers.widget() {} -> {
                        set_name: &AppPage::Servers.to_string(),
                    },

                    add_child = model.account_list.widget() {} -> {
                        set_name: &AppPage::Accounts.to_string(),
                    },

                    add_child = model.video_player.widget() {} -> {
                        set_name: &AppPage::VideoPlayer.to_string(),
                    },

                    #[watch]
                    set_visible_child_name: &model.page.to_string(),
                },
            }
        }
    }

    fn init(
        config: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let servers = ServerList::builder()
            .launch(Arc::clone(&config))
            .forward(sender.input_sender(), convert_server_list_output);

        let account_list = AccountList::builder()
            .launch(Arc::clone(&config))
            .forward(sender.input_sender(), convert_account_list_output);

        let video_player = VideoPlayer::builder()
            .launch(Arc::clone(&config))
            .forward(sender.input_sender(), convert_video_player_output);

        let model = App {
            config,
            api_client: None,
            page: AppPage::Servers,
            servers,
            account_list,
            library: None,
            video_player,
            server: None,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AppInput::SetPage(page) => {
                let stack = &widgets.stack;

                let cur = self.page as u8;
                let next = page as u8;

                match cur.partial_cmp(&next) {
                    Some(std::cmp::Ordering::Less) => {
                        stack.set_transition_type(gtk::StackTransitionType::SlideLeft)
                    }
                    Some(std::cmp::Ordering::Greater) => {
                        stack.set_transition_type(gtk::StackTransitionType::SlideRight)
                    }
                    _ => return,
                }

                self.page = page;
            }
            AppInput::NavigateBack => {
                sender.input(AppInput::SetPage(self.page.back()));
            }
            AppInput::ServerSelected(server) => {
                self.server = Some(server.clone());
                self.account_list.emit(AccountListInput::SetServer(server));
                sender.input(AppInput::SetPage(AppPage::Accounts));
            }
            AppInput::AccountSelected(server, account) => {
                let stack = &widgets.stack;

                if let Some(previous) = &self.library {
                    stack.remove(previous.widget());
                }

                let api_client = ApiClient::new(Arc::clone(&self.config), &server, &account);
                let api_client = Arc::new(api_client);
                self.api_client = Some(api_client.clone());

                let library = Library::builder()
                    .launch(api_client)
                    .forward(sender.input_sender(), convert_library_output);
                stack.add_named(library.widget(), Some(&AppPage::Library.to_string()));
                self.library = Some(library);

                sender.input(AppInput::SetPage(AppPage::Library));
            }
            AppInput::PlayVideo(media) => {
                if let (Some(api_client), Some(server)) = (&self.api_client, &self.server) {
                    self.video_player.emit(VideoPlayerInput::PlayVideo(
                        api_client.clone(),
                        server.clone(),
                        media,
                    ));
                    sender.input(AppInput::SetPage(AppPage::VideoPlayer));
                }
            }
        }

        self.update_view(widgets, sender);
    }
}

fn convert_server_list_output(output: ServerListOutput) -> AppInput {
    match output {
        ServerListOutput::ServerSelected(server) => AppInput::ServerSelected(server),
    }
}

fn convert_account_list_output(output: AccountListOutput) -> AppInput {
    match output {
        AccountListOutput::AccountSelected(server, account) => {
            AppInput::AccountSelected(server, account)
        }
    }
}

fn convert_video_player_output(output: VideoPlayerOutput) -> AppInput {
    match output {
        VideoPlayerOutput::NavigateBack => AppInput::NavigateBack,
    }
}

fn convert_library_output(output: LibraryOutput) -> AppInput {
    match output {
        LibraryOutput::NavigateBack => AppInput::NavigateBack,
        LibraryOutput::PlayVideo(media) => AppInput::PlayVideo(media),
    }
}
