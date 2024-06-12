use adw::prelude::*;
use relm4::{adw, gtk, prelude::*, Component, ComponentParts};
use tracing::error;
use uuid::Uuid;

use crate::{
    config::Server,
    jellyfin_api::api::user::{authenticate_by_name, AuthenticateByNameRes, authenticate_by_pin},
    tr,
};

#[derive(Debug, Default)]
enum ValidationState {
    #[default]
    Invalid,
    Loading,
}

#[derive(Default)]
pub struct AddAccountDialog {
    server: Server,
    device_id: Uuid,
    toaster: adw::ToastOverlay,
    username: String,
    password: String,
    valid: ValidationState,
}

#[derive(Debug)]
pub enum AddAccountInput {
    Toast(String),
    UsernameChanged(String),
    PasswordChanged(String),
    QuickConnectEnabled,
    SignIn,
}

#[derive(Debug)]
pub enum AddAccountOutput {
    AccountAdded(AuthenticateByNameRes),
}

#[derive(Debug)]
pub enum AddAccountCommandOutput {
    SignInSuccess(AuthenticateByNameRes),
    SignInFail(anyhow::Error),
}

#[relm4::component(pub)]
impl Component for AddAccountDialog {
    type Init = Server;
    type Input = AddAccountInput;
    type Output = AddAccountOutput;
    type CommandOutput = AddAccountCommandOutput;

    view! {
        adw::Window {
            set_application: Some(&relm4::main_application()),
            set_title: Some(tr!("account-list-add-account-title")),
            set_modal: true,
            set_visible: true,

            #[wrap(Some)]
            set_content = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_all: 20,

                    #[local_ref]
                    toaster -> adw::ToastOverlay {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical,
                            set_spacing: 20,

                            adw::PreferencesGroup {
                                adw::ActionRow {
                                    set_title: "Use Quick Connect",// change this later!
                                    set_subtitle: "Use a PIN to connect to your Jellyfin Instance",
                                    set_activatable: true,
                                    connect_activated[sender] => move |_| {
                                        sender.input(AddAccountInput::QuickConnectEnabled);
                                    },
                                    add_suffix = &gtk::Image {
                                        set_icon_next: Some("go-next-symbolic")
                                    }
                                },
                                adw::EntryRow {
                                    set_title: tr!("account-list-add-account-username"),
                                    set_activates_default: true,
                                    connect_changed[sender] => move |entry| {
                                        sender.input(AddAccountInput::UsernameChanged(entry.text().to_string()))
                                    },
                                },
                                adw::PasswordEntryRow {
                                    set_title: tr!("account-list-add-account-password"),
                                    set_activates_default: true,
                                    connect_changed[sender] => move |entry| {
                                        sender.input(AddAccountInput::PasswordChanged(entry.text().to_string()))
                                    },
                                },
                            },

                            #[transition = "Crossfade"]
                            append = if matches!(model.valid, ValidationState::Invalid) {
                                #[name = "submit_btn"]
                                gtk::Button {
                                    set_halign: gtk::Align::Center,
                                    set_label: tr!("account-list-add-account-submit-button"),
                                    add_css_class: "pill",
                                    add_css_class: "suggested-action",
                                    #[watch]
                                    set_sensitive: !model.username.is_empty(),
                                    connect_clicked[sender] => move |_| {
                                        sender.input(AddAccountInput::SignIn);
                                    }
                                }
                            } else {
                                gtk::Spinner { set_spinning: true }
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        server: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = AddAccountDialog {
            server,
            device_id: Uuid::new_v4(),
            toaster: adw::ToastOverlay::new(),
            username: String::new(),
            password: String::new(),
            valid: ValidationState::default(),
        };
        let toaster = &model.toaster;
        let widgets = view_output!();
        root.set_default_widget(Some(&widgets.submit_btn));
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            AddAccountInput::QuickConnectEnabled => {
                let url = self.server.url.clone();
                sender.oneshot_command(async move {
                    let device_id = self.device_id;
                    let auth_info = authenticate_by_pin(&url, &device_id).await;
                    match auth_info {
                        Ok(auth_info) => AddAccountCommandOutput::SignInSuccess(auth_info),
                        Err(err) => AddAccountCommandOutput::SignInFail(err)
                    }
                });

            }
            AddAccountInput::Toast(message) => {
                let toast = adw::Toast::new(&message);
                toast.set_timeout(3);
                self.toaster.add_toast(toast);
            }
            AddAccountInput::UsernameChanged(username) => self.username = username,
            AddAccountInput::PasswordChanged(password) => self.password = password,
            AddAccountInput::SignIn => {
                self.valid = ValidationState::Loading;
                let url = self.server.url.clone();
                let device_id = self.device_id;
                let username = self.username.clone();
                let password = self.password.clone();
                sender.oneshot_command(async move {
                    let auth_info =
                        authenticate_by_name(&url, &device_id, &username, &password).await;
                    match auth_info {
                        Ok(auth_info) => AddAccountCommandOutput::SignInSuccess(auth_info),
                        Err(err) => AddAccountCommandOutput::SignInFail(err),
                    }
                });
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            AddAccountCommandOutput::SignInSuccess(res) => {
                sender.output(AddAccountOutput::AccountAdded(res)).unwrap();
                root.close();
            }
            AddAccountCommandOutput::SignInFail(err) => {
                error!("Sign in failed: {:#?}", err);
                sender.input(AddAccountInput::Toast(err.to_string()));
                self.valid = ValidationState::Invalid;
            }
        }
    }
}
