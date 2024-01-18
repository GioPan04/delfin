use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::{
    borgar::borgar_menu::BorgarMenu,
    config::{general::MostRecentLogin, Account, Server},
    globals::CONFIG,
    jellyfin_api::api::user::AuthenticateByNameRes,
    tr,
    utils::constants::PAGE_MARGIN,
};

use super::{
    account_list_item::{AccountListItem, AccountListItemOutput},
    add_account::{AddAccountDialog, AddAccountOutput},
};

pub struct AccountList {
    server: Server,
    accounts: FactoryVecDeque<AccountListItem>,
    add_account_dialog: Option<Controller<AddAccountDialog>>,
    borgar: Controller<BorgarMenu>,
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
    type Init = ();
    type Input = AccountListInput;
    type Output = AccountListOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: tr!("account-list-page-title"),

            connect_shown[sender] => move |_| {
                sender.input(AccountListInput::ReloadAccounts);
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = model.borgar.widget(),
                },

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_top: PAGE_MARGIN,

                    adw::PreferencesGroup {
                        #[watch]
                        set_title: tr!("account-list.title", { "serverName" => &*model.server.name }),
                        set_description: Some(tr!("account-list.description")),
                        #[wrap(Some)]
                        set_header_suffix = &gtk::Button {
                            connect_clicked[sender] => move |_| {
                                sender.input(AccountListInput::AddAccount);
                            },
                            adw::ButtonContent {
                                set_icon_name: "list-add-symbolic",
                                set_label: tr!("account-list-add-account-button"),
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
                                set_title: tr!("account-list-empty.title"),
                                set_subtitle: tr!("account-list-empty.subtitle"),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let accounts = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), convert_account_list_item_output);

        let model = AccountList {
            server: Server::default(),
            accounts,
            add_account_dialog: None,
            borgar: BorgarMenu::builder().launch(None).detach(),
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
                if let Some(server) = CONFIG
                    .read()
                    .servers
                    .iter()
                    .find(|s| s.id == self.server.id)
                {
                    for account in &server.accounts {
                        accounts.push_back((self.server.url.clone(), account.clone()));
                    }
                }
            }
            AccountListInput::SetServer(server) => {
                self.server = server.clone();
                sender.input(AccountListInput::ReloadAccounts);
            }
            AccountListInput::AddAccount => {
                self.add_account_dialog = Some(
                    AddAccountDialog::builder()
                        .transient_for(root)
                        .launch(self.server.clone())
                        .forward(sender.input_sender(), convert_add_account_output),
                );
            }
            AccountListInput::AccountAdded(auth_info) => {
                let account: Account = auth_info.into();
                self.accounts
                    .guard()
                    .push_front((self.server.url.clone(), account.clone()));
                let mut config = CONFIG.write();
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
                let mut config = CONFIG.write();

                let server = config
                    .servers
                    .clone()
                    .into_iter()
                    .find(|s| s.id == self.server.id);

                if let Some(server) = server {
                    let account = &server.accounts[index];

                    config.general.most_recent_login = Some(MostRecentLogin {
                        server_id: server.id,
                        account_id: account.id,
                    });
                    config.save().expect("Failed to update previous login");

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

fn convert_account_list_item_output(output: AccountListItemOutput) -> AccountListInput {
    match output {
        AccountListItemOutput::AccountSelected(index) => AccountListInput::AcountSelected(index),
    }
}

impl From<AuthenticateByNameRes> for Account {
    fn from(val: AuthenticateByNameRes) -> Self {
        Account {
            id: val.user.id,
            username: val.user.name,
            access_token: val.access_token,
            device_id: val.session_info.device_id,
        }
    }
}
