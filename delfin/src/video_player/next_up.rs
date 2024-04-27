use core::fmt;
use std::{cell::RefCell, collections::VecDeque, sync::Arc};

use gtk::prelude::*;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    gtk::{
        gdk::{self, Texture},
        gdk_pixbuf,
    },
    prelude::*,
    SharedState,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::api_client::ApiClient,
    tr,
    utils::item_name::ItemName,
};

use super::backends::VideoPlayerBackend;

// How many seconds should be remaining when the next up popup appears
const SHOW_NEXT_UP_AT: usize = 30;

pub(crate) static NEXT_UP_VISIBILE: SharedState<bool> = SharedState::new();

#[derive(Debug)]
struct NextUpItem {
    item: BaseItemDto,
    name: Option<String>,
}

impl NextUpItem {
    fn new(item: BaseItemDto) -> Self {
        Self {
            name: item.episode_name_with_number().or(item.name.clone()),
            item,
        }
    }
}

#[derive(Debug)]
struct MaybeNextUpItem(Option<NextUpItem>);

impl fmt::Display for MaybeNextUpItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = self
            .0
            .as_ref()
            .and_then(|n| n.name.clone())
            .unwrap_or(tr!("vp-unnamed-item").to_string());
        write!(f, "{name}")
    }
}

#[derive(Debug)]
pub(crate) struct NextUp {
    state: NextUpState,
    next_up: MaybeNextUpItem,
    duration: Option<usize>,
    thumbnail: Option<Texture>,
}

#[derive(Debug)]
enum NextUpState {
    Ready,
    Shown(usize),
    Hidden,
}

impl fmt::Display for NextUpState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let remaining = match *self {
            NextUpState::Shown(remaining) => remaining,
            _ => 0,
        };
        write!(
            f,
            "{}",
            tr!("vp-next-up-starting", {"remaining" => remaining}),
        )
    }
}

#[derive(Debug)]
pub(crate) enum NextUpInput {
    Reset,
    SetNextUp((Box<Option<BaseItemDto>>, Arc<ApiClient>)),
    SetDuration(usize),
    SetPosition(usize),
    PlayNext,
    Hide,
}

#[derive(Debug)]
pub(crate) enum NextUpCommandOutput {
    SetThumbnail(VecDeque<u8>),
}

#[relm4::component(pub(crate))]
impl Component for NextUp {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = NextUpInput;
    type Output = ();
    type CommandOutput = NextUpCommandOutput;

    view! {
        gtk::Box {
            #[watch]
            set_visible: matches!(model.state, NextUpState::Shown(_)),

            add_css_class: "osd",
            add_css_class: "next-up",
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 8,
            set_halign: gtk::Align::End,
            set_valign: gtk::Align::End,
            set_margin_end: 24,
            set_margin_bottom: 24,

            gtk::Label {
                #[watch]
                set_label: &model.state.to_string(),

                add_css_class: "body",
                set_halign: gtk::Align::Start,
            },

            gtk::Picture {
                #[watch]
                set_paintable: model.thumbnail.as_ref(),

                add_css_class: "next-up__thumbnail",
                set_width_request: 300,
            },

            gtk::Label {
                #[watch]
                set_label: &model.next_up.to_string(),
                #[watch]
                set_tooltip_text: Some(&model.next_up.to_string()),
                #[watch]
                set_visible: model.next_up.0.as_ref().map_or(false, |n| n.name.is_some()),

                set_halign: gtk::Align::Start,
                set_ellipsize: gtk::pango::EllipsizeMode::End,
                set_max_width_chars: 36,
            },

            gtk::Box {
                add_css_class: "linked",
                set_orientation: gtk::Orientation::Horizontal,
                set_homogeneous: true,

                gtk::Button {
                    add_css_class: "suggested-action",
                    adw::ButtonContent {
                        set_icon_name: "play-filled",
                        set_label: tr!("vp-next-up-action.play"),
                    },

                    connect_clicked[sender] => move |_| {
                        sender.input(NextUpInput::PlayNext);
                    },
                },

                gtk::Button {
                    set_label: tr!("vp-next-up-action.hide"),

                    connect_clicked[sender] => move |_| {
                        sender.input(NextUpInput::Hide);
                    },
                },
            },
        }
    }

    fn init(
        video_player: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NextUp {
            state: NextUpState::Ready,
            next_up: MaybeNextUpItem(None),
            duration: None,
            thumbnail: None,
        };

        video_player.borrow_mut().connect_position_updated({
            let sender = sender.clone();
            Box::new(move |position| {
                sender.input(NextUpInput::SetPosition(position));
            })
        });

        video_player.borrow_mut().connect_duration_updated({
            let sender = sender.clone();
            Box::new(move |duration| {
                sender.input(NextUpInput::SetDuration(duration));
            })
        });

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>, _root: &Self::Root) {
        match message {
            NextUpInput::Reset => {
                self.state = NextUpState::Ready;
                self.next_up.0 = None;
                self.duration = None;
                self.set_visible(false);
            }
            NextUpInput::SetNextUp((next, api_client)) => {
                self.next_up.0 = next.map(NextUpItem::new);
                self.fetch_next_up_thumbnail(&sender, &api_client);
            }
            NextUpInput::SetDuration(duration) => {
                self.duration = Some(duration);
            }
            NextUpInput::SetPosition(position) => match self.state {
                NextUpState::Ready => {
                    if let (Some(_), Some(duration)) = (&self.next_up.0, &self.duration) {
                        if duration.saturating_sub(position) <= SHOW_NEXT_UP_AT {
                            self.set_visible(true);
                        }
                    }
                }
                NextUpState::Shown(_) => {
                    if let Some(duration) = &self.duration {
                        self.state = NextUpState::Shown(duration.saturating_sub(position));
                    }
                }
                NextUpState::Hidden => {}
            },
            NextUpInput::PlayNext => {
                if let Some(next_up) = &self.next_up.0 {
                    APP_BROKER.send(AppInput::PlayVideo(next_up.item.clone()));
                }
            }
            NextUpInput::Hide => {
                self.set_visible(false);
            }
        }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let NextUpCommandOutput::SetThumbnail(img_bytes) = message;
        let pixbuf = gdk_pixbuf::Pixbuf::from_read(img_bytes)
            .expect("Error creating media tile pixbuf: {img_url}");
        self.thumbnail = Some(gdk::Texture::for_pixbuf(&pixbuf));
    }
}

impl NextUp {
    fn set_visible(&mut self, visible: bool) {
        if visible {
            self.state = NextUpState::Shown(SHOW_NEXT_UP_AT);
        } else if let NextUpState::Shown(_) = self.state {
            self.state = NextUpState::Hidden;
        }
        *NEXT_UP_VISIBILE.write() = visible;
    }

    fn fetch_next_up_thumbnail(&mut self, sender: &ComponentSender<Self>, api_client: &ApiClient) {
        if let Some(next_up) = &self.next_up.0 {
            if let Ok(img_url) = api_client.get_next_up_thumbnail_url(&next_up.item) {
                sender.oneshot_command(async {
                    let img_bytes: VecDeque<u8> = reqwest::get(img_url)
                        .await
                        .expect("Error getting media tile image: {img_url}")
                        .bytes()
                        .await
                        .expect("Error getting media tile image bytes: {img_url}")
                        .into_iter()
                        .collect();
                    NextUpCommandOutput::SetThumbnail(img_bytes)
                });
            }
        }
    }
}
