use adw::prelude::*;
use core::fmt;
use jellyfin_api::types::BaseItemDto;
use relm4::{prelude::*, MessageBroker};
use std::sync::{Arc, RwLock};

use crate::{
    accounts::account_list::{AccountList, AccountListInput, AccountListOutput},
    config::{self, Config},
    jellyfin_api::api_client::ApiClient,
    library::library_component::{Library, LibraryOutput},
    media_details::MediaDetails,
    servers::server_list::{ServerList, ServerListOutput},
    utils::main_window::MAIN_APP_WINDOW_NAME,
    video_player::video_player_component::{VideoPlayer, VideoPlayerInput, VideoPlayerOutput},
};

#[derive(Debug)]
pub enum AppBrokerMessage {
    PlayVideo(BaseItemDto),
}

pub static APP_BROKER: MessageBroker<AppInput> = MessageBroker::new();

#[derive(Clone, Copy, Debug)]
pub enum AppPage {
    Servers,
    Accounts,
    Library,
    MediaDetails,
    VideoPlayer,
}

impl fmt::Display for AppPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppPage::Servers => write!(f, "servers"),
            AppPage::Accounts => write!(f, "accounts"),
            AppPage::Library => write!(f, "library"),
            AppPage::MediaDetails => write!(f, "media_details"),
            AppPage::VideoPlayer => write!(f, "video_player"),
        }
    }
}

pub struct App {
    config: Arc<RwLock<Config>>,
    api_client: Option<Arc<ApiClient>>,
    servers: Controller<ServerList>,
    account_list: Controller<AccountList>,
    library: Option<Controller<Library>>,
    media_details: Option<Controller<MediaDetails>>,
    video_player: Controller<VideoPlayer>,
    server: Option<config::Server>,
    account: Option<config::Account>,
}

#[derive(Debug)]
pub enum AppInput {
    NavigateBack,
    PopToPage(AppPage),
    ServerSelected(config::Server),
    AccountSelected(config::Server, config::Account),
    ShowDetails(BaseItemDto),
    PlayVideo(BaseItemDto),
    SignOut,
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

            #[name = "navigation"]
            #[wrap(Some)]
            set_content = &adw::NavigationView {
                add = model.servers.widget() {
                    set_tag: Some(&AppPage::Servers.to_string()),
                },
                add = model.account_list.widget() {
                    set_tag: Some(&AppPage::Accounts.to_string()),
                },
                add = model.video_player.widget() {
                    set_tag: Some(&AppPage::VideoPlayer.to_string()),
                },
            },
        }
    }

    fn init(
        config: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        // Use development styles when running debug build
        #[cfg(debug_assertions)]
        root.add_css_class("devel");

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
            servers,
            account_list,
            library: None,
            media_details: None,
            video_player,
            server: None,
            account: None,
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
        let navigation = &widgets.navigation;

        match message {
            AppInput::NavigateBack => {
                navigation.pop();
            }
            AppInput::PopToPage(page) => {
                navigation.pop_to_tag(&page.to_string());
            }
            AppInput::ServerSelected(server) => {
                self.server = Some(server.clone());
                self.account_list.emit(AccountListInput::SetServer(server));
                navigation.push_by_tag(&AppPage::Accounts.to_string());
            }
            AppInput::AccountSelected(server, account) => {
                self.account = Some(account.clone());

                let api_client = ApiClient::new(Arc::clone(&self.config), &server, &account);
                let api_client = Arc::new(api_client);
                self.api_client = Some(api_client.clone());

                let library = Library::builder()
                    .launch((self.config.clone(), server, account, api_client))
                    .forward(sender.input_sender(), convert_library_output);
                navigation.push(library.widget());
                self.library = Some(library);
            }
            AppInput::ShowDetails(media) => {
                if let (Some(api_client), Some(server), Some(account)) =
                    (&self.api_client, &self.server, &self.account)
                {
                    let media_details = MediaDetails::builder()
                        .launch((
                            api_client.clone(),
                            media,
                            self.config.clone(),
                            server.clone(),
                            account.clone(),
                        ))
                        .detach();
                    navigation.push(media_details.widget());
                    self.media_details = Some(media_details);
                }
            }
            AppInput::PlayVideo(item) => {
                if let (Some(api_client), Some(server)) = (&self.api_client, &self.server) {
                    self.video_player.emit(VideoPlayerInput::PlayVideo(
                        api_client.clone(),
                        server.clone(),
                        Box::new(item),
                    ));
                    navigation.push_by_tag(&AppPage::VideoPlayer.to_string());
                }
            }
            AppInput::SignOut => {
                navigation.pop_to_tag(&AppPage::Servers.to_string());
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
        LibraryOutput::PlayVideo(media) => AppInput::PlayVideo(*media),
    }
}
