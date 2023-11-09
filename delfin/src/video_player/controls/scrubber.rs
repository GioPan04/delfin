use std::{
    cell::RefCell,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use gdk::Rectangle;
use graphene::Point;
use gtk::{gdk, graphene, prelude::*};
use relm4::{prelude::*, MessageBroker};

use crate::{tr, video_player::backends::VideoPlayerBackend};

pub(crate) struct ScrubberBroker(RwLock<MessageBroker<ScrubberInput>>);

const TIMESTAMP_WIDTH: i32 = 80;

impl ScrubberBroker {
    const fn new() -> Self {
        Self(RwLock::new(MessageBroker::new()))
    }

    pub(crate) fn read(&self) -> RwLockReadGuard<MessageBroker<ScrubberInput>> {
        self.0.read().unwrap()
    }

    pub(crate) fn reset(&self) {
        *self.0.write().unwrap() = MessageBroker::new();
    }
}

pub(crate) static SCRUBBER_BROKER: ScrubberBroker = ScrubberBroker::new();

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
}

pub(crate) struct Scrubber {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,

    loading: bool,

    position: usize,
    duration: usize,
    duration_display: DurationDisplay,

    scrubbing: bool,
    popover: Option<ScrubberPopover>,
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
                });
            }
            ScrubberInput::ScrubberMouseLeave => {
                self.popover = None;
            }
        }

        self.update_view(widgets, sender);
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
