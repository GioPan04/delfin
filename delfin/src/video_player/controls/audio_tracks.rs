use std::{cell::RefCell, sync::Arc};

use gtk::prelude::*;
use relm4::{
    actions::{ActionGroupName, ActionName, RelmAction, RelmActionGroup},
    gtk::{self, gio},
    Component, ComponentParts,
};

use crate::{
    tr,
    video_player::backends::{AudioTrack, VideoPlayerBackend},
};

relm4::new_action_group!(AudioTracksActionGroup, "audio_tracks_actions");
relm4::new_stateful_action!(
    SelectedAudioTrackAction,
    AudioTracksActionGroup,
    "selected_audio_track",
    Option<i32>,
    Option<i32>
);

#[derive(Debug)]
pub struct AudioTracks {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    menu: gio::Menu,
    audio_tracks_available: bool,
}

#[derive(Debug)]
pub enum AudioTracksInput {
    Reset,
    AudioTracksUpdated(Vec<AudioTrack>),
}

#[relm4::component(pub)]
impl Component for AudioTracks {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = AudioTracksInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            set_icon_name: "music-note-single",
            set_menu_model: Some(&model.menu),
            set_tooltip_text: Some(tr!("vp-audio-track-tooltip")),
            set_focus_on_click: false,
            #[watch]
            set_visible: model.audio_tracks_available,
        }
    }

    fn init(
        video_player: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AudioTracks {
            video_player,
            menu: gio::Menu::new(),
            audio_tracks_available: false,
        };

        model
            .video_player
            .borrow_mut()
            .connect_audio_tracks_updated(Box::new(move |tracks| {
                sender.input(AudioTracksInput::AudioTracksUpdated(tracks.clone()));
            }));

        let widgets = view_output!();

        let selected_audio_track_action: RelmAction<SelectedAudioTrackAction> =
            RelmAction::new_stateful_with_target_value(&None, {
                let video_player = model.video_player.clone();
                move |_, state, value: Option<i32>| {
                    *state = value;
                    video_player
                        .borrow()
                        .set_audio_track(value.map(|id| id as usize));
                }
            });

        let mut group = RelmActionGroup::<AudioTracksActionGroup>::new();
        group.add_action(selected_audio_track_action);
        group.register_for_widget(root);

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            AudioTracksInput::Reset => {
                self.audio_tracks_available = false;
            }
            AudioTracksInput::AudioTracksUpdated(audio_tracks) => {
                if audio_tracks.len() < 2 {
                    return;
                }

                self.audio_tracks_available = true;

                // Build updated audio tracks menu
                let audio_tracks_menu = gio::Menu::new();
                audio_tracks
                    .iter()
                    .map(|audio_track| {
                        RelmAction::<SelectedAudioTrackAction>::to_menu_item_with_target_value(
                            &audio_track.name,
                            &Some(audio_track.id as i32),
                        )
                    })
                    .for_each(|menu_item| audio_tracks_menu.append_item(&menu_item));

                self.menu.remove_all();
                self.menu
                    .append_section(Some(tr!("vp-audio-track-menu")), &audio_tracks_menu);

                // Select current audio track in menu
                if let Some(current_audio_track) = self.video_player.borrow().current_audio_track()
                {
                    root.activate_action(
                        &format!(
                            "{}.{}",
                            AudioTracksActionGroup::NAME,
                            SelectedAudioTrackAction::NAME
                        ),
                        Some(&Some(current_audio_track as i32).to_variant()),
                    )
                    .expect("Error selecting current audio track.");
                }
            }
        }

        self.update_view(widgets, sender);
    }
}
