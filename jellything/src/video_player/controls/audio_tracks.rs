use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use gstplay::{traits::PlayStreamInfoExt, PlayAudioInfo};
use gtk::prelude::*;
use relm4::{
    actions::{ActionGroupName, ActionName, RelmAction, RelmActionGroup},
    gtk::{self, gio},
    Component, ComponentParts,
};

use crate::video_player::gst_play_widget::GstVideoPlayer;

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
    video_player: Rc<GstVideoPlayer>,
    menu: gio::Menu,
    audio_track_count: Arc<RwLock<Option<usize>>>,
    audio_tracks_available: bool,
}

#[derive(Debug)]
pub enum AudioTracksInput {
    Reset,
    AudioTracksUpdated(Vec<PlayAudioInfo>),
}

#[relm4::component(pub)]
impl Component for AudioTracks {
    type Init = Rc<GstVideoPlayer>;
    type Input = AudioTracksInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            set_icon_name: "music-note-single",
            set_menu_model: Some(&model.menu),
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
            audio_track_count: Arc::new(RwLock::new(None)),
            audio_tracks_available: false,
        };

        model.video_player.connect_media_info_updated({
            let audio_track_count = model.audio_track_count.clone();
            move |media_info| {
                let audio_tracks = media_info.audio_streams();
                let audio_streams_count = audio_tracks.len();
                // subtitle_count keeps track of the current subtitle track count for the
                // currently playing media. If a different number is reported we update the
                // subtitles menu.
                match *audio_track_count.read().unwrap() {
                    Some(audio_track_count) if audio_track_count == audio_streams_count => {}
                    _ => {
                        sender.input(AudioTracksInput::AudioTracksUpdated(audio_tracks));
                    }
                };
            }
        });

        let widgets = view_output!();

        let selected_audio_track_action: RelmAction<SelectedAudioTrackAction> =
            RelmAction::new_stateful_with_target_value(&None, {
                let video_player = model.video_player.clone();
                move |_, state, value: Option<i32>| {
                    *state = value;

                    video_player.set_audio_track_enabled(value.is_some());
                    if let Some(value) = value {
                        video_player
                            .set_audio_track(value)
                            .expect("Error setting audio track");
                    }
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
                let mut audio_track_count = self.audio_track_count.write().unwrap();
                *audio_track_count = None;

                self.audio_tracks_available = false;
            }
            AudioTracksInput::AudioTracksUpdated(audio_tracks) => {
                let mut audio_track_count = self.audio_track_count.write().unwrap();
                *audio_track_count = Some(audio_tracks.len());

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
                            &audio_track.display_name(),
                            &Some(audio_track.index()),
                        )
                    })
                    .for_each(|menu_item| audio_tracks_menu.append_item(&menu_item));

                self.menu.remove_all();
                self.menu
                    .append_section(Some("Audio Track"), &audio_tracks_menu);

                // Select current audio track in menu
                if let Some(current_audio_track) = self.video_player.current_audio_track() {
                    root.activate_action(
                        &format!(
                            "{}.{}",
                            AudioTracksActionGroup::NAME,
                            SelectedAudioTrackAction::NAME
                        ),
                        Some(&Some(current_audio_track.index()).to_variant()),
                    )
                    .unwrap();
                }
            }
        }

        self.update_view(widgets, sender);
    }
}

trait PlayAudioInfoExt {
    fn display_name(&self) -> String;
}

impl PlayAudioInfoExt for PlayAudioInfo {
    fn display_name(&self) -> String {
        let mut display_name = self
            .language()
            .map(|l| l.to_string())
            .unwrap_or(self.index().to_string());

        let tags = self.tags();
        if let Some(tags) = tags {
            if let Some(title) = tags.get::<gst::tags::Title>() {
                let title = title.get();
                display_name = format!("{display_name} - {title}");
            }
        }

        display_name
    }
}
