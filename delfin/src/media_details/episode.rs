use std::{cell::OnceCell, collections::VecDeque, sync::Arc};

use adw::prelude::*;
use gdk::Texture;
use gtk::{gdk, gdk_pixbuf};
use jellyfin_api::types::BaseItemDto;
use relm4::{
    gtk::gdk_pixbuf::{InterpType, Pixbuf},
    prelude::*,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::api_client::ApiClient,
    library::LIBRARY_REFRESH_QUEUED,
    media_details::watched_state::{watched_label, Played},
    tr,
};

use super::{watched_state::toggle_watched, MediaDetailsInput, MEDIA_DETAILS_BROKER};

const EPISODE_THUMBNAIL_SIZE: u16 = 75;

pub(crate) struct Episode {
    media: BaseItemDto,
    api_client: Arc<ApiClient>,
    thumbnail: OnceCell<AsyncController<EpisodeThumbnail>>,
}

#[derive(Debug)]
pub(crate) enum EpisodeInput {
    ToggleWatched(bool),
}

#[relm4::component(pub(crate) async)]
impl AsyncComponent for Episode {
    type Init = (BaseItemDto, Arc<ApiClient>);
    type Input = EpisodeInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        adw::ActionRow {
            set_title: &title,
            set_title_lines: 1,
            set_use_markup: false,

            set_subtitle: if let Some(overview) = &model.media.overview {
                // Limit description to first paragraph, otherwise ActionRow
                // doesn't ellipsize the text properly
                overview.split_once('\n').unwrap_or((overview, "")).0
            } else { "" },
            set_subtitle_lines: 3,

            add_suffix = &gtk::ToggleButton {
                set_icon_name: "eye-open-negative-filled",
                set_css_classes: &["image-button", "flat", "btn-watched"],
                set_valign: gtk::Align::Center,
                #[watch]
                set_tooltip: &watched_label(model.media.played()),

                #[watch]
                #[block_signal(toggle_handler)]
                set_active: model.media.played(),
                connect_toggled[sender] => move |btn| {
                    sender.input(EpisodeInput::ToggleWatched(btn.is_active()));
                } @toggle_handler,
            },

            set_activatable: true,
            connect_activated[media] => move |_| {
                APP_BROKER.send(AppInput::PlayVideo(media.clone()));
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (media, api_client) = init;

        let model = Self {
            media: media.clone(),
            api_client: api_client.clone(),
            thumbnail: OnceCell::new(),
        };

        let title = match (&model.media.index_number, &model.media.name) {
            (Some(index_number), Some(name)) => format!("{index_number}. {name}"),
            (_, Some(name)) => name.clone(),
            _ => tr!("media-details-unnamed-episode").to_string(),
        };

        let widgets = view_output!();

        let thumbnail = EpisodeThumbnail::builder()
            .launch((media, api_client))
            .detach();
        root.add_prefix(thumbnail.widget());

        model.thumbnail.set(thumbnail).unwrap();

        AsyncComponentParts { model, widgets }
    }

    async fn update(
        &mut self,
        message: Self::Input,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            EpisodeInput::ToggleWatched(watched) => {
                self.media.user_data =
                    match toggle_watched(&self.media, &self.api_client, watched).await {
                        Ok(user_data) => Some(user_data),
                        Err(err) => {
                            tracing::error!(
                                "Failed to mark episode as {}: {err}",
                                watched_label(watched)
                            );
                            APP_BROKER.send(AppInput::Toast(
                                tr!("media-details-toggle-watched-error", {
                                    "type" => "episode",
                                    "watched" => watched.to_string(),
                                })
                                .to_owned(),
                                None,
                            ));
                            return;
                        }
                    };
                *LIBRARY_REFRESH_QUEUED.write() = true;
                MEDIA_DETAILS_BROKER.send(MediaDetailsInput::UpdatePlayNext);
            }
        }
    }
}

#[derive(Debug)]
struct EpisodeThumbnail {
    media: BaseItemDto,
    thumbnail: Option<Texture>,
}
#[derive(Debug)]
enum EpisodeThumbnailCommandOutput {
    LoadThumbnail(VecDeque<u8>),
}

#[relm4::component(async)]
impl AsyncComponent for EpisodeThumbnail {
    type Init = (BaseItemDto, Arc<ApiClient>);
    type Input = ();
    type Output = ();
    type CommandOutput = EpisodeThumbnailCommandOutput;

    view! {
        gtk::Box {
            gtk::Overlay {
                set_margin_top: 8,
                set_margin_bottom: 8,

                gtk::Picture {
                    #[watch]
                    set_paintable: model.thumbnail.as_ref(),

                    add_css_class: "episode-thumbnail",
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: EPISODE_THUMBNAIL_SIZE.into(),
                    set_height_request: EPISODE_THUMBNAIL_SIZE.into(),
                    set_content_fit: gtk::ContentFit::Cover,
                },

                add_overlay = &gtk::Box {
                    #[watch]
                    set_visible: !model.media.user_data.as_ref()
                        .and_then(|user_data| user_data.play_count)
                        .unwrap_or(0) == 0,
                    set_tooltip: "This episode has never been played",
                    add_css_class: "episode-unplayed-indicator",
                    set_halign: gtk::Align::End,
                    set_valign: gtk::Align::Start,
                    set_width_request: 12,
                    set_height_request: 12,
                },

                add_overlay = &gtk::Spinner {
                    #[watch]
                    set_visible: model.thumbnail.is_none(),
                    set_spinning: true,
                    set_halign: gtk::Align::Center,
                    set_valign: gtk::Align::Center,
                    set_width_request: 24,
                    set_height_request: 24,
                },
            },
        }
    }

    async fn init(
        init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let (media, api_client) = init;

        // TODO: convert to async component and remove this ugliness
        if let Some(img_url) =
            api_client.get_episode_primary_image_url(&media, EPISODE_THUMBNAIL_SIZE)
        {
            if let Ok(img_bytes) = api_client.get_image(&img_url).await {
                sender.oneshot_command(async {
                    EpisodeThumbnailCommandOutput::LoadThumbnail(img_bytes)
                });
            }
        }

        let model = Self {
            media,
            thumbnail: None,
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            EpisodeThumbnailCommandOutput::LoadThumbnail(img_bytes) => {
                let pixbuf = Pixbuf::from_read(img_bytes)
                    .expect("Error creating media tile pixbuf: {img_url}");

                // Resize image to match thumbnail widget size
                // This makes widget sizing a bit less wonky
                let resized = Pixbuf::new(
                    gdk_pixbuf::Colorspace::Rgb,
                    false,
                    8,
                    EPISODE_THUMBNAIL_SIZE.into(),
                    EPISODE_THUMBNAIL_SIZE.into(),
                )
                .unwrap();

                let offset_x = ((pixbuf.width() - EPISODE_THUMBNAIL_SIZE as i32) / 2).abs() as f64;

                pixbuf.scale(
                    &resized,
                    0,
                    0,
                    EPISODE_THUMBNAIL_SIZE.into(),
                    EPISODE_THUMBNAIL_SIZE.into(),
                    -offset_x,
                    0.0,
                    1.0,
                    1.0,
                    InterpType::Nearest,
                );

                self.thumbnail = Some(gdk::Texture::for_pixbuf(&resized));
            }
        }
    }
}
