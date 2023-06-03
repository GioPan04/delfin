use adw::prelude::*;
use relm4::{
    adw,
    factory::FactoryVecDeque,
    gtk,
    prelude::{DynamicIndex, FactoryComponent},
    Component, ComponentParts, Controller,
};

use super::add_server::{AddServerDialog, AddServerOutput};

pub struct ServerList {
    servers: FactoryVecDeque<ServerListItem>,
    add_server_dialog: Option<Controller<AddServerDialog>>,
}

#[derive(Debug)]
pub enum ServerListInput {
    AddServer,
    ServerAdded(String, String),
    ServerSelected(DynamicIndex),
}

#[derive(Debug)]
pub enum ServerListOutput {
    ServerSelected(String),
}

#[relm4::component(pub)]
impl Component for ServerList {
    type Init = ();
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
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let mut model = ServerList {
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
            ServerListInput::AddServer => {
                self.add_server_dialog = Some(
                    AddServerDialog::builder()
                        .transient_for(&root)
                        .launch(())
                        .forward(sender.input_sender(), convert_add_server_output),
                );
            }
            ServerListInput::ServerAdded(url, server_name) => {
                self.servers.guard().push_back((url, server_name));
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
        AddServerOutput::ServerAdded(url, server_name) => {
            ServerListInput::ServerAdded(url, server_name)
        }
    }
}

struct ServerListItem {
    url: String,
    name: String,
}

#[derive(Debug)]
enum ServerListItemOutput {
    ServerSelected(DynamicIndex),
}

#[relm4::factory]
impl FactoryComponent for ServerListItem {
    type Init = (String, String);
    type Input = ();
    type Output = ServerListItemOutput;
    type CommandOutput = ();
    type ParentInput = ServerListInput;
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
                sender.output(ServerListItemOutput::ServerSelected(index.clone()));
            },
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(match output {
            ServerListItemOutput::ServerSelected(index) => ServerListInput::ServerSelected(index),
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
