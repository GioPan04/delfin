use std::sync::{Arc, RwLock};

use adw::prelude::*;
use relm4::{
    adw, factory::FactoryVecDeque, gtk, prelude::DynamicIndex, Component, ComponentParts,
    Controller,
};

use crate::{
    config::{Account, Config, Server},
    jellyfin_api::api::user::AuthenticateByNameRes,
    utils::constants::PAGE_MARGIN,
};

use super::{
    account_list_item::AccountListItem,
    add_account::{AddAccountDialog, AddAccountOutput},
};

pub struct AccountList {
    config: Arc<RwLock<Config>>,
    server: Server,
    accounts: FactoryVecDeque<AccountListItem>,
    add_account_dialog: Option<Controller<AddAccountDialog>>,
}

#[derive(Debug)]
pub enum AccountListInput {
    ReloadAccounts,
    SetServer(Server),
    AddAccount,
    AccountAdded(AuthenticateByNameRes),
    AcountSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum AccountListOutput {
    AccountSelected(Server, Account),
}

#[relm4::component(pub)]
impl Component for AccountList {
    type Init = Arc<RwLock<Config>>;
    type Input = AccountListInput;
    type Output = AccountListOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: "Accounts",

            connect_showing[sender] => move |_| {
                sender.input(AccountListInput::ReloadAccounts);
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_top: PAGE_MARGIN,

                    adw::PreferencesGroup {
                        #[watch]
                        set_title: &format!("Sign in to {}", &model.server.name),
                        set_description: Some("Choose which account you'd like to sign in as"),
                        #[wrap(Some)]
                        set_header_suffix = &gtk::Button {
                            connect_clicked[sender] => move |_| {
                                sender.input(AccountListInput::AddAccount);
                            },
                            adw::ButtonContent {
                                set_icon_name: "list-add-symbolic",
                                set_label: "Add an account",
                            },
                        },

                        #[local_ref]
                        accounts_box -> gtk::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk::SelectionMode::None,
                        },

                        // Empty state
                        gtk::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk::SelectionMode::None,
                            #[watch]
                            set_visible: model.accounts.is_empty(),
                            adw::ActionRow {
                                set_title: "No Accounts Available",
                                set_subtitle: "Add an account to start watching",
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        config: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AccountList {
            config,
            server: Server::default(),
            accounts: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
            add_account_dialog: None,
        };

        let accounts_box = model.accounts.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            AccountListInput::ReloadAccounts => {
                let mut accounts = self.accounts.guard();
                accounts.clear();
                for account in &self.server.accounts {
                    accounts.push_back((self.server.url.clone(), account.clone()));
                }
            }
            AccountListInput::SetServer(server) => {
                self.server = server.clone();

                self.accounts.guard().clear();
                for account in &server.accounts {
                    self.accounts
                        .guard()
                        .push_back((self.server.url.clone(), account.clone()));
                }
            }
            AccountListInput::AddAccount => {
                let config = self.config.read().unwrap();
                self.add_account_dialog = Some(
                    AddAccountDialog::builder()
                        .transient_for(root)
                        .launch((self.server.clone(), config.device_id.clone()))
                        .forward(sender.input_sender(), convert_add_account_output),
                );
            }
            AccountListInput::AccountAdded(auth_info) => {
                let account = Account {
                    id: auth_info.user.id,
                    username: auth_info.user.name,
                    access_token: auth_info.access_token,
                };
                self.accounts
                    .guard()
                    .push_front((self.server.url.clone(), account.clone()));
                let mut config = self.config.write().unwrap();
                let server = config.servers.iter_mut().find(|s| s.id == self.server.id);
                if let Some(server) = server {
                    server.accounts.insert(0, account.clone());
                    sender
                        .output(AccountListOutput::AccountSelected(server.clone(), account))
                        .unwrap();
                }
                config.save().unwrap();
            }
            AccountListInput::AcountSelected(index) => {
                let index = index.current_index();
                let config = self.config.read().unwrap();
                let server = config.servers.iter().find(|s| s.id == self.server.id);
                if let Some(server) = server {
                    let account = &server.accounts[index];
                    sender
                        .output(AccountListOutput::AccountSelected(
                            server.clone(),
                            account.clone(),
                        ))
                        .unwrap();
                }
            }
        }
    }
}

fn convert_add_account_output(output: AddAccountOutput) -> AccountListInput {
    match output {
        AddAccountOutput::AccountAdded(auth_info) => AccountListInput::AccountAdded(auth_info),
    }
}
