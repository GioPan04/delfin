use std::{cell::RefCell, sync::Arc};

use gtk::prelude::*;
use relm4::{
    component::{AsyncComponent, AsyncComponentParts},
    prelude::*,
    AsyncComponentSender,
};
use uuid::Uuid;

use crate::{
    globals::CONFIG,
    jellyfin_api::{api::intro_skipper::IntroTimestamps, api_client::ApiClient},
    tr,
};

use super::backends::VideoPlayerBackend;

#[derive(Debug)]
enum State {
    Hidden,
    Visible,
    AutoSkipCountdown(usize),
}

impl State {
    fn visibility(&self) -> bool {
        matches!(self, State::Visible | State::AutoSkipCountdown(_))
    }

    fn label(&self) -> String {
        match self {
            Self::Visible => tr!("vp-skip-intro.manual").to_string(),
            Self::AutoSkipCountdown(seconds) => {
                tr!("vp-skip-intro.auto", { "seconds" => seconds }).to_string()
            }
            _ => String::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct SkipIntro {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    intro_timestamps: Option<IntroTimestamps>,
    state: State,
    already_skipped: bool,
    enabled: bool,
    auto_skip: bool,
}

#[derive(Debug)]
pub(crate) enum SkipIntroInput {
    Load(Uuid, Arc<ApiClient>),
    PositionUpdate(usize),
    SkipIntro,
    ConfigUpdated(bool, bool),
}

#[relm4::component(pub(crate) async)]
impl AsyncComponent for SkipIntro {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = SkipIntroInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Button {
            #[watch]
            set_visible: model.state.visibility(),
            #[watch]
            set_label: model.state.label().as_ref(),
            add_css_class: "opaque",
            set_margin_bottom: 12,
            set_halign: gtk::Align::End,

            connect_clicked[sender] => move |_| {
                sender.input(SkipIntroInput::SkipIntro);
            },
        }
    }

    async fn init(
        video_player: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        video_player
            .borrow_mut()
            .connect_position_updated(Box::new({
                let sender = sender.clone();
                move |position| {
                    sender.input(SkipIntroInput::PositionUpdate(position));
                }
            }));

        let model = SkipIntro {
            video_player,
            intro_timestamps: None,
            state: State::Hidden,
            already_skipped: false,
            enabled: CONFIG.read().video_player.intro_skipper,
            auto_skip: CONFIG.read().video_player.intro_skipper_auto_skip,
        };

        model.subscribe_to_config(&sender);

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            SkipIntroInput::Load(id, api_client) => {
                self.intro_timestamps = match api_client.get_intro_timestamps(&id).await {
                    Ok(intro_timestamps) => intro_timestamps,
                    Err(err) => {
                        println!("Error getting intro timestamps for {id}: {err:#?}");
                        None
                    }
                };
                self.already_skipped = false;
            }
            SkipIntroInput::PositionUpdate(position) => 'msg_block: {
                let intro_timestamps = match (self.enabled, &self.intro_timestamps) {
                    (true, Some(intro_timestamps)) => intro_timestamps,
                    _ => {
                        self.state = State::Hidden;
                        break 'msg_block;
                    }
                };

                self.state = match (self.auto_skip, self.already_skipped) {
                    // Auto skip and hide button
                    (true, false)
                        if intro_timestamps.range_intro().contains(&(position as f32)) =>
                    {
                        self.already_skipped = true;
                        self.video_player
                            .borrow()
                            .seek_to(intro_timestamps.intro_end as usize);

                        State::Hidden
                    }

                    // User has auto skip disabled, OR user has auto skip enabled and intro was
                    // already skipped, in which case we show a manual skip button, in case the
                    // user rewound so they could watch the intro
                    (false, _) | (true, true)
                        if intro_timestamps.range_show().contains(&(position as f32)) =>
                    {
                        State::Visible
                    }

                    // Show auto skip countdown
                    (true, _)
                        if intro_timestamps
                            .range_auto_skip_show()
                            .contains(&(position as f32)) =>
                    {
                        State::AutoSkipCountdown((intro_timestamps.intro_start as usize) - position)
                    }

                    _ => State::Hidden,
                };
            }
            SkipIntroInput::SkipIntro => {
                if let Some(intro_end) = self.intro_timestamps.as_ref().map(|ts| ts.intro_end) {
                    self.video_player.borrow().seek_to(intro_end as usize);
                }
            }
            SkipIntroInput::ConfigUpdated(enabled, auto_skip) => {
                self.enabled = enabled;
                self.auto_skip = auto_skip;
            }
        }
    }
}

impl SkipIntro {
    fn subscribe_to_config(&self, sender: &AsyncComponentSender<Self>) {
        CONFIG.subscribe(sender.input_sender(), |config| {
            SkipIntroInput::ConfigUpdated(
                config.video_player.intro_skipper,
                config.video_player.intro_skipper_auto_skip,
            )
        });
    }
}
