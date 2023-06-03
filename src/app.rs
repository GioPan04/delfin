use core::fmt;

use adw::prelude::*;
use relm4::prelude::*;

use crate::servers::server_list::ServerList;

#[derive(Debug)]
pub enum AppPage {
    Servers,
    Accounts,
}

impl fmt::Display for AppPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppPage::Servers => write!(f, "servers"),
            AppPage::Accounts => write!(f, "accounts"),
        }
    }
}

pub struct App {
    page: AppPage,
    servers: Controller<ServerList>,
}

#[derive(Debug)]
pub enum AppInput {
    SetPage(AppPage),
}

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppInput;
    type Output = ();

    view! {
        adw::ApplicationWindow {
            set_title: Some("Jellything"),
            set_default_width: 960,
            set_default_height: 540,

            #[wrap(Some)]
            set_content = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,

                adw::HeaderBar {
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        set_title: "Jellything",
                    }
                },

                gtk::Stack {
                    set_transition_type: gtk::StackTransitionType::SlideLeft,

                    add_child = model.servers.widget() {} -> {
                        set_name: &AppPage::Servers.to_string(),
                    },

                    add_child = &gtk::Box {
                        gtk::Label {
                            set_label: "Accounts",
                        },
                        gtk::Button {
                            set_label: "thingy",
                            connect_clicked[sender] => move |_| {
                                sender.input(AppInput::SetPage(AppPage::Servers));
                            }
                        }
                    } -> {
                        set_name: &AppPage::Accounts.to_string(),
                    },

                    #[watch]
                    set_visible_child_name: &model.page.to_string(),
                },
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = App {
            page: AppPage::Servers,
            servers: ServerList::builder().launch(()).detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            AppInput::SetPage(page) => self.page = page,
        }
    }
}
