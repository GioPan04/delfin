use adw::prelude::*;
use relm4::{
    adw, gtk,
    prelude::{DynamicIndex, FactoryComponent},
};

use crate::config;

use super::server_list::ServerListInput;

pub struct ServerListItem {
    pub id: String,
    pub url: String,
    pub name: String,
}

#[derive(Debug)]
pub enum ServerListItemOutput {
    ServerSelected(DynamicIndex),
}

#[relm4::factory(pub)]
impl FactoryComponent for ServerListItem {
    type Init = config::Server;
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
            id: init.id,
            url: init.url,
            name: init.name,
        }
    }
}
