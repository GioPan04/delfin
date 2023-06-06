use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use gst::glib;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4::{gtk, ComponentParts};

use crate::api::item::get_stream_url;
use crate::api::latest::LatestMedia;
use crate::config::Server;
use crate::video_player::gtksink::create_gtk_sink;
use crate::video_player::player::create_player;

struct VideoPlayerBuilder {
    media: LatestMedia,
    player: Option<gstplay::Play>,
    playback_timeout_id: Option<glib::SourceId>,
    show_controls: bool,
    playing: bool,
    scrubber_being_moved: Arc<RwLock<bool>>,
    scrubber_debounce_id: usize,
}

impl VideoPlayerBuilder {
    fn new(media: LatestMedia) -> Self {
        Self {
            media,
            player: None,
            playback_timeout_id: None,
            show_controls: false,
            playing: true,
            scrubber_being_moved: Arc::new(RwLock::new(false)),
            scrubber_debounce_id: 0,
        }
    }

    fn set_player(mut self, player: gstplay::Play) -> Self {
        self.player = Some(player);
        self
    }

    fn set_playback_timeout_id(mut self, playback_timeout_id: glib::SourceId) -> Self {
        self.playback_timeout_id = Some(playback_timeout_id);
        self
    }

    fn build(self) -> VideoPlayer {
        let player = self
            .player
            .expect("Tried to build VideoPlayer without player.");
        let playback_timeout_id = self
            .playback_timeout_id
            .expect("Tried to build VideoPlayer without playback_timeout_id.");
        VideoPlayer {
            media: self.media.clone(),
            player,
            playback_timeout_id: Some(playback_timeout_id),
            show_controls: self.show_controls,
            playing: self.playing,
            scrubber_being_moved: self.scrubber_being_moved,
            scrubber_debounce_id: self.scrubber_debounce_id,
        }
    }
}

pub struct VideoPlayer {
    media: LatestMedia,
    player: gstplay::Play,
    playback_timeout_id: Option<glib::SourceId>,
    show_controls: bool,
    playing: bool,
    scrubber_being_moved: Arc<RwLock<bool>>,
    scrubber_debounce_id: usize,
}

#[derive(Debug)]
pub enum VideoPlayerInput {
    ToggleControls,
    TogglePlaying,
    ScrubberBeingMoved(bool),
    ScrubberMoved,
    Seek(f64),
    ExitPlayer,
}

#[derive(Debug)]
pub enum VideoPlayerOutput {
    NavigateBack,
}

#[derive(Debug)]
pub enum VideoPlayerCommandOutput {
    ScrubberDebounce(usize),
}

#[relm4::component(pub)]
impl Component for VideoPlayer {
    type Init = (Server, LatestMedia);
    type Input = VideoPlayerInput;
    type Output = VideoPlayerOutput;
    type CommandOutput = VideoPlayerCommandOutput;

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

                add_overlay = &gtk::Box {
                    #[watch]
                    set_visible: model.show_controls,
                    set_valign: gtk::Align::End,
                    add_css_class: "toolbar",
                    add_css_class: "osd",
                    add_css_class: "video-player-controls",

                    gtk::Button {
                        #[watch]
                        set_icon_name: if model.playing {
                            "media-playback-pause"
                        } else {
                            "media-playback-start"
                        },
                        connect_clicked[sender] => move |_| {
                            sender.input(VideoPlayerInput::TogglePlaying);
                        },
                    },

                    #[name = "scrubber"]
                    gtk::Scale {
                        set_range: (0.0, 100.0),
                        set_value: 0.0,
                        set_hexpand: true,
                        add_controller = gtk::GestureClick {
                            connect_pressed[sender] => move |_, _, _, _| {
                                sender.input(VideoPlayerInput::ScrubberBeingMoved(true));
                            },
                            connect_unpaired_release[sender] => move |_, _, _, _, _| {
                                sender.input(VideoPlayerInput::ScrubberBeingMoved(false));
                            },
                            connect_stopped[sender] => move |gesture| {
                                if gesture.current_button() == 0 {
                                    sender.input(VideoPlayerInput::ScrubberBeingMoved(false));
                                }
                            },
                        },
                    },

                    #[name = "timestamp"]
                    gtk::Label {
                        set_label: "4:20/69:42",
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

        let (gtksink, paintable) = create_gtk_sink();

        let model = VideoPlayerBuilder::new(media);

        let widgets = view_output!();
        let video_out = &widgets.video_out;
        let scrubber = &widgets.scrubber;
        let timestamp = &widgets.timestamp;

        // Allow clicking on any scrubber position to seek to that timestamp
        // By default, this would move the scrubber by a set increment
        let settings = scrubber.settings();
        settings.set_gtk_primary_button_warps_slider(true);

        let scrubber_value_changed_handler = scrubber.connect_value_changed(move |_| {
            sender.input(VideoPlayerInput::ScrubberMoved);
        });

        let (player, playback_timeout_id) = create_player(
            &gtksink,
            scrubber,
            scrubber_value_changed_handler,
            &model.scrubber_being_moved,
            timestamp,
        );

        let model = model
            .set_player(player)
            .set_playback_timeout_id(playback_timeout_id)
            .build();

        video_out.set_paintable(Some(&paintable));

        let url = get_stream_url(&server, &model.media.id);

        model.player.set_uri(Some(&url));
        model.player.play();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            VideoPlayerInput::ToggleControls => self.show_controls = !self.show_controls,
            VideoPlayerInput::TogglePlaying => match self.playing {
                true => {
                    self.player.pause();
                    self.playing = false;
                }
                false => {
                    self.player.play();
                    self.playing = true;
                }
            },
            VideoPlayerInput::ScrubberBeingMoved(scrubber_being_moved) => {
                *self.scrubber_being_moved.write().unwrap() = scrubber_being_moved;
            }
            VideoPlayerInput::ScrubberMoved => {
                self.scrubber_debounce_id = self.scrubber_debounce_id.wrapping_add(1);
                let id = self.scrubber_debounce_id;
                sender.spawn_oneshot_command(move || {
                    sleep(Duration::from_millis(250));
                    VideoPlayerCommandOutput::ScrubberDebounce(id)
                });
            }
            VideoPlayerInput::Seek(timestamp) => {
                self.player
                    .seek(gst::ClockTime::from_seconds(timestamp as u64));
            }
            VideoPlayerInput::ExitPlayer => {
                if let Some(playback_timeout_id) = self.playback_timeout_id.take() {
                    playback_timeout_id.remove();
                }
                self.player.stop();
                sender.output(VideoPlayerOutput::NavigateBack).unwrap();
            }
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            VideoPlayerCommandOutput::ScrubberDebounce(id) => {
                if id == self.scrubber_debounce_id {
                    sender.input(VideoPlayerInput::Seek(widgets.scrubber.value()));
                }
            }
        }
    }
}
