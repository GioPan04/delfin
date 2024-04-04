use adw::prelude::*;
use relm4::{factory::FactoryVecDeque, prelude::*};

use crate::{
    borgar::borgar_menu::BorgarMenu, config, globals::CONFIG, tr, utils::constants::PAGE_MARGIN,
};

use super::{
    add_server::{AddServerDialog, AddServerOutput},
    server_list_item::{ServerListItem, ServerListItemOutput},
};

pub struct ServerList {
    servers: FactoryVecDeque<ServerListItem>,
    add_server_dialog: Option<Controller<AddServerDialog>>,
    borgar: Controller<BorgarMenu>,
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
            set_title: tr!("server-list-page-title"),

            connect_showing[sender] => move |_| {
                sender.input(ServerListInput::ReloadServers);
            },

            #[wrap(Some)]
            set_child = &adw::ToolbarView {
                add_top_bar = &adw::HeaderBar {
                    pack_end = model.borgar.widget(),
                },

                #[wrap(Some)]
                set_content = &adw::Clamp {
                    set_margin_top: PAGE_MARGIN,
                    set_margin_end: 12,
                    set_margin_start: 12,

                    adw::PreferencesGroup {
                        set_title: tr!("server-list.title"),
                        set_description: Some(tr!("server-list.description")),
                        #[wrap(Some)]

                        set_header_suffix = &gtk::Button::from_icon_name("list-add-symbolic") {
                            set_margin_start: 10,
                            set_valign: gtk::Align::Start,
                            set_tooltip: tr!("server-list-add-server-button"),
                            connect_clicked[sender] => move |_| {
                                sender.input(ServerListInput::AddServer);
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
                                set_title: tr!("server-list-empty.title"),
                                set_subtitle: tr!("server-list-empty.subtitle"),
                            },
                        },
                    },
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let servers = FactoryVecDeque::builder()
            .launch(gtk::ListBox::default())
            .forward(sender.input_sender(), convert_server_list_item_output);

        let model = ServerList {
            servers,
            add_server_dialog: None,
            borgar: BorgarMenu::builder().launch(None).detach(),
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

fn convert_server_list_item_output(output: ServerListItemOutput) -> ServerListInput {
    match output {
        ServerListItemOutput::ServerSelected(index) => ServerListInput::ServerSelected(index),
    }
}
