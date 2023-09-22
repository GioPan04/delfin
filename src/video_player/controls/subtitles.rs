use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use gstplay::{traits::PlayStreamInfoExt, PlaySubtitleInfo};
use gtk::prelude::*;
use relm4::{
    actions::{ActionGroupName, ActionName, RelmAction, RelmActionGroup},
    gtk::{self, gio},
    Component, ComponentParts,
};

use crate::video_player::gst_play_widget::GstVideoPlayer;

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
    video_player: Rc<GstVideoPlayer>,
    menu: gio::Menu,
    subtitle_count: Arc<RwLock<Option<usize>>>,
    subtitles_available: bool,
}

#[derive(Debug)]
pub enum SubtitlesInput {
    Reset,
    SubtitlesUpdated(Vec<PlaySubtitleInfo>),
}

#[relm4::component(pub)]
impl Component for Subtitles {
    type Init = Rc<GstVideoPlayer>;
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
            subtitle_count: Arc::new(RwLock::new(None)),
            subtitles_available: false,
        };

        model.video_player.connect_media_info_updated({
            let subtitle_count = model.subtitle_count.clone();
            move |media_info| {
                let subtitle_streams = media_info.subtitle_streams();
                let subtitle_stream_count = subtitle_streams.len();
                // subtitle_count keeps track of the current subtitle track count for the
                // currently playing media. If a different number is reported we update the
                // subtitles menu.
                match *subtitle_count.read().unwrap() {
                    Some(subtitle_count) if subtitle_count == subtitle_stream_count => {}
                    _ => {
                        sender.input(SubtitlesInput::SubtitlesUpdated(subtitle_streams));
                    }
                };
            }
        });

        let widgets = view_output!();

        let selected_subtitle_action: RelmAction<SelectedSubtitleAction> =
            RelmAction::new_stateful_with_target_value(&None, {
                let video_player = model.video_player.clone();
                move |_, state, value: Option<i32>| {
                    *state = value;

                    video_player.set_subtitle_track_enabled(value.is_some());
                    if let Some(value) = value {
                        video_player
                            .set_subtitle_track(value)
                            .expect("Error setting subtitle track");
                    }
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
                let mut subtitle_count = self.subtitle_count.write().unwrap();
                *subtitle_count = None;

                self.subtitles_available = false;
            }
            SubtitlesInput::SubtitlesUpdated(subtitle_streams) => {
                // Keep track of sub count so we don't update subtitles again
                let mut subtitle_count = self.subtitle_count.write().unwrap();
                *subtitle_count = Some(subtitle_streams.len());

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
                            &subtitle_stream.display_name(),
                            &Some(subtitle_stream.index()),
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
                if let Some(current_subtitle_track) = self.video_player.current_subtitle_track() {
                    root.activate_action(
                        &format!(
                            "{}.{}",
                            SubtitleActionGroup::NAME,
                            SelectedSubtitleAction::NAME
                        ),
                        Some(&Some(current_subtitle_track.index()).to_variant()),
                    )
                    .unwrap();
                }
            }
        }

        self.update_view(widgets, sender);
    }
}

trait PlaySubtitleInfoExt {
    fn display_name(&self) -> String;
}

impl PlaySubtitleInfoExt for PlaySubtitleInfo {
    fn display_name(&self) -> String {
        let mut display_name = self
            .language()
            .map(|l| l.to_string())
            .unwrap_or(self.index().to_string());

        let tags = self.tags();
        if let Some(tags) = tags {
            if let Some(title) = tags.get::<gst::tags::Title>() {
                let title = title.get();
                display_name = format!("{display_name} - {title}");
            }
        }

        display_name
    }
}
