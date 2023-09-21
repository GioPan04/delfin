use std::cell::OnceCell;

use crate::{
    app::APP_BROKER,
    jellyfin_api::models::media::Media,
    video_player::{
        controls::{
            next_prev_episode::NextPrevEpisodeDirection,
            play_pause::PLAY_PAUSE_BROKER,
            scrubber::SCRUBBER_BROKER,
            skip_forwards_backwards::{
                SkipForwardsBackwardsDirection, SKIP_BACKWARDS_BROKER, SKIP_FORWARDS_BROKER,
            },
        },
        gst_play_widget::GstVideoPlayer,
    },
};
use gtk::prelude::*;
use relm4::{gtk, Component, ComponentController, ComponentParts, Controller, SimpleComponent};

use super::{
    audio_tracks::{AudioTracks, AudioTracksInput},
    fullscreen::Fullscreen,
    next_prev_episode::{NextPrevEpisode, NextPrevEpisodeInput, NextPrevEpisodeOutput},
    play_pause::PlayPause,
    scrubber::Scrubber,
    skip_forwards_backwards::SkipForwardsBackwards,
    subtitles::{Subtitles, SubtitlesInput},
    volume::Volume,
};

pub struct VideoPlayerControls {
    show_controls: bool,
    next_prev_episodes: (Option<Media>, Option<Media>),
    // We need to keep these controllers around, even if we don't read them
    _scrubber: Option<Controller<Scrubber>>,
    _play_pause: Option<Controller<PlayPause>>,
    _skip_forwards_backwards: OnceCell<(
        Controller<SkipForwardsBackwards>,
        Controller<SkipForwardsBackwards>,
    )>,
    next_prev_episode_controls:
        OnceCell<(Controller<NextPrevEpisode>, Controller<NextPrevEpisode>)>,
    _volume: Option<Controller<Volume>>,
    subtitles: OnceCell<Controller<Subtitles>>,
    audio_tracks: OnceCell<Controller<AudioTracks>>,
    _fullscreen: Option<Controller<Fullscreen>>,
}

pub struct VideoPlayerControlsInit {
    pub player: OnceCell<GstVideoPlayer>,
    pub default_show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerControlsInput {
    SetShowControls(bool),
    SetPlaying(Box<Media>),
    SetNextPreviousEpisodes(Box<Option<Media>>, Box<Option<Media>>),
    PlayPreviousEpisode,
    PlayNextEpisode,
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
            set_margin_start: 24,
            set_margin_end: 24,
            set_margin_bottom: 24,

            #[name = "second_row"]
            gtk::Box {},
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let VideoPlayerControlsInit {
            player,
            default_show_controls,
        } = init;

        let mut model = VideoPlayerControls {
            show_controls: default_show_controls,
            _skip_forwards_backwards: OnceCell::new(),
            next_prev_episodes: (None, None),
            _scrubber: None,
            _play_pause: None,
            next_prev_episode_controls: OnceCell::new(),
            _volume: None,
            subtitles: OnceCell::new(),
            audio_tracks: OnceCell::new(),
            _fullscreen: None,
        };

        let widgets = view_output!();
        let second_row = &widgets.second_row;

        let scrubber = Scrubber::builder()
            .launch_with_broker(player.clone(), &SCRUBBER_BROKER)
            .detach();
        root.prepend(scrubber.widget());
        model._scrubber = Some(scrubber);

        let prev_episode = NextPrevEpisode::builder()
            .launch(NextPrevEpisodeDirection::Previous)
            .forward(sender.input_sender(), |msg| match msg {
                NextPrevEpisodeOutput::Clicked => VideoPlayerControlsInput::PlayPreviousEpisode,
            });
        second_row.append(prev_episode.widget());

        let skip_backwards = SkipForwardsBackwards::builder()
            .launch_with_broker(
                (SkipForwardsBackwardsDirection::Backwards, player.clone()),
                &SKIP_BACKWARDS_BROKER,
            )
            .detach();
        second_row.append(skip_backwards.widget());

        let play_pause = PlayPause::builder()
            .launch_with_broker(player.clone(), &PLAY_PAUSE_BROKER)
            .detach();
        second_row.append(play_pause.widget());
        model._play_pause = Some(play_pause);

        let skip_forwards = SkipForwardsBackwards::builder()
            .launch_with_broker(
                (SkipForwardsBackwardsDirection::Forwards, player.clone()),
                &SKIP_FORWARDS_BROKER,
            )
            .detach();
        second_row.append(skip_forwards.widget());
        model
            ._skip_forwards_backwards
            .set((skip_backwards, skip_forwards))
            .unwrap();

        let next_episode = NextPrevEpisode::builder()
            .launch(NextPrevEpisodeDirection::Next)
            .forward(sender.input_sender(), |msg| match msg {
                NextPrevEpisodeOutput::Clicked => VideoPlayerControlsInput::PlayNextEpisode,
            });
        second_row.append(next_episode.widget());
        model
            .next_prev_episode_controls
            .set((prev_episode, next_episode))
            .unwrap();

        // Push remaining controls to end
        second_row.append(&gtk::Box::builder().hexpand(true).build());

        let subtitles = Subtitles::builder().launch(player.clone()).detach();
        second_row.append(subtitles.widget());
        model.subtitles.set(subtitles).unwrap();

        let audio_tracks = AudioTracks::builder().launch(player.clone()).detach();
        second_row.append(audio_tracks.widget());
        model.audio_tracks.set(audio_tracks).unwrap();

        let volume = Volume::builder().launch(player).detach();
        second_row.append(volume.widget());
        model._volume = Some(volume);

        let fullscreen = Fullscreen::builder().launch(()).detach();
        second_row.append(fullscreen.widget());
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
                if let Some(audio_tracks) = self.audio_tracks.get() {
                    audio_tracks.emit(AudioTracksInput::Reset);
                }
            }
            VideoPlayerControlsInput::SetNextPreviousEpisodes(prev, next) => {
                if let Some((prev_control, next_control)) = self.next_prev_episode_controls.get() {
                    prev_control.emit(if prev.is_some() {
                        NextPrevEpisodeInput::Show
                    } else {
                        NextPrevEpisodeInput::Hide
                    });
                    next_control.emit(if next.is_some() {
                        NextPrevEpisodeInput::Show
                    } else {
                        NextPrevEpisodeInput::Hide
                    });
                }

                self.next_prev_episodes = (*prev, *next);
            }
            VideoPlayerControlsInput::PlayPreviousEpisode => {
                if let (Some(previous), _) = &self.next_prev_episodes {
                    APP_BROKER.send(crate::app::AppInput::PlayVideo(previous.clone()));
                }
            }
            VideoPlayerControlsInput::PlayNextEpisode => {
                if let (_, Some(next)) = &self.next_prev_episodes {
                    APP_BROKER.send(crate::app::AppInput::PlayVideo(next.clone()));
                }
            }
        }
    }
}
