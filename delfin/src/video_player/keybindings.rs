use glib::Propagation;
use gtk::{gdk, glib, EventControllerKey};
use relm4::gtk;

use super::controls::{
    fullscreen::{FullscreenInput, FULLSCREEN_BROKER},
    play_pause::{PlayPauseInput, PLAY_PAUSE_BROKER},
    skip_forwards_backwards::{
        SkipForwardsBackwardsInput, SKIP_BACKWARDS_BROKER, SKIP_FORWARDS_BROKER,
    },
    subtitles::{SubtitlesInput, SUBTITLES_BROKER},
    volume::{VolumeInput, VOLUME_BROKER},
};

pub fn keybindings_controller() -> EventControllerKey {
    let controller = EventControllerKey::new();

    controller.connect_key_pressed(move |_, key, _, _| {
        match key {
            gdk::Key::space => {
                PLAY_PAUSE_BROKER.read().send(PlayPauseInput::TogglePlaying);
            }
            gdk::Key::Left => {
                SKIP_BACKWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::Skip);
            }
            gdk::Key::Right => {
                SKIP_FORWARDS_BROKER
                    .read()
                    .send(SkipForwardsBackwardsInput::Skip);
            }
            gdk::Key::Up => {
                VOLUME_BROKER.read().send(VolumeInput::ChangeVolume(0.1));
            }
            gdk::Key::Down => {
                VOLUME_BROKER.read().send(VolumeInput::ChangeVolume(-0.1));
            }
            gdk::Key::m => {
                VOLUME_BROKER.read().send(VolumeInput::ToggleMute);
            }
            gdk::Key::f => {
                FULLSCREEN_BROKER
                    .read()
                    .send(FullscreenInput::ToggleFullscreen);
            }
            gdk::Key::c => {
                SUBTITLES_BROKER
                    .read()
                    .send(SubtitlesInput::ToggleSubtitles);
            }
            _ => return Propagation::Proceed,
        };
        Propagation::Stop
    });

    controller
}
