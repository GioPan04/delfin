use std::{
    sync::{Arc, RwLock},
    thread::sleep,
    time::Duration,
};

use gst::{
    glib::{self, WeakRef},
    ClockTime,
};
use gtk::prelude::*;
use relm4::{gtk, Component, ComponentParts};

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

pub struct Scrubber {
    scrubber_being_moved: Arc<RwLock<bool>>,
    debounce_id: usize,
    duration_display: Arc<RwLock<DurationDisplay>>,
}

#[derive(Debug)]
pub enum ScrubberInput {
    SetScrubberBeingMoved(bool),
    ScrubberMoved,
    ToggleDurationDisplay,
}

#[derive(Debug)]
pub enum ScrubberOutput {
    Seek(f64),
}

#[derive(Debug)]
pub enum ScrubberCommandOutput {
    Debounce(usize),
}

#[relm4::component(pub)]
impl Component for Scrubber {
    type Init = WeakRef<gstplay::Play>;
    type Input = ScrubberInput;
    type Output = ScrubberOutput;
    type CommandOutput = ScrubberCommandOutput;

    view! {
        gtk::Box {
            set_spacing: 8,

            #[name = "position"]
            gtk::Label::new(Some("00:00")) {
                add_css_class: "scrubber-position-label",
            },

            #[name = "scrubber"]
            gtk::Scale {
                set_hexpand: true,
                add_controller = gtk::GestureClick {
                    connect_pressed[sender] => move |_, _, _, _| {
                        sender.input(ScrubberInput::SetScrubberBeingMoved(true));
                    },
                    connect_unpaired_release[sender] => move |_, _, _, _, _| {
                        sender.input(ScrubberInput::SetScrubberBeingMoved(false));
                    },
                    connect_stopped[sender] => move |gesture| {
                        if gesture.current_button() == 0 {
                            sender.input(ScrubberInput::SetScrubberBeingMoved(false));
                        }
                    },
                },
            },


            #[name = "duration"]
            gtk::Button::with_label("42:69") {
                add_css_class: "flat",
                add_css_class: "scrubber-duration-label",
                connect_clicked[sender] => move |_| {
                    sender.input(ScrubberInput::ToggleDurationDisplay);
                },
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let player = init;

        let model = Scrubber {
            scrubber_being_moved: Arc::new(RwLock::new(false)),
            debounce_id: 0,
            duration_display: Arc::new(RwLock::new(DurationDisplay::Total)),
        };

        let widgets = view_output!();
        let scrubber = &widgets.scrubber;
        let position = &widgets.position;
        let duration = &widgets.duration;

        // Allow clicking on any scrubber position to seek to that timestamp
        // By default, this would move the scrubber by a set increment
        let settings = scrubber.settings();
        settings.set_gtk_primary_button_warps_slider(true);

        let scrubber_value_changed_handler = scrubber.connect_value_changed({
            move |_| {
                sender.input(ScrubberInput::ScrubberMoved);
            }
        });

        glib::timeout_add_local(Duration::from_millis(250), {
            let scrubber = scrubber.downgrade();
            let position = position.downgrade();
            let duration = duration.downgrade();
            let scrubber_being_moved = Arc::clone(&model.scrubber_being_moved);
            let duration_display = Arc::clone(&model.duration_display);
            move || {
                if !*scrubber_being_moved.read().unwrap() {
                    if let (Some(player), Some(scrubber), Some(position), Some(duration)) = (
                        player.upgrade(),
                        scrubber.upgrade(),
                        position.upgrade(),
                        duration.upgrade(),
                    ) {
                        scrubber.block_signal(&scrubber_value_changed_handler);

                        if let (Some(position_time), Some(duration_time)) =
                            (player.position(), player.duration())
                        {
                            if !*scrubber_being_moved.read().unwrap() {
                                position.set_label(&position_time.to_timestamp());

                                match *duration_display.read().unwrap() {
                                    DurationDisplay::Total => {
                                        duration.set_label(&duration_time.to_timestamp());
                                    }
                                    DurationDisplay::Remaining => {
                                        let timestamp = &duration_time
                                            .wrapping_sub(position_time)
                                            .to_timestamp();
                                        duration.set_label(&format!("-{}", timestamp));
                                    }
                                }

                                scrubber.set_range(0.0, duration_time.seconds() as f64);
                                scrubber.set_value(position_time.seconds() as f64);
                            } else {
                                position.set_label(
                                    &ClockTime::from_seconds(scrubber.value() as u64)
                                        .to_timestamp(),
                                );
                            }
                        }

                        scrubber.unblock_signal(&scrubber_value_changed_handler);
                    }
                }

                glib::Continue(true)
            }
        });

        ComponentParts { model, widgets }
    }

    fn update(
        &mut self,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ScrubberInput::ScrubberMoved => {
                self.debounce_id = self.debounce_id.wrapping_add(1);
                let id = self.debounce_id;
                sender.spawn_oneshot_command(move || {
                    sleep(Duration::from_millis(250));
                    ScrubberCommandOutput::Debounce(id)
                });
            }
            ScrubberInput::SetScrubberBeingMoved(scrubber_being_moved) => {
                *self.scrubber_being_moved.write().unwrap() = scrubber_being_moved;
            }

            ScrubberInput::ToggleDurationDisplay => {
                let mut duration_display = self.duration_display.write().unwrap();
                *duration_display = duration_display.toggle();
            }
        }
    }

    fn update_cmd_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::CommandOutput,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ScrubberCommandOutput::Debounce(id) => {
                if id == self.debounce_id {
                    sender
                        .output(ScrubberOutput::Seek(widgets.scrubber.value()))
                        .unwrap();
                }
            }
        }
    }
}

trait ToTimestamp {
    fn to_timestamp(self) -> String;
}

impl ToTimestamp for gst::ClockTime {
    fn to_timestamp(self) -> String {
        let minutes = self.seconds() / 60;
        let seconds = self.seconds() % 60;
        format!("{:0>2}:{:0>2}", minutes, seconds)
    }
}
