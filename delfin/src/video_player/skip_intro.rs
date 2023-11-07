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
pub(crate) struct SkipIntro {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    intro_timestamps: Option<IntroTimestamps>,
    visible: bool,
}

#[derive(Debug)]
pub(crate) enum SkipIntroInput {
    Load(Uuid, Arc<ApiClient>),
    PositionUpdate(usize),
    SkipIntro,
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
            set_visible: model.visible,
            set_label: tr!("vp-skip-intro"),
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
            visible: false,
        };

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
            }
            SkipIntroInput::PositionUpdate(position) => {
                self.visible = match &self.intro_timestamps {
                    Some(intro_timestamps) => {
                        intro_timestamps.range_show().contains(&(position as f32))
                            && CONFIG.read().video_player.intro_skipper
                    }
                    _ => false,
                }
            }
            SkipIntroInput::SkipIntro => {
                if let Some(intro_end) = self.intro_timestamps.as_ref().map(|ts| ts.intro_end) {
                    self.video_player.borrow().seek_to(intro_end as usize);
                }
            }
        }
    }
}
