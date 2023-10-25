use std::{cell::RefCell, sync::Arc};

use gtk::prelude::*;
use relm4::{
    actions::{ActionGroupName, ActionName, RelmAction, RelmActionGroup},
    gtk::{self, gio},
    Component, ComponentParts,
};

use crate::video_player::backends::{SubtitleTrack, VideoPlayerBackend};

relm4::new_action_group!(SubtitleActionGroup, "subtitle_actions");
relm4::new_stateful_action!(
    SelectedSubtitleAction,
    SubtitleActionGroup,
    "selected_subtitle",
    Option<i32>,
    Option<i32>
);

#[derive(Debug)]
pub struct Subtitles {
    video_player: Arc<RefCell<dyn VideoPlayerBackend>>,
    menu: gio::Menu,
    subtitles_available: bool,
}

#[derive(Debug)]
pub enum SubtitlesInput {
    Reset,
    SubtitlesUpdated(Vec<SubtitleTrack>),
}

#[relm4::component(pub)]
impl Component for Subtitles {
    type Init = Arc<RefCell<dyn VideoPlayerBackend>>;
    type Input = SubtitlesInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::MenuButton {
            set_icon_name: "closed-captioning",
            set_menu_model: Some(&model.menu),
            set_tooltip_text: Some("No Subtitle Tracks Available"),
            #[watch]
            set_sensitive: model.subtitles_available,
            #[watch]
            set_has_tooltip: !model.subtitles_available,
        }
    }

    fn init(
        video_player: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = Subtitles {
            video_player,
            menu: gio::Menu::new(),
            subtitles_available: false,
        };

        model
            .video_player
            .borrow_mut()
            .connect_subtitle_tracks_updated(Box::new(move |tracks| {
                sender.input(SubtitlesInput::SubtitlesUpdated(tracks.clone()));
            }));

        let widgets = view_output!();

        let selected_subtitle_action: RelmAction<SelectedSubtitleAction> =
            RelmAction::new_stateful_with_target_value(&None, {
                let video_player = model.video_player.clone();
                move |_, state, value: Option<i32>| {
                    *state = value;
                    video_player
                        .borrow()
                        .set_subtitle_track(value.map(|id| id as usize));
                }
            });

        let mut group = RelmActionGroup::<SubtitleActionGroup>::new();
        group.add_action(selected_subtitle_action);
        group.register_for_widget(root);

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: relm4::ComponentSender<Self>,
        root: &Self::Root,
    ) {
        match message {
            SubtitlesInput::Reset => {
                self.subtitles_available = false;
            }
            SubtitlesInput::SubtitlesUpdated(subtitle_streams) => {
                if subtitle_streams.is_empty() {
                    return;
                }

                self.subtitles_available = true;

                // Build updated subtitle tracks menu
                let subs_menu = gio::Menu::new();
                subtitle_streams
                    .iter()
                    .map(|subtitle_stream| {
                        RelmAction::<SelectedSubtitleAction>::to_menu_item_with_target_value(
                            &subtitle_stream.name,
                            &Some(subtitle_stream.id as i32),
                        )
                    })
                    .for_each(|menu_item| subs_menu.append_item(&menu_item));
                subs_menu.append_item(
                    &RelmAction::<SelectedSubtitleAction>::to_menu_item_with_target_value(
                        "Off", &None,
                    ),
                );

                self.menu.remove_all();
                self.menu.append_section(Some("Subtitle Track"), &subs_menu);

                // Select current subtitle track in menu
                if let Some(current_subtitle_track) =
                    self.video_player.borrow().current_subtitle_track()
                {
                    root.activate_action(
                        &format!(
                            "{}.{}",
                            SubtitleActionGroup::NAME,
                            SelectedSubtitleAction::NAME
                        ),
                        Some(&Some(current_subtitle_track as i32).to_variant()),
                    )
                    .expect("Error selecting current subtitle track.");
                }
            }
        }

        self.update_view(widgets, sender);
    }
}
