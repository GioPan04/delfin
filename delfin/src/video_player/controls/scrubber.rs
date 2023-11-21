use bytes::Buf;
use std::{cell::RefCell, sync::Arc};

use gdk::{Rectangle, Texture};
use graphene::Point;
use gtk::{gdk, gdk_pixbuf, graphene, prelude::*};
use relm4::prelude::*;

use crate::{tr, utils::bif::Thumbnail, video_player::backends::VideoPlayerBackend};

use super::control_broker::ControlBroker;

const TIMESTAMP_WIDTH: i32 = 80;

pub(crate) static SCRUBBER_BROKER: ControlBroker<ScrubberInput> = ControlBroker::new();

#[derive(Clone, Copy, Debug)]
enum DurationDisplay {
    Total,
    Remaining,
}

impl DurationDisplay {
    fn toggle(&self) -> Self {
        match self {
            Self::Total => Self::Remaining,
            Self::Remaining => Self::Total,
        }
    }
}

struct ScrubberPopover {
    position: f64,
    timestamp: usize,
    thumbnail: Option<Texture>,
}

pub(crate) struct Scrubber {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,

    loading: bool,

    position: usize,
    duration: usize,
    duration_display: DurationDisplay,

    scrubbing: bool,
    popover: Option<ScrubberPopover>,
    thumbnails: Option<Vec<Thumbnail>>,
}

#[derive(Debug)]
pub enum ScrubberInput {
    Reset,
    SetPosition(usize),
    SetDuration(usize),
    SetPlaying,
    ToggleDurationDisplay,
    SetScrubbing(bool),
    ScrubberMouseHover(f64),
    ScrubberMouseLeave,
    LoadedThumbnails(Option<Vec<Thumbnail>>),
}

#[relm4::component(pub(crate))]
impl Component for Scrubber {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = ScrubberInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_spacing: 8,

            #[name = "position"]
            gtk::Label {
                #[watch]
                set_label: &seconds_to_timestamp(model.position),
                add_css_class: "scrubber-position-label",
                set_width_request: TIMESTAMP_WIDTH,
            },

            #[name = "scrubber"]
            gtk::Scale {
                set_hexpand: true,
                set_focus_on_click: false,

                #[watch]
                set_value: model.position as f64,
                #[watch]
                set_range: (0.0, model.duration as f64),
                #[watch]
                set_sensitive: !model.loading,

                add_controller = gtk::GestureClick {
                    connect_pressed[sender] => move |_, _, x, _| {
                        sender.input(ScrubberInput::SetScrubbing(true));
                        // Update position so we seek correctly if user immediately releases click
                        sender.input(ScrubberInput::ScrubberMouseHover(x));
                    },
                    connect_unpaired_release[sender] => move |_, _, _, _, _| {
                        sender.input(ScrubberInput::SetScrubbing(false));
                    },
                    connect_stopped[sender] => move |gesture| {
                        if gesture.current_button() == 0 {
                            sender.input(ScrubberInput::SetScrubbing(false));
                        }
                    },
                },

                add_controller = gtk::EventControllerMotion {
                    connect_motion[sender] => move |_, x, _| {
                        sender.input(ScrubberInput::ScrubberMouseHover(x));
                    },
                    connect_leave[sender] => move |_| {
                        sender.input(ScrubberInput::ScrubberMouseLeave);
                    },
                },
            },

            #[name = "popover"]
            gtk::Popover {
                set_autohide: false,
                set_position: gtk::PositionType::Top,

                #[watch]
                set_visible: model.popover.is_some(),
                #[watch]
                set_pointing_to: model.popover
                    .as_ref()
                    .map(|p| Rectangle::new(p.position as i32 , -15, 0, 0))
                    .as_ref(),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 4,

                    gtk::Picture {
                        add_css_class: "scrubber-popover-thumbnail",
                        #[watch]
                        set_paintable: model.popover
                            .as_ref()
                            .and_then(|p| p.thumbnail.as_ref())
                    },

                    #[name = "popover_label"]
                    gtk::Label {
                        #[watch]
                        set_label: &model.popover
                            .as_ref()
                            .map(|p| seconds_to_timestamp(p.timestamp))
                            .unwrap_or("".to_string()),
                    },
                },
            },

            #[name = "duration"]
            gtk::Button {
                set_focus_on_click: false,
                #[watch]
                set_label: &duration_to_timestamp(model.position, model.duration, model.duration_display),
                #[watch]
                set_tooltip_text: Some(&match model.duration_display {
                    DurationDisplay::Total => tr!("vp-duration-tooltip.total").to_string(),
                    DurationDisplay::Remaining => tr!("vp-duration-tooltip.remaining").to_string(),
                }),
                set_width_request: TIMESTAMP_WIDTH,
                add_css_class: "flat",
                add_css_class: "scrubber-duration-label",
                connect_clicked[sender] => move |_| {
                    sender.input(ScrubberInput::ToggleDurationDisplay);
                },
            },
        }
    }

    fn init(
        video_player: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        video_player.borrow_mut().connect_position_updated({
            let sender = sender.clone();
            Box::new(move |position| {
                sender.input(ScrubberInput::SetPosition(position));
            })
        });

        video_player.borrow_mut().connect_duration_updated({
            let sender = sender.clone();
            Box::new(move |duration| {
                sender.input(ScrubberInput::SetDuration(duration));
            })
        });

        let model = Scrubber {
            video_player,
            loading: true,
            position: 0,
            duration: 0,
            duration_display: DurationDisplay::Total,
            scrubbing: false,
            popover: None,
            thumbnails: None,
        };

        let widgets = view_output!();

        relm4::ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let scrubber = &widgets.scrubber;
        let popover = &widgets.popover;

        match message {
            ScrubberInput::Reset => {
                self.loading = true;
                self.position = 0;
                self.duration = 0;
            }
            ScrubberInput::SetPosition(position) => {
                if !self.scrubbing {
                    self.position = position;
                }
            }
            ScrubberInput::SetDuration(duration) => {
                self.duration = duration;
            }
            ScrubberInput::SetPlaying => {
                self.loading = false;
            }
            ScrubberInput::ToggleDurationDisplay => {
                self.duration_display = self.duration_display.toggle();
            }
            ScrubberInput::SetScrubbing(scrubbing) => {
                self.scrubbing = scrubbing;

                if !scrubbing {
                    self.video_player.borrow().seek_to(self.position);
                }
            }
            ScrubberInput::ScrubberMouseHover(position) => {
                let percent = (position / scrubber.width() as f64).clamp(0.0, 1.0);
                let timestamp = (self.duration as f64 * percent) as usize;

                if self.scrubbing {
                    self.position = timestamp;
                }

                let popover_position =
                    scrubber.compute_point(popover, &Point::new(position as f32, 0.0));

                self.popover = Some(ScrubberPopover {
                    position: popover_position.map(|p| p.x()).unwrap_or(0.0) as f64,
                    timestamp,
                    thumbnail: self.get_thumbnail(timestamp),
                });
            }
            ScrubberInput::ScrubberMouseLeave => {
                self.popover = None;
            }
            ScrubberInput::LoadedThumbnails(thumbnails) => {
                self.thumbnails = thumbnails;
            }
        }

        self.update_view(widgets, sender);
    }
}

impl Scrubber {
    fn get_thumbnail(&self, timestamp: usize) -> Option<Texture> {
        let thumbnails = match &self.thumbnails {
            Some(thumbnails) => thumbnails,
            None => return None,
        };

        let mut nearest_thumbnail_idx = 0;
        for (i, thumbnail) in thumbnails.iter().enumerate() {
            if thumbnail.timestamp < timestamp {
                nearest_thumbnail_idx = i
            }
        }

        let image = &thumbnails[nearest_thumbnail_idx].image;
        let pixbuf = match gdk_pixbuf::Pixbuf::from_read(image.clone().reader()) {
            Ok(pixbuf) => pixbuf,
            Err(err) => {
                println!(
                    "Error creating pixbuf for scrubber thumbnail at timestamp {timestamp}: {err}"
                );
                return None;
            }
        };
        Some(Texture::for_pixbuf(&pixbuf))
    }
}

fn seconds_to_timestamp(seconds: usize) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:0>2}:{:0>2}", minutes, seconds)
}

fn duration_to_timestamp(
    position: usize,
    duration: usize,
    duration_display: DurationDisplay,
) -> String {
    match duration_display {
        DurationDisplay::Total => seconds_to_timestamp(duration),
        DurationDisplay::Remaining => {
            format!("-{}", seconds_to_timestamp(duration.wrapping_sub(position)))
        }
    }
}
