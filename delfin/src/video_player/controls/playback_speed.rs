use std::{cell::RefCell, sync::Arc};

use gtk::{gio, prelude::*};
use relm4::{
    actions::{RelmAction, RelmActionGroup},
    gtk,
    prelude::ComponentParts,
    SimpleComponent,
};

use crate::{tr, video_player::backends::VideoPlayerBackend};

const PLAYBACK_SPEED_OPTIONS: [f64; 8] = [0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 4.0];

#[derive(Debug)]
pub(super) struct PlaybackSpeed {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    menu: gio::Menu,
}

#[relm4::component(pub(super))]
impl SimpleComponent for PlaybackSpeed {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = ();
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
        let model = Self {
            video_player,
            menu: gio::Menu::new(),
        };

        let playback_speed_menu = gio::Menu::new();
        PLAYBACK_SPEED_OPTIONS
            .iter()
            .map(|speed| {
                RelmAction::<PlaybackSpeedAction>::to_menu_item_with_target_value(
                    &if *speed == 1.0 {
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

        let playback_speed_action: RelmAction<PlaybackSpeedAction> =
            RelmAction::new_stateful_with_target_value(&1.0, {
                let video_player = model.video_player.clone();
                move |_, state, value: f64| {
                    *state = value;
                    video_player.borrow().set_playback_speed(value);
                }
            });
        let mut group = RelmActionGroup::<PlaybackSpeedActionGroup>::new();
        group.add_action(playback_speed_action);
        group.register_for_widget(&root);

        let widgets = view_output!();

        ComponentParts { model, widgets }
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
