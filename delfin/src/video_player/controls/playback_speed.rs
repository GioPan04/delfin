use std::{cell::RefCell, sync::Arc};

use gtk::{gio, prelude::*};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    gtk,
    prelude::ComponentParts,
    SimpleComponent,
};

use crate::{
    app::{AppInput, APP_BROKER},
    tr,
    utils::message_broker::ResettableMessageBroker,
    video_player::backends::VideoPlayerBackend,
};

const PLAYBACK_SPEED_OPTIONS: [f64; 9] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 1.75, 2.0, 4.0];

pub static PLAYBACK_SPEED_BROKER: ResettableMessageBroker<PlaybackSpeedInput> =
    ResettableMessageBroker::new();

#[derive(Debug)]
pub(super) struct PlaybackSpeed {
    menu: gio::Menu,
    playback_speed_action: gio::SimpleAction,
}

#[derive(Debug)]
pub enum PlaybackSpeedInput {
    SpeedUp,
    SlowDown,
}

#[relm4::component(pub(super))]
impl SimpleComponent for PlaybackSpeed {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = PlaybackSpeedInput;
    type Output = ();

    view! {
        gtk::MenuButton {
            set_menu_model: Some(&model.menu),
            set_icon_name: "playback-speed",
            set_tooltip_text: Some(tr!("vp-playback-speed-tooltip")),
            set_direction: gtk::ArrowType::Up,
        }
    }

    fn init(
        video_player: Self::Init,
        root: Self::Root,
        _sender: relm4::prelude::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let playback_speed_action: RelmAction<PlaybackSpeedAction> =
            RelmAction::new_stateful_with_target_value(&1.0, {
                let video_player = video_player.clone();
                move |_, state, value: f64| {
                    *state = value;
                    video_player.borrow().set_playback_speed(value);
                }
            });

        let model = Self {
            menu: gio::Menu::new(),
            playback_speed_action: gio::SimpleAction::from(playback_speed_action.clone()),
        };

        let playback_speed_menu = gio::Menu::new();
        PLAYBACK_SPEED_OPTIONS
            .iter()
            .map(|speed| {
                RelmAction::<PlaybackSpeedAction>::to_menu_item_with_target_value(
                    &if (*speed - 1.0).abs() < f64::EPSILON {
                        tr!("vp-playback-speed-normal").into()
                    } else {
                        format!("{speed}x")
                    },
                    speed,
                )
            })
            .for_each(|menu_item| playback_speed_menu.append_item(&menu_item));
        model
            .menu
            .append_section(Some(tr!("vp-playback-speed-tooltip")), &playback_speed_menu);

        let widgets = view_output!();

        let mut group = RelmActionGroup::<PlaybackSpeedActionGroup>::new();
        group.add_action(playback_speed_action);
        group.register_for_widget(&root);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::prelude::ComponentSender<Self>) {
        match message {
            PlaybackSpeedInput::SlowDown => {
                if let Some(index) = self.playback_speed_index() {
                    let new_speed_index = index.saturating_sub(1);
                    let new_speed = PLAYBACK_SPEED_OPTIONS[new_speed_index];
                    self.playback_speed_action
                        .activate(Some(&new_speed.to_variant()));
                    APP_BROKER.send(AppInput::Toast(
                        tr!("vp-playback-speed-toast", { "speed" => new_speed }).into(),
                        Some(1),
                    ));
                }
            }
            PlaybackSpeedInput::SpeedUp => {
                if let Some(index) = self.playback_speed_index() {
                    let new_speed_index = (index + 1).min(PLAYBACK_SPEED_OPTIONS.len() - 1);
                    let new_speed = PLAYBACK_SPEED_OPTIONS[new_speed_index];
                    self.playback_speed_action
                        .activate(Some(&new_speed.to_variant()));
                    APP_BROKER.send(AppInput::Toast(
                        tr!("vp-playback-speed-toast", { "speed" => new_speed }).into(),
                        Some(1),
                    ));
                }
            }
        }
    }
}

impl PlaybackSpeed {
    fn playback_speed_index(&self) -> Option<usize> {
        let speed = self.playback_speed_action.state()?.get::<f64>()?;
        PLAYBACK_SPEED_OPTIONS
            .iter()
            .position(|s| (speed - s).abs() < f64::EPSILON)
    }
}

relm4::new_action_group!(PlaybackSpeedActionGroup, "playback_speed_actions");
relm4::new_stateful_action!(
    PlaybackSpeedAction,
    PlaybackSpeedActionGroup,
    "playback_speed",
    f64,
    f64
);

impl Clone for PlaybackSpeedAction {
    fn clone(&self) -> Self {
        Self {}
    }
}
