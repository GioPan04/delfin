use std::rc::Rc;

use gst::ClockTime;
use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, ComponentSender, MessageBroker, SimpleComponent};

use crate::video_player::gst_play_widget::GstVideoPlayer;

use super::scrubber::SCRUBBER_BROKER;

pub static SKIP_FORWARDS_BROKER: MessageBroker<SkipForwardsBackwardsInput> = MessageBroker::new();
pub static SKIP_BACKWARDS_BROKER: MessageBroker<SkipForwardsBackwardsInput> = MessageBroker::new();

#[derive(Debug)]
pub(super) enum SkipForwardsBackwardsDirection {
    Forwards,
    Backwards,
}

#[derive(Debug)]
pub(super) struct SkipForwardsBackwards {
    direction: SkipForwardsBackwardsDirection,
    player: Rc<GstVideoPlayer>,
    loading: bool,
}

#[derive(Debug)]
pub enum SkipForwardsBackwardsInput {
    Skip,
    SetLoading(bool),
}

#[relm4::component(pub(super))]
impl SimpleComponent for SkipForwardsBackwards {
    type Init = (SkipForwardsBackwardsDirection, Rc<GstVideoPlayer>);
    type Input = SkipForwardsBackwardsInput;
    type Output = ();

    view! {
        gtk::Button {
            #[watch]
            set_icon_name: if matches!(model.direction, SkipForwardsBackwardsDirection::Forwards) {
                "skip-forward-10"
            } else {
                "skip-backwards-10"
            },
            #[watch]
            set_tooltip_text: Some(if matches!(model.direction, SkipForwardsBackwardsDirection::Forwards) {
                "Skip forwards"
            } else {
                "Skip backwards"
            }),
            #[watch]
            set_sensitive: !model.loading,

            connect_clicked[sender] => move |_| {
                sender.input(SkipForwardsBackwardsInput::Skip);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (direction, player) = init;
        let model = SkipForwardsBackwards {
            direction,
            player,
            loading: true,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SkipForwardsBackwardsInput::Skip => {
                if let Some(position) = self.player.position() {
                    let skip_amount = ClockTime::from_seconds(10);
                    let seek_to = if let SkipForwardsBackwardsDirection::Forwards = self.direction {
                        position.saturating_add(skip_amount)
                    } else {
                        position.saturating_sub(skip_amount)
                    };
                    SKIP_FORWARDS_BROKER.send(SkipForwardsBackwardsInput::SetLoading(true));
                    SKIP_BACKWARDS_BROKER.send(SkipForwardsBackwardsInput::SetLoading(true));
                    SCRUBBER_BROKER.send(super::scrubber::ScrubberInput::SetPosition(seek_to));
                    self.player.seek(seek_to);
                }
            }
            SkipForwardsBackwardsInput::SetLoading(loading) => {
                self.loading = loading;
            }
        }
    }
}
