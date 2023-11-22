use std::sync::Arc;

use gtk::gio;
use gtk::prelude::*;
use relm4::{
    actions::{AccelsPlus, RelmAction, RelmActionGroup},
    prelude::*,
};

use crate::{
    borgar::sign_out_dialog::SignOutDialog,
    config::{Account, Server},
    jellyfin_api::api_client::ApiClient,
    preferences::Preferences,
    tr,
    utils::main_window::get_main_window,
};

use super::about::About;

pub struct BorgarMenuAuth {
    pub api_client: Arc<ApiClient>,
    pub server: Server,
    pub account: Account,
}

pub struct BorgarMenu {
    auth: Option<BorgarMenuAuth>,
    preferences: Option<Controller<Preferences>>,
    sign_out_dialog: Option<Controller<SignOutDialog>>,
    about: Option<Controller<About>>,
}

#[derive(Debug)]
pub enum BorgarMenuInput {
    SignOut,
    Preferences,
    About,
}

relm4::new_action_group!(BorgarMenuActionGroup, "menu");
relm4::new_stateless_action!(SignOutAction, BorgarMenuActionGroup, "sign_out");
relm4::new_stateless_action!(PreferencesAction, BorgarMenuActionGroup, "preferences");
relm4::new_stateless_action!(
    KeyboardShortcutsAction,
    BorgarMenuActionGroup,
    "keyboard-shortcuts"
);
relm4::new_stateless_action!(AboutAction, BorgarMenuActionGroup, "about");

#[relm4::component(pub)]
impl Component for BorgarMenu {
    type Init = Option<BorgarMenuAuth>;
    type Input = BorgarMenuInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            add_css_class: "flat",
            set_menu_model: Some(&menu),
            set_icon_name: "open-menu",
            set_tooltip_text: Some(tr!("borgar-menu-tooltip")),
        }
    }

    menu! {
        menu: {
            section! {
                &*tr!("borgar-preferences") => PreferencesAction,
                &*tr!("borgar-keyboard-shortcuts") => KeyboardShortcutsAction,
                &*tr!("borgar-about") => AboutAction,
            },
        }
    }

    fn init(
        auth: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = BorgarMenu {
            auth,
            preferences: None,
            sign_out_dialog: None,
            about: None,
        };

        let app = relm4::main_application();

        let widgets = view_output!();

        let mut group = RelmActionGroup::<BorgarMenuActionGroup>::new();

        let preferences_action: RelmAction<PreferencesAction> = RelmAction::new_stateless({
            let sender = sender.clone();
            move |_| {
                sender.input(BorgarMenuInput::Preferences);
            }
        });

        let keyboard_shortcuts_action: RelmAction<KeyboardShortcutsAction> =
            RelmAction::new_stateless(move |_| {
                get_main_window()
                    .and_then(|win| win.lookup_action("show-help-overlay"))
                    .expect("Error getting show-help-overlay action")
                    .activate(None);
            });

        app.set_accelerators_for_action::<KeyboardShortcutsAction>(&["<Ctrl>question"]);

        let about_action: RelmAction<AboutAction> = RelmAction::new_stateless({
            let sender = sender.clone();
            move |_| {
                sender.input(BorgarMenuInput::About);
            }
        });

        group.add_action(preferences_action);
        group.add_action(keyboard_shortcuts_action);
        group.add_action(about_action);

        if model.auth.is_some() {
            add_signed_in_items(&sender, &mut group, &menu);
        }

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
            BorgarMenuInput::SignOut => 'msg: {
                let BorgarMenuAuth {
                    api_client,
                    server,
                    account,
                } = match &self.auth {
                    Some(auth) => auth,
                    _ => break 'msg,
                };

                self.sign_out_dialog = Some(
                    SignOutDialog::builder()
                        .transient_for(root)
                        .launch((api_client.clone(), server.clone(), account.clone()))
                        .detach(),
                );
            }
            BorgarMenuInput::Preferences => {
                self.preferences = Some(
                    Preferences::builder()
                        .transient_for(root)
                        .launch(())
                        .detach(),
                );
            }
            BorgarMenuInput::About => {
                self.about = Some(About::builder().transient_for(root).launch(()).detach())
            }
        }
        self.update_view(widgets, sender);
    }
}

fn add_signed_in_items(
    sender: &ComponentSender<BorgarMenu>,
    group: &mut RelmActionGroup<BorgarMenuActionGroup>,
    menu: &gio::Menu,
) {
    let sign_out_action: RelmAction<SignOutAction> = RelmAction::new_stateless({
        let sender = sender.clone();
        move |_| {
            sender.input(BorgarMenuInput::SignOut);
        }
    });
    let section = gio::Menu::new();
    menu.prepend_section(None, &section);
    let sign_out_entry = RelmAction::<SignOutAction>::to_menu_item(tr!("borgar-sign-out"));
    section.append_item(&sign_out_entry);

    group.add_action(sign_out_action);
}
