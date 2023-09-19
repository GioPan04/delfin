use gtk::prelude::*;
use relm4::{prelude::*, ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub(super) enum NextPrevEpisodeDirection {
    Next,
    Previous,
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
            #[watch]
            set_sensitive: model.show,
            #[watch]
            set_icon_name: if matches!(model.direction, NextPrevEpisodeDirection::Next) {
                "fast-forward-filled"
            } else {
                "rewind-filled"
            },
            #[watch]
            set_tooltip_text: Some(if matches!(model.direction, NextPrevEpisodeDirection::Next) {
                "Next episode"
            } else {
                "Previous episode"
            }),

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
