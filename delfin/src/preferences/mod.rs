mod general_preferences;
mod video_player_preferences;

use adw::prelude::*;
use relm4::prelude::*;

use crate::tr;

use self::{
    general_preferences::GeneralPreferences, video_player_preferences::VideoPlayerPreferences,
};

pub struct Preferences {
    general_preferences: Controller<GeneralPreferences>,
    video_player_preferences: Controller<VideoPlayerPreferences>,
}

#[relm4::component(pub)]
impl SimpleComponent for Preferences {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        adw::PreferencesWindow {
            set_visible: true,
            set_modal: true,
            set_title: Some(tr!("prefs-window-title")),

            add = model.general_preferences.widget(),
            add = model.video_player_preferences.widget(),
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Preferences {
            general_preferences: GeneralPreferences::builder().launch(()).detach(),
            video_player_preferences: VideoPlayerPreferences::builder().launch(()).detach(),
        };
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
