use std::rc::Rc;

use gst::ClockTime;
use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, ComponentSender, MessageBroker, SimpleComponent};

use crate::{
    config::video_player_config::VideoPlayerSkipAmount, globals::CONFIG,
    video_player::gst_play_widget::GstVideoPlayer,
};

use super::scrubber::SCRUBBER_BROKER;

pub static SKIP_FORWARDS_BROKER: MessageBroker<SkipForwardsBackwardsInput> = MessageBroker::new();
pub static SKIP_BACKWARDS_BROKER: MessageBroker<SkipForwardsBackwardsInput> = MessageBroker::new();

#[derive(Debug, Clone)]
pub(super) enum SkipForwardsBackwardsDirection {
    Forwards,
    Backwards,
}

#[derive(Debug)]
pub(super) struct SkipForwardsBackwards {
    direction: SkipForwardsBackwardsDirection,
    player: Rc<GstVideoPlayer>,
    skip_amount: VideoPlayerSkipAmount,
    loading: bool,
}

#[derive(Debug)]
pub enum SkipForwardsBackwardsInput {
    Skip,
    SetLoading(bool),
    SkipAmountUpdated(VideoPlayerSkipAmount),
}

#[relm4::component(pub(super))]
impl SimpleComponent for SkipForwardsBackwards {
    type Init = (SkipForwardsBackwardsDirection, Rc<GstVideoPlayer>);
    type Input = SkipForwardsBackwardsInput;
    type Output = ();

    view! {
        gtk::Button {
            #[watch]
            set_icon_name: &model.get_icon_name(),
            #[watch]
            set_tooltip_text: Some(&model.get_tooltip()),
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

        let model = SkipForwardsBackwards::new(direction, player);

        let widgets = view_output!();

        model.subscribe_to_config(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            SkipForwardsBackwardsInput::Skip => {
                if let Some(position) = self.player.position() {
                    let skip_amount = ClockTime::from_seconds(self.skip_amount as u64);
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
            SkipForwardsBackwardsInput::SkipAmountUpdated(skip_amount) => {
                self.skip_amount = skip_amount;
            }
        }
    }
}

impl SkipForwardsBackwards {
    fn new(direction: SkipForwardsBackwardsDirection, player: Rc<GstVideoPlayer>) -> Self {
        let config = CONFIG.read();
        let skip_amount = if let SkipForwardsBackwardsDirection::Forwards = direction {
            config.video_player.skip_forwards_amount
        } else {
            config.video_player.skip_backwards_amount
        };

        Self {
            direction,
            player,
            skip_amount,
            loading: true,
        }
    }

    fn subscribe_to_config(&self, sender: &ComponentSender<Self>) {
        CONFIG.subscribe(sender.input_sender(), {
            let direction = self.direction.clone();
            move |config| {
                SkipForwardsBackwardsInput::SkipAmountUpdated(
                    if let SkipForwardsBackwardsDirection::Forwards = direction {
                        config.video_player.skip_forwards_amount
                    } else {
                        config.video_player.skip_backwards_amount
                    },
                )
            }
        });
    }

    fn get_icon_name(&self) -> String {
        if let SkipForwardsBackwardsDirection::Forwards = self.direction {
            format!("skip-forward-{}", self.skip_amount as usize)
        } else {
            format!("skip-backwards-{}", self.skip_amount as usize)
        }
    }

    fn get_tooltip(&self) -> String {
        if let SkipForwardsBackwardsDirection::Forwards = self.direction {
            format!("Skip forwards {} seconds", self.skip_amount as usize)
        } else {
            format!("Skip backwards {} seconds", self.skip_amount as usize)
        }
    }
}
