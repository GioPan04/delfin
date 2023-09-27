use adw::prelude::*;
use relm4::{
    adw, factory::FactoryVecDeque, gtk, prelude::DynamicIndex, Component, ComponentParts,
    Controller,
};

use crate::{config, globals::CONFIG, utils::constants::PAGE_MARGIN};

use super::{
    add_server::{AddServerDialog, AddServerOutput},
    server_list_item::ServerListItem,
};

pub struct ServerList {
    servers: FactoryVecDeque<ServerListItem>,
    add_server_dialog: Option<Controller<AddServerDialog>>,
}

#[derive(Debug)]
pub enum ServerListInput {
    ReloadServers,
    AddServer,
    ServerAdded(config::Server),
    ServerSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum ServerListOutput {
    ServerSelected(config::Server),
}

#[relm4::component(pub)]
impl Component for ServerList {
    type Init = ();
    type Input = ServerListInput;
    type Output = ServerListOutput;
    type CommandOutput = ();

    view! {
        adw::NavigationPage {
            set_title: "Servers",

            connect_showing[sender] => move |_| {
                sender.input(ServerListInput::ReloadServers);
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {},

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_top: PAGE_MARGIN,

                    adw::PreferencesGroup {
                        set_title: "Select a server",
                        set_description: Some("Choose which Jellyfin server you'd like to use"),
                        #[wrap(Some)]
                        set_header_suffix = &gtk::Button {
                            connect_clicked[sender] => move |_| {
                                sender.input(ServerListInput::AddServer);
                            },
                            adw::ButtonContent {
                                set_icon_name: "list-add-symbolic",
                                set_label: "Add a server",
                            },
                        },

                        #[local_ref]
                        servers_box -> gtk::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk::SelectionMode::None,
                        },

                        gtk::ListBox {
                            add_css_class: "boxed-list",
                            set_selection_mode: gtk::SelectionMode::None,
                            #[watch]
                            set_visible: model.servers.is_empty(),
                            adw::ActionRow {
                                set_title: "No Servers Available",
                                set_subtitle: "Add a server to start watching",
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
        let servers = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());

        let model = ServerList {
            servers,
            add_server_dialog: None,
        };

        let servers_box = model.servers.widget();

        let widgets = view_output!();

        sender.input(ServerListInput::ReloadServers);

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            ServerListInput::ReloadServers => {
                let mut servers = self.servers.guard();
                servers.clear();
                for server in &CONFIG.read().servers {
                    servers.push_back(server.clone());
                }
            }
            ServerListInput::AddServer => {
                self.add_server_dialog = Some(
                    AddServerDialog::builder()
                        .transient_for(root)
                        .launch(())
                        .forward(sender.input_sender(), convert_add_server_output),
                );
            }
            ServerListInput::ServerAdded(server) => {
                self.servers.guard().push_back(server.clone());
                let mut config = CONFIG.write();
                config.servers.push(server);
                config.save().unwrap();
            }
            ServerListInput::ServerSelected(index) => {
                let index: usize = index.current_index();
                let server = &CONFIG.read().servers[index];
                sender
                    .output(ServerListOutput::ServerSelected(server.clone()))
                    .unwrap();
            }
        };
    }
}

fn convert_add_server_output(output: AddServerOutput) -> ServerListInput {
    match output {
        AddServerOutput::ServerAdded(server) => ServerListInput::ServerAdded(server),
    }
}
