use std::{cell::RefCell, fmt::Display, sync::Arc, time::Duration};

use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, ComponentSender, SimpleComponent};

use crate::{
    config::video_player_config::VideoPlayerSkipAmount, globals::CONFIG, tr,
    utils::message_broker::ResettableMessageBroker, video_player::backends::VideoPlayerBackend,
};

use super::scrubber::{ScrubberInput, SCRUBBER_BROKER};

pub(crate) static SKIP_FORWARDS_BROKER: ResettableMessageBroker<SkipForwardsBackwardsInput> =
    ResettableMessageBroker::new();
pub(crate) static SKIP_BACKWARDS_BROKER: ResettableMessageBroker<SkipForwardsBackwardsInput> =
    ResettableMessageBroker::new();

#[derive(Debug, Clone)]
pub(super) enum SkipForwardsBackwardsDirection {
    Forwards,
    Backwards,
}

impl Display for SkipForwardsBackwardsDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipForwardsBackwardsDirection::Forwards => write!(f, "forwards"),
            SkipForwardsBackwardsDirection::Backwards => write!(f, "backwards"),
        }
    }
}

#[derive(Debug)]
pub(super) struct SkipForwardsBackwards {
    direction: SkipForwardsBackwardsDirection,
    player: Arc<RefCell<dyn VideoPlayerBackend>>,
    skip_amount: VideoPlayerSkipAmount,
    loading: bool,
}

#[derive(Debug)]
pub enum SkipForwardsBackwardsInput {
    Skip,
    SkipByAmount(Duration),
    SkipTo(Duration),
    SetLoading(bool),
    SkipAmountUpdated(VideoPlayerSkipAmount),
    FrameStep,
}

#[relm4::component(pub(super))]
impl SimpleComponent for SkipForwardsBackwards {
    type Init = (
        SkipForwardsBackwardsDirection,
        Arc<RefCell<dyn VideoPlayerBackend>>,
    );
    type Input = SkipForwardsBackwardsInput;
    type Output = ();

    view! {
        gtk::Button {
            set_focus_on_click: false,

            #[watch]
            set_icon_name: &model.get_icon_name(),
            #[watch]
            set_tooltip_text: Some(tr!(
                "vp-skip-forwards-backwards-tooltip", {
                    "direction" => model.direction.to_string(),
                    "seconds" => model.skip_amount as usize,
                },
            )),
            #[watch]
            set_sensitive: !model.loading,

            connect_clicked[sender] => move |_| {
                sender.input(SkipForwardsBackwardsInput::Skip);
            },
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let (direction, player) = init;

        let model = SkipForwardsBackwards::new(direction, player);

        let widgets = view_output!();

        model.subscribe_to_config(&sender);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SkipForwardsBackwardsInput::Skip => {
                sender.input(SkipForwardsBackwardsInput::SkipByAmount(
                    self.skip_amount.into(),
                ));
            }
            SkipForwardsBackwardsInput::SkipByAmount(amount) => {
                let position = self.player.borrow().position();
                let skip_amount = amount.as_secs() as usize;

                let (seek_to, skip_amount) = match self.direction {
                    SkipForwardsBackwardsDirection::Forwards => {
                        (position + skip_amount, skip_amount as isize)
                    }
                    SkipForwardsBackwardsDirection::Backwards => (
                        position.saturating_sub(skip_amount),
                        -(skip_amount as isize),
                    ),
                };

                self.player.borrow().seek_by(skip_amount);

                SCRUBBER_BROKER.send(ScrubberInput::SetPosition(seek_to));
            }
            SkipForwardsBackwardsInput::SkipTo(position) => {
                let position = position.as_secs() as usize;
                self.player.borrow().seek_to(position);
                SCRUBBER_BROKER.send(ScrubberInput::SetPosition(position));
            }
            SkipForwardsBackwardsInput::SetLoading(loading) => {
                self.loading = loading;
            }
            SkipForwardsBackwardsInput::SkipAmountUpdated(skip_amount) => {
                self.skip_amount = skip_amount;
            }
            SkipForwardsBackwardsInput::FrameStep => {
                let player = self.player.borrow();
                if let SkipForwardsBackwardsDirection::Forwards = self.direction {
                    player.frame_step_forwards();
                } else {
                    player.frame_step_backwards();
                }
            }
        }
    }
}

impl SkipForwardsBackwards {
    fn new(
        direction: SkipForwardsBackwardsDirection,
        player: Arc<RefCell<dyn VideoPlayerBackend>>,
    ) -> Self {
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
}
