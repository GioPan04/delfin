use adw::prelude::*;
use relm4::prelude::*;

use crate::{config, jellyfin_api::api::system::get_public_server_info, tr};

#[derive(Clone, Debug)]
pub enum ValidationState {
    Invalid,
    Loading,
    Valid(config::Server),
    Error,
}

#[derive(Clone)]
pub struct AddServerDialog {
    valid: ValidationState,
}

#[derive(Debug)]
pub enum AddServerInput {
    ValidateServer(String),
    Invalidate,
    AddServer,
}

#[derive(Debug)]
pub enum AddServerOutput {
    ServerAdded(config::Server),
}

#[derive(Debug)]
pub enum AddServerCommandOutput {
    ServerValidated(ValidationState),
}

#[relm4::component(pub)]
impl Component for AddServerDialog {
    type Init = ();
    type Input = AddServerInput;
    type Output = AddServerOutput;
    type CommandOutput = AddServerCommandOutput;

    view! {
        adw::Window {
            set_application: Some(&relm4::main_application()),
            set_title: Some(tr!("server-list-add-server-title")),
            set_modal: true,
            set_visible: true,

            #[wrap(Some)]
            set_content = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_all: 20,
                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 20,

                        adw::PreferencesGroup {
                            #[name = "url_entry"]
                            adw::EntryRow {
                                set_title: tr!("server-list-add-server-url"),
                                set_show_apply_button: true,
                                #[watch]
                                set_editable: !matches!(model.valid, ValidationState::Loading),
                                set_input_purpose: gtk::InputPurpose::Url,

                                connect_changed[sender] => move |_| { sender.input(AddServerInput::Invalidate); },
                                connect_apply[sender] => move |entry| {
                                    sender.input(AddServerInput::ValidateServer(entry.text().to_string()));
                                },
                            },

                            adw::ActionRow {
                                set_title: tr!("server-list-add-server-name"),
                                #[watch]
                                set_subtitle: if let ValidationState::Valid(server) = &model.valid {
                                    &server.name
                                } else {
                                    "..."
                                },
                                add_suffix = if matches!(model.valid, ValidationState::Loading) {
                                    gtk::Spinner {
                                        set_spinning: true,
                                    }
                                } else if matches!(model.valid, ValidationState::Valid(_)) {
                                    gtk::Image {
                                        set_icon_name: Some("check-round-outline-symbolic")
                                    }
                                } else { gtk::Label {} },
                            },
                        },

                        #[name = "submit_btn"]
                        gtk::Button {
                            set_halign: gtk::Align::Center,
                            set_label: tr!("server-list-add-server-submit-button"),
                            add_css_class: "pill",
                            add_css_class: "suggested-action",
                            #[watch]
                            set_sensitive: matches!(model.valid, ValidationState::Valid(_)),
                            connect_clicked[sender] => move |_| {
                                sender.input(AddServerInput::AddServer);
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
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AddServerDialog {
            valid: ValidationState::Invalid,
        };
        let widgets = view_output!();
        widgets.url_entry.grab_focus();
        root.set_default_widget(Some(&widgets.submit_btn));
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            AddServerInput::ValidateServer(url) => {
                self.valid = ValidationState::Loading;
                sender.oneshot_command(async move {
                    let public_server_info = get_public_server_info(&url).await;
                    if let Ok(public_server_info) = public_server_info {
                        return AddServerCommandOutput::ServerValidated(ValidationState::Valid(
                            config::Server {
                                id: public_server_info.id,
                                url,
                                name: public_server_info.server_name,
                                accounts: Vec::new(),
                            },
                        ));
                    }
                    println!("Error getting server info: {:#?}", public_server_info);
                    AddServerCommandOutput::ServerValidated(ValidationState::Error)
                });
            }
            AddServerInput::Invalidate => self.valid = ValidationState::Invalid,
            AddServerInput::AddServer => {
                if let ValidationState::Valid(server) = &self.valid {
                    sender
                        .output(AddServerOutput::ServerAdded(server.clone()))
                        .unwrap();
                    root.close();
                }
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            AddServerCommandOutput::ServerValidated(valid) => self.valid = valid,
        }
    }
}
