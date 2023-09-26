use adw::{prelude::*, ResponseAppearance};
use relm4::prelude::*;
use std::sync::{Arc, RwLock};

use crate::{
    app::{AppInput, AppPage, APP_BROKER},
    config::{Account, Config, Server},
    jellyfin_api::api_client::ApiClient,
};

enum Responses {
    Cancel,
    SignOut,
}

impl From<Responses> for &str {
    fn from(val: Responses) -> Self {
        match val {
            Responses::Cancel => "cancel",
            Responses::SignOut => "sign_out",
        }
    }
}

pub(crate) struct SignOutDialog {
    api_client: Arc<ApiClient>,
    config: Arc<RwLock<Config>>,
    server: Server,
    account: Account,
}

#[derive(Debug)]
pub(crate) enum SignOutDialogInput {
    SignOut,
}

#[relm4::component(pub(crate))]
impl Component for SignOutDialog {
    type Init = (Arc<ApiClient>, Arc<RwLock<Config>>, Server, Account);
    type Input = SignOutDialogInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::MessageDialog {
            set_visible: true,
            set_modal: true,
            set_heading: Some("Sign Out"),
            set_body: "Do you want to sign out and remove this account from Jellything?",
            add_responses: &[
                (Responses::Cancel.into(), "Cancel"),
                (Responses::SignOut.into(), "Sign Out"),
            ],
            set_default_response: Some(Responses::Cancel.into()),
            set_close_response: Responses::Cancel.into(),
            set_response_appearance: (Responses::SignOut.into(), ResponseAppearance::Destructive),
            connect_response: (Some(Responses::SignOut.into()), move |_, _| {
                sender.input(SignOutDialogInput::SignOut);
            }),

            #[name = "remove_server"]
            #[wrap(Some)]
            set_extra_child = &gtk::CheckButton {
                set_visible: false,
                set_label: Some("Remove this server from Jellything"),
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, config, server, account) = init;

        let model = SignOutDialog {
            api_client,
            config: config.clone(),
            server: server.clone(),
            account,
        };

        let widgets = view_output!();
        let remove_server = &widgets.remove_server;

        let config = config.read().unwrap();
        if let Some(current_server) = config.servers.iter().find(|s| s.id == server.id) {
            if current_server.accounts.len() < 2 {
                remove_server.set_visible(true);
            }
        }

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        _message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        tokio::task::spawn({
            let api_client = self.api_client.clone();
            async move {
                api_client.sign_out().await.unwrap();
            }
        });

        let remove_server = widgets.remove_server.is_active();

        let mut config = self.config.write().unwrap();
        let mut servers = config.servers.clone();
        if remove_server {
            servers.retain(|s| s.id != self.server.id);
        } else {
            let server = servers.iter_mut().find(|s| s.id == self.server.id).unwrap();
            server.accounts.retain(|a| a.id != self.account.id);
        }
        config.servers = servers;
        config.save().unwrap();

        APP_BROKER.send(AppInput::PopToPage(if remove_server {
            AppPage::Servers
        } else {
            AppPage::Accounts
        }));

        self.update_view(widgets, sender);
    }
}
