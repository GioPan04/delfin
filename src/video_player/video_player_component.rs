use gst::glib::WeakRef;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::video_player::fullscreen::Fullscreen;
use crate::video_player::player::create_player;
use crate::video_player::scrubber::Scrubber;
use crate::video_player::volume::Volume;

use super::play_pause::PlayPause;
use super::scrubber::ScrubberOutput;

struct VideoPlayerBuilder {
    media: LatestMedia,
    scrubber: Option<Controller<Scrubber>>,
    play_pause: Option<Controller<PlayPause>>,
    volume: Option<Controller<Volume>>,
    fullscreen: Option<Controller<Fullscreen>>,
    player: Option<WeakRef<gstplay::Play>>,
    show_controls: bool,
}

impl VideoPlayerBuilder {
    fn new(media: LatestMedia) -> Self {
        Self {
            media,
            scrubber: None,
            play_pause: None,
            volume: None,
            fullscreen: None,
            player: None,
            show_controls: false,
        }
    }

    fn set_player(mut self, player: &gstplay::Play) -> Self {
        self.player = Some(player.downgrade());
        self
    }

    fn set_play_pause(mut self, play_pause: Controller<PlayPause>) -> Self {
        self.play_pause = Some(play_pause);
        self
    }

    fn set_scrubber(mut self, scrubber: Controller<Scrubber>) -> Self {
        self.scrubber = Some(scrubber);
        self
    }

    fn set_volume(mut self, volume: Controller<Volume>) -> Self {
        self.volume = Some(volume);
        self
    }

    fn set_fullscreen(mut self, fullscreen: Controller<Fullscreen>) -> Self {
        self.fullscreen = Some(fullscreen);
        self
    }

    fn build(self) -> VideoPlayer {
        let player = self
            .player
            .expect("Tried to build VideoPlayer without player.");

        if self.scrubber.is_none() {
            panic!("Tried to build VideoPlayer without scrubber.");
        }

        if self.play_pause.is_none() {
            panic!("Tried to build VideoPlayer without play_pause.");
        }

        if self.volume.is_none() {
            panic!("Tried to build VideoPlayer without volume.");
        }

        if self.fullscreen.is_none() {
            panic!("Tried to build VideoPlayer without fullscreen.");
        }

        VideoPlayer {
            media: self.media.clone(),
            _scrubber: self.scrubber,
            _play_pause: self.play_pause,
            _volume: self.volume,
            _fullscreen: self.fullscreen,
            player,
            show_controls: self.show_controls,
        }
    }
}

pub struct VideoPlayer {
    media: LatestMedia,
    // We need to keep these controllers around, even if we don't read them
    _scrubber: Option<Controller<Scrubber>>,
    _play_pause: Option<Controller<PlayPause>>,
    _volume: Option<Controller<Volume>>,
    _fullscreen: Option<Controller<Fullscreen>>,
    player: WeakRef<gstplay::Play>,
    show_controls: bool,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ToggleControls,
    Seek(f64),
    ExitPlayer,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = (Server, LatestMedia);
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            add_css_class: "video-player",

            gtk::Overlay {
                #[name = "video_out"]
                gtk::Picture {
                    set_vexpand: true,
                    add_css_class: "video-out",
                    add_controller = gtk::GestureClick {
                        connect_pressed[sender] => move |_, _, _, _| {
                            sender.input(VideoPlayerInput::ToggleControls);
                        },
                    },
                },

                add_overlay = &adw::HeaderBar {
                    #[watch]
                    set_visible: model.show_controls,
                    set_valign: gtk::Align::Start,
                    add_css_class: "osd",
                    #[wrap(Some)]
                    set_title_widget = &adw::WindowTitle {
                        #[watch]
                        set_title: &model.media.name,
                    },
                    pack_start = &gtk::Button {
                        set_icon_name: "go-previous",
                        connect_clicked[sender] => move |_| {
                            sender.input(VideoPlayerInput::ExitPlayer);
                        },
                    },
                },

                #[name = "overlay"]
                add_overlay = &gtk::Box {
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

                        #[name = "fullscreen_placeholder"]
                        gtk::Box {},
                    },
                },
            },
        }

    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let server = init.0;
        let media = init.1;

        let model = VideoPlayerBuilder::new(media);

        let widgets = view_output!();
        let overlay = &widgets.overlay;
        let video_out = &widgets.video_out;
        let second_row = &widgets.second_row;
        let play_pause_placeholder = &widgets.play_pause_placeholder;
        let volume_placeholder = &widgets.volume_placeholder;
        let fullscreen_placeholder = &widgets.fullscreen_placeholder;

        let (player, paintable) = create_player();
        video_out.set_paintable(Some(&paintable));

        let scrubber = Scrubber::builder()
            .launch(player.downgrade())
            .forward(sender.input_sender(), convert_scrubber_output);
        overlay.prepend(scrubber.widget());

        let play_pause = PlayPause::builder().launch(player.downgrade()).detach();
        second_row.insert_child_after(play_pause.widget(), Some(play_pause_placeholder));

        let volume = Volume::builder().launch(player.downgrade()).detach();
        second_row.insert_child_after(volume.widget(), Some(volume_placeholder));

        let fullscreen = Fullscreen::builder().launch(()).detach();
        second_row.insert_child_after(fullscreen.widget(), Some(fullscreen_placeholder));

        let model = model
            .set_player(&player)
            .set_scrubber(scrubber)
            .set_play_pause(play_pause)
            .set_volume(volume)
            .set_fullscreen(fullscreen)
            .build();

        let url = get_stream_url(&server, &model.media.id);
        player.set_uri(Some(&url));
        relm4::spawn(async move {
            player.play();
        });

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            VideoPlayerInput::ToggleControls => self.show_controls = !self.show_controls,
            VideoPlayerInput::Seek(timestamp) => {
                if let Some(player) = self.player.upgrade() {
                    player.seek(gst::ClockTime::from_seconds(timestamp as u64));
                }
            }
            VideoPlayerInput::ExitPlayer => {
                if let Some(player) = self.player.upgrade() {
                    player.stop();
                }
                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
        }
    }
}

fn convert_scrubber_output(output: ScrubberOutput) -> VideoPlayerInput {
    match output {
        ScrubberOutput::Seek(timestamp) => VideoPlayerInput::Seek(timestamp),
    }
}
