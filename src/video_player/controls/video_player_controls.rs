use std::cell::OnceCell;

use crate::{jellyfin_api::models::media::Media, video_player::gst_play_widget::GstVideoPlayer};
use gtk::prelude::*;
use relm4::{gtk, Component, ComponentController, ComponentParts, Controller, SimpleComponent};

use super::{
    fullscreen::Fullscreen,
    play_pause::PlayPause,
    scrubber::Scrubber,
    subtitles::{Subtitles, SubtitlesInput},
    volume::Volume,
};

pub struct VideoPlayerControls {
    show_controls: bool,
    // We need to keep these controllers around, even if we don't read them
    _scrubber: Option<Controller<Scrubber>>,
    _play_pause: Option<Controller<PlayPause>>,
    _volume: Option<Controller<Volume>>,
    subtitles: OnceCell<Controller<Subtitles>>,
    _fullscreen: Option<Controller<Fullscreen>>,
}

pub struct VideoPlayerControlsInit {
    pub player: OnceCell<GstVideoPlayer>,
    pub default_show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerControlsInput {
    SetShowControls(bool),
    SetPlaying(Media),
}

#[relm4::component(pub)]
impl SimpleComponent for VideoPlayerControls {
    type Init = VideoPlayerControlsInit;
    type Input = VideoPlayerControlsInput;
    type Output = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            #[watch]
            set_visible: model.show_controls,
            set_valign: gtk::Align::End,
            add_css_class: "toolbar",
            add_css_class: "osd",
            add_css_class: "video-player-controls",

            #[name = "second_row"]
            gtk::Box {
                #[name = "play_pause_placeholder"]
                gtk::Box {},

                #[name = "volume_placeholder"]
                gtk::Box {},

                #[name = "subtitles_placeholder"]
                gtk::Box {},

                #[name = "fullscreen_placeholder"]
                gtk::Box {},
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let VideoPlayerControlsInit {
            player,
            default_show_controls,
        } = init;

        let mut model = VideoPlayerControls {
            show_controls: default_show_controls,
            _scrubber: None,
            _play_pause: None,
            _volume: None,
            subtitles: OnceCell::new(),
            _fullscreen: None,
        };

        let widgets = view_output!();
        let second_row = &widgets.second_row;
        let play_pause_placeholder = &widgets.play_pause_placeholder;
        let volume_placeholder = &widgets.volume_placeholder;
        let subtitles_placeholder = &widgets.subtitles_placeholder;
        let fullscreen_placeholder = &widgets.fullscreen_placeholder;

        let scrubber = Scrubber::builder().launch(player.clone()).detach();
        root.prepend(scrubber.widget());
        model._scrubber = Some(scrubber);

        let play_pause = PlayPause::builder().launch(player.clone()).detach();
        second_row.insert_child_after(play_pause.widget(), Some(play_pause_placeholder));
        model._play_pause = Some(play_pause);

        let volume = Volume::builder().launch(player.clone()).detach();
        second_row.insert_child_after(volume.widget(), Some(volume_placeholder));
        model._volume = Some(volume);

        let subtitles = Subtitles::builder().launch(player).detach();
        second_row.insert_child_after(subtitles.widget(), Some(subtitles_placeholder));
        model.subtitles.set(subtitles).unwrap();

        let fullscreen = Fullscreen::builder().launch(()).detach();
        second_row.insert_child_after(fullscreen.widget(), Some(fullscreen_placeholder));
        model._fullscreen = Some(fullscreen);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            VideoPlayerControlsInput::SetShowControls(show_controls) => {
                self.show_controls = show_controls
            }
            VideoPlayerControlsInput::SetPlaying(_) => {
                if let Some(subtitles) = self.subtitles.get() {
                    subtitles.emit(SubtitlesInput::Reset);
                }
            }
        }
    }
}
