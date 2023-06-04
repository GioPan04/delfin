use adw::prelude::*;
use core::fmt;
use relm4::prelude::*;
use std::sync::{Arc, RwLock};

use crate::{
    accounts::account_list::{AccountList, AccountListInput},
    config::{self, Config},
    servers::server_list::{ServerList, ServerListOutput},
    video_player::video_player_component::VideoPlayer,
};

#[derive(Clone, Copy, Debug)]
pub enum AppPage {
    Servers,
    Accounts,
    VideoPlayer,
}

impl fmt::Display for AppPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppPage::Servers => write!(f, "servers"),
            AppPage::Accounts => write!(f, "accounts"),
            AppPage::VideoPlayer => write!(f, "video_player"),
        }
    }
}

pub struct App {
    page: AppPage,
    servers: Controller<ServerList>,
    account_list: Controller<AccountList>,
    video_player: Controller<VideoPlayer>,
}

#[derive(Debug)]
pub enum AppInput {
    SetPage(AppPage),
    NavigateBack,
    ServerSelected(config::Server),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = Arc<RwLock<Config>>;
    type Input = AppInput;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Jellything"),
            set_default_width: 960,
            set_default_height: 540,

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                adw::HeaderBar {
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

                gtk::Stack {
                    set_transition_type: gtk::StackTransitionType::SlideLeft,

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
        let account_list = AccountList::builder().launch(Arc::clone(&config)).detach();
        let video_player = VideoPlayer::builder().launch(()).detach();

        let model = App {
            page: AppPage::Servers,
            servers,
            account_list,
            video_player,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            AppInput::SetPage(page) => self.page = page,
            AppInput::NavigateBack => {
                // TODO: need to handle stack transition, right now it always
                // slides right, even when we go back
                self.page = match self.page {
                    AppPage::Accounts => AppPage::Servers,
                    _ => self.page,
                }
            }
            AppInput::ServerSelected(server) => {
                self.account_list.emit(AccountListInput::SetServer(server));
                sender.input(AppInput::SetPage(AppPage::Accounts));
            }
        }
    }
}

fn convert_server_list_output(output: ServerListOutput) -> AppInput {
    match output {
        ServerListOutput::ServerSelected(server) => AppInput::ServerSelected(server),
    }
}
