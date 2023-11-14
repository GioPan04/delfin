use std::fmt::Display;

use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, ComponentSender, SimpleComponent};

use crate::tr;

#[derive(Debug)]
pub(super) enum NextPrevEpisodeDirection {
    Next,
    Previous,
}

impl Display for NextPrevEpisodeDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NextPrevEpisodeDirection::Next => write!(f, "next"),
            NextPrevEpisodeDirection::Previous => write!(f, "previous"),
        }
    }
}

#[derive(Debug)]
pub(super) struct NextPrevEpisode {
    direction: NextPrevEpisodeDirection,
    show: bool,
}

#[derive(Debug)]
pub(super) enum NextPrevEpisodeInput {
    Show,
    Hide,
}

#[derive(Debug)]
pub(super) enum NextPrevEpisodeOutput {
    Clicked,
}

#[relm4::component(pub(super))]
impl SimpleComponent for NextPrevEpisode {
    type Init = NextPrevEpisodeDirection;
    type Input = NextPrevEpisodeInput;
    type Output = NextPrevEpisodeOutput;

    view! {
        gtk::Button {
            set_focus_on_click: false,

            #[watch]
            set_sensitive: model.show,
            #[watch]
            set_icon_name: if matches!(model.direction, NextPrevEpisodeDirection::Next) {
                "fast-forward-filled"
            } else {
                "rewind-filled"
            },
            #[watch]
            set_tooltip_text: Some(tr!(
                "vp-next-prev-episode-tooltip",
                {"direction" => model.direction.to_string()},
            )),

            connect_clicked[sender] => move |_| {
                sender.output(NextPrevEpisodeOutput::Clicked).unwrap();
            },
        }
    }

    fn init(
        direction: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = NextPrevEpisode {
            direction,
            show: false,
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            NextPrevEpisodeInput::Show => self.show = true,
            NextPrevEpisodeInput::Hide => self.show = false,
        }
    }
}
