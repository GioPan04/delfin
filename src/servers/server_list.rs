use std::sync::RwLock;

use adw::prelude::*;
use relm4::{
    adw, factory::FactoryVecDeque, gtk, prelude::DynamicIndex, Component, ComponentParts,
    Controller,
};

use crate::config::{self, Config};

use super::{
    add_server::{AddServerDialog, AddServerOutput},
    server_list_item::ServerListItem,
};

pub struct ServerList {
    config: RwLock<Config>,
    servers: FactoryVecDeque<ServerListItem>,
    add_server_dialog: Option<Controller<AddServerDialog>>,
}

#[derive(Debug)]
pub enum ServerListInput {
    AddServer,
    ServerAdded(config::Server),
    ServerSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum ServerListOutput {
    ServerSelected(String),
}

#[relm4::component(pub)]
impl Component for ServerList {
    type Init = RwLock<Config>;
    type Input = ServerListInput;
    type Output = ServerListOutput;
    type CommandOutput = ();

    view! {
        adw::Clamp {
            adw::PreferencesGroup {
                set_title: "Select a server",
                set_description: Some("Choose which Jellyfin server you'd like to use"),
                #[wrap(Some)]
                set_header_suffix = &gtk::Button {
                    set_tooltip_text: Some("Add a server"),
                    set_icon_name: "list-add-symbolic",
                    connect_clicked[sender] => move |_| {
                        sender.input(ServerListInput::AddServer);
                    },
                },

                #[local_ref]
                servers_box -> gtk::ListBox {
                    add_css_class: "boxed-list",
                    set_selection_mode: gtk::SelectionMode::None,
                },
            },
        }
    }

    fn init(
        config: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut servers = FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender());
        for server in &config.read().unwrap().servers {
            servers.guard().push_back(server.clone());
        }

        let model = ServerList {
            config,
            servers,
            add_server_dialog: None,
        };

        let servers_box = model.servers.widget();

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        _widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            ServerListInput::AddServer => {
                self.add_server_dialog = Some(
                    AddServerDialog::builder()
                        .transient_for(&root)
                        .launch(())
                        .forward(sender.input_sender(), convert_add_server_output),
                );
            }
            ServerListInput::ServerAdded(server) => {
                self.servers.guard().push_back(server.clone());
                let mut config = self.config.write().unwrap();
                config.servers.push(server);
                config.save().unwrap();
            }
            ServerListInput::ServerSelected(index) => {
                let server = &self.servers.guard()[index.current_index()];
                println!("Server selected: {}", server.url);
            }
        }
    }
}

fn convert_add_server_output(output: AddServerOutput) -> ServerListInput {
    match output {
        AddServerOutput::ServerAdded(server) => ServerListInput::ServerAdded(server),
    }
}
