use std::cell::OnceCell;

use glib::closure;
use gst::ClockTime;
use gtk::{glib, prelude::*};
use relm4::{
    gtk::{self, ExpressionWatch},
    Component, ComponentParts,
};

use crate::video_player::gst_play_widget::GstVideoPlayer;

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
    video_player: OnceCell<GstVideoPlayer>,
    position: u64,
    duration: u64,
    scrubber_being_moved: bool,
    duration_display: DurationDisplay,
    // While the scrubber is being moved, we bind it's value to the position
    // label and store the binding here, so that we can unbind it later.
    scrubber_moving_position_binding: Option<ExpressionWatch>,
}

#[derive(Debug)]
pub enum ScrubberInput {
    SetScrubberBeingMoved(bool),
    ToggleDurationDisplay,
    SetPositionDuration(gst::ClockTime, gst::ClockTime),
}

#[relm4::component(pub)]
impl Component for Scrubber {
    type Init = OnceCell<GstVideoPlayer>;
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
            },

            #[name = "scrubber"]
            gtk::Scale {
                set_hexpand: true,

                #[watch]
                set_value: model.position as f64,
                #[watch]
                set_range: (0.0, model.duration as f64),

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
            gtk::Button {
                #[watch]
                set_label: &duration_to_timestamp(model.position, model.duration, model.duration_display),
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
        let video_player = init;

        let model = Scrubber {
            video_player: video_player.clone(),
            position: 0,
            duration: 100,
            scrubber_being_moved: false,
            duration_display: DurationDisplay::Total,
            scrubber_moving_position_binding: None,
        };

        let widgets = view_output!();
        let scrubber = &widgets.scrubber;

        // Allow clicking on any scrubber position to seek to that timestamp
        // By default, this would move the scrubber by a set increment
        let settings = scrubber.settings();
        settings.set_gtk_primary_button_warps_slider(true);

        let video_player = video_player.get().unwrap();
        video_player.connect_position_updated({
            move |position, duration| {
                sender.input(ScrubberInput::SetPositionDuration(position, duration));
            }
        });

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ScrubberInput::SetScrubberBeingMoved(scrubber_being_moved) => {
                self.scrubber_being_moved = scrubber_being_moved;

                let scrubber = &widgets.scrubber;
                let position = &widgets.position;

                if scrubber_being_moved {
                    self.scrubber_moving_position_binding = Some(
                        scrubber
                            .adjustment()
                            .property_expression("value")
                            .chain_closure::<String>(closure!(
                                |_: Option<glib::Object>, position: f64| {
                                    seconds_to_timestamp(position as u64)
                                }
                            ))
                            .bind(position, "label", gtk::Widget::NONE),
                    );
                } else if let Some(scrubber_moving_position_binding) =
                    &self.scrubber_moving_position_binding
                {
                    scrubber_moving_position_binding.unwatch();
                    self.scrubber_moving_position_binding = None;

                    let video_player = self.video_player.get().unwrap();
                    video_player.seek(ClockTime::from_seconds(scrubber.value() as u64));
                }
            }

            ScrubberInput::ToggleDurationDisplay => {
                self.duration_display = self.duration_display.toggle();
            }
            ScrubberInput::SetPositionDuration(position, duration) => {
                let scrubber = &widgets.scrubber;

                if self.scrubber_being_moved {
                    self.position = scrubber.value() as u64;
                } else {
                    self.position = position.seconds();
                    self.duration = duration.seconds();
                }
            }
        }

        self.update_view(widgets, sender);
    }
}

fn seconds_to_timestamp(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:0>2}:{:0>2}", minutes, seconds)
}

fn duration_to_timestamp(
    position: u64,
    duration: u64,
    duration_display: DurationDisplay,
) -> String {
    match duration_display {
        DurationDisplay::Total => seconds_to_timestamp(duration),
        DurationDisplay::Remaining => {
            let position = ClockTime::from_seconds(position);
            let duration = ClockTime::from_seconds(duration);
            format!(
                "-{}",
                seconds_to_timestamp(duration.wrapping_sub(position).seconds())
            )
        }
    }
}
