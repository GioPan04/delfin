use adw::prelude::*;
use relm4::{
    adw,
    factory::FactoryVecDeque,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    Component, ComponentParts, Controller,
};

use crate::add_server::{AddServerDialog, AddServerOutput};

pub struct ServersModel {
    servers: FactoryVecDeque<ServerModel>,
    add_server_dialog: Option<Controller<AddServerDialog>>,
}

#[derive(Debug)]
pub enum ServersInput {
    AddServer,
    ServerAdded(String, String),
    ServerSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum ServersOutput {
    ServerSelected(String),
}

#[relm4::component(pub)]
impl Component for ServersModel {
    type Init = ();
    type Input = ServersInput;
    type Output = ServersOutput;
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
                        sender.input(ServersInput::AddServer);
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
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = ServersModel {
            servers: FactoryVecDeque::new(gtk::ListBox::default(), sender.input_sender()),
            add_server_dialog: None,
        };
        model
            .servers
            .guard()
            .push_back(("Example".into(), "jelly.example.com".into()));

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
            ServersInput::AddServer => {
                self.add_server_dialog = Some(
                    AddServerDialog::builder()
                        .transient_for(&root)
                        .launch(())
                        .forward(sender.input_sender(), convert_add_server_output),
                );
            }
            ServersInput::ServerAdded(url, server_name) => {
                self.servers.guard().push_back((url, server_name));
            }
            ServersInput::ServerSelected(index) => {
                let server = &self.servers.guard()[index.current_index()];
                println!("Server selected: {}", server.url);
            }
        }
    }
}

fn convert_add_server_output(output: AddServerOutput) -> ServersInput {
    match output {
        AddServerOutput::ServerAdded(url, server_name) => {
            ServersInput::ServerAdded(url, server_name)
        }
    }
}

struct ServerModel {
    url: String,
    name: String,
}

#[derive(Debug)]
enum ServerOutput {
    ServerSelected(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for ServerModel {
    type Init = (String, String);
    type Input = ();
    type Output = ServerOutput;
    type CommandOutput = ();
    type ParentInput = ServersInput;
    type ParentWidget = gtk::ListBox;

    view! {
        adw::ActionRow {
            #[watch]
            set_title: &self.name,
            #[watch]
            set_subtitle: &self.url,
            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },
            set_activatable: true,

            connect_activated[sender, index] => move |_| {
                sender.output(ServerOutput::ServerSelected(index.clone()));
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            ServerOutput::ServerSelected(index) => ServersInput::ServerSelected(index),
        })
    }

    fn init_model(
        init: Self::Init,
        _index: &Self::Index,
        _sender: relm4::FactorySender<Self>,
    ) -> Self {
        Self {
            url: init.0,
            name: init.1,
        }
    }
}
