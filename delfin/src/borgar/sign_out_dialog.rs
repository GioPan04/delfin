use adw::{prelude::*, ResponseAppearance};
use relm4::prelude::*;
use std::sync::Arc;

use crate::{
    app::{AppInput, AppPage, APP_BROKER},
    config::{Account, Server},
    globals::CONFIG,
    jellyfin_api::api_client::ApiClient,
    tr,
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
    server: Server,
    account: Account,
}

#[derive(Debug)]
pub(crate) enum SignOutDialogInput {
    SignOut,
}

#[relm4::component(pub(crate))]
impl Component for SignOutDialog {
    type Init = (Arc<ApiClient>, Server, Account);
    type Input = SignOutDialogInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::MessageDialog {
            set_visible: true,
            set_modal: true,
            set_heading: Some(tr!("borgar-sign-out-dialog.heading")),
            set_body: tr!("borgar-sign-out-dialog.body"),
            add_responses: &[
                (Responses::Cancel.into(), tr!("borgar-sign-out-dialog.response-cancel")),
                (Responses::SignOut.into(), tr!("borgar-sign-out-dialog.response-sign-out")),
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
                set_label: Some(tr!("borgar-sign-out-dialog-remove-server")),
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, server, account) = init;

        let model = SignOutDialog {
            api_client,
            server: server.clone(),
            account,
        };

        let widgets = view_output!();
        let remove_server = &widgets.remove_server;

        let config = CONFIG.read();
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

        let mut config = CONFIG.write();
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
