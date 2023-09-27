use std::sync::{Arc, RwLock};

use gtk::prelude::*;
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    prelude::*,
};

use crate::{
    borgar::sign_out_dialog::SignOutDialog,
    config::{Account, Config, Server},
    jellyfin_api::api_client::ApiClient,
    preferences::Preferences,
};

pub struct BorgarMenu {
    api_client: Arc<ApiClient>,
    config: Arc<RwLock<Config>>,
    server: Server,
    account: Account,
    preferences: Option<Controller<Preferences>>,
    sign_out_dialog: Option<Controller<SignOutDialog>>,
}

#[derive(Debug)]
pub enum BorgarMenuInput {
    Preferences,
    SignOut,
}

relm4::new_action_group!(BorgarMenuActionGroup, "menu");
relm4::new_stateless_action!(PreferencesAction, BorgarMenuActionGroup, "preferences");
relm4::new_stateless_action!(SignOutAction, BorgarMenuActionGroup, "sign_out");

#[relm4::component(pub)]
impl Component for BorgarMenu {
    type Init = (Arc<ApiClient>, Arc<RwLock<Config>>, Server, Account);
    type Input = BorgarMenuInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            add_css_class: "flat",
            set_menu_model: Some(&menu),
            set_icon_name: "open-menu",
        }
    }

    menu! {
        menu: {
            "Preferences" => PreferencesAction,
            "Sign Out" => SignOutAction,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (api_client, config, server, account) = init;

        let model = BorgarMenu {
            api_client,
            config,
            server,
            account,
            preferences: None,
            sign_out_dialog: None,
        };
        let widgets = view_output!();

        let preferences_action: RelmAction<PreferencesAction> = RelmAction::new_stateless({
            let sender = sender.clone();
            move |_| {
                sender.input(BorgarMenuInput::Preferences);
            }
        });

        let sign_out_action: RelmAction<SignOutAction> = RelmAction::new_stateless({
            let sender = sender.clone();
            move |_| {
                sender.input(BorgarMenuInput::SignOut);
            }
        });

        let mut group = RelmActionGroup::<BorgarMenuActionGroup>::new();
        group.add_action(preferences_action);
        group.add_action(sign_out_action);
        group.register_for_widget(root);

        ComponentParts { model, widgets }
    }
    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            BorgarMenuInput::Preferences => {
                self.preferences = Some(
                    Preferences::builder()
                        .transient_for(root)
                        .launch(())
                        .detach(),
                );
            }
            BorgarMenuInput::SignOut => {
                self.sign_out_dialog = Some(
                    SignOutDialog::builder()
                        .transient_for(root)
                        .launch((
                            self.api_client.clone(),
                            self.config.clone(),
                            self.server.clone(),
                            self.account.clone(),
                        ))
                        .detach(),
                );
            }
        }
        self.update_view(widgets, sender);
    }
}
