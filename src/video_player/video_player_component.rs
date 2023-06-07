use gst::glib::WeakRef;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::main_window::get_main_window;
use crate::video_player::player::create_player;
use crate::video_player::scrubber::Scrubber;

use super::scrubber::ScrubberOutput;

struct VideoPlayerBuilder {
    media: LatestMedia,
    scrubber: Option<Controller<Scrubber>>,
    player: Option<WeakRef<gstplay::Play>>,
    show_controls: bool,
    fullscreen: bool,
    playing: bool,
}

impl VideoPlayerBuilder {
    fn new(media: LatestMedia) -> Self {
        Self {
            media,
            scrubber: None,
            player: None,
            show_controls: false,
            playing: true,
            fullscreen: false,
        }
    }

    fn set_player(mut self, player: &gstplay::Play) -> Self {
        self.player = Some(player.downgrade());
        self
    }

    fn set_scrubber(mut self, scrubber: Controller<Scrubber>) -> Self {
        self.scrubber = Some(scrubber);
        self
    }

    fn build(self) -> VideoPlayer {
        let player = self
            .player
            .expect("Tried to build VideoPlayer without player.");

        if self.scrubber.is_none() {
            panic!("Tried to build VideoPlayer without scrubber.");
        }

        VideoPlayer {
            media: self.media.clone(),
            _scrubber: self.scrubber,
            player,
            show_controls: self.show_controls,
            playing: self.playing,
            fullscreen: self.fullscreen,
        }
    }
}

pub struct VideoPlayer {
    media: LatestMedia,
    // We need to keep this controller around, even if we don't read it
    _scrubber: Option<Controller<Scrubber>>,
    player: WeakRef<gstplay::Play>,
    show_controls: bool,
    playing: bool,
    fullscreen: bool,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ToggleControls,
    TogglePlaying,
    Seek(f64),
    ToggleFullscreen,
    WindowFullscreenChanged(bool),
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

                    gtk::Box {
                        gtk::Button {
                            #[watch]
                            set_icon_name: if model.playing {
                                "media-playback-pause"
                            } else {
                                "media-playback-start"
                            },
                            #[watch]
                            set_tooltip_text: Some(if model.playing {
                                "Pause"
                            } else {
                                "Play"
                            }),
                            connect_clicked[sender] => move |_| {
                                sender.input(VideoPlayerInput::TogglePlaying);
                            },
                        },

                        gtk::Button {
                            #[watch]
                            // TODO: probably find better icons
                            set_icon_name: if model.fullscreen {
                                "view-restore"
                            } else {
                                "view-fullscreen"
                            },
                            #[watch]
                            set_tooltip_text: Some(if model.fullscreen {
                                "Exit fullscreen"
                            } else {
                                "Enter fullscreen"
                            }),
                            set_halign: gtk::Align::End,
                            set_hexpand: true,
                            connect_clicked[sender] => move |_| {
                                sender.input(VideoPlayerInput::ToggleFullscreen);
                            },
                        },
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

        let (player, paintable) = create_player();
        video_out.set_paintable(Some(&paintable));

        let scrubber = Scrubber::builder()
            .launch(player.downgrade())
            .forward(sender.input_sender(), convert_scrubber_output);
        overlay.prepend(scrubber.widget());

        let mut model = model.set_player(&player).set_scrubber(scrubber).build();

        let url = get_stream_url(&server, &model.media.id);
        player.set_uri(Some(&url));
        player.play();

        if let Some(window) = get_main_window() {
            model.fullscreen = window.is_fullscreen();
            window.connect_notify(Some("fullscreened"), move |window, _| {
                sender.input(VideoPlayerInput::WindowFullscreenChanged(
                    window.is_fullscreen(),
                ));
            });
        }

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, root: &Self::Root) {
        match message {
            VideoPlayerInput::ToggleControls => self.show_controls = !self.show_controls,
            VideoPlayerInput::TogglePlaying => {
                if let Some(player) = self.player.upgrade() {
                    match self.playing {
                        true => {
                            player.pause();
                            self.playing = false;
                        }
                        false => {
                            player.play();
                            self.playing = true;
                        }
                    }
                }
            }
            VideoPlayerInput::Seek(timestamp) => {
                if let Some(player) = self.player.upgrade() {
                    player.seek(gst::ClockTime::from_seconds(timestamp as u64));
                }
            }
            VideoPlayerInput::ToggleFullscreen => {
                self.fullscreen = !self.fullscreen;
                if let Some(window) = root.toplevel_window() {
                    window.set_fullscreened(self.fullscreen);
                }
            }
            VideoPlayerInput::WindowFullscreenChanged(fullscreen) => {
                self.fullscreen = fullscreen;
            }
            VideoPlayerInput::ExitPlayer => {
                // if let Some(playback_timeout_id) = self.playback_timeout_id.take() {
                //     playback_timeout_id.remove();
                // }
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
