use std::{cell::OnceCell, collections::VecDeque};

use adw::prelude::*;
use gdk::Texture;
use gtk::{gdk, gdk_pixbuf};
use relm4::{
    gtk::gdk_pixbuf::{InterpType, Pixbuf},
    prelude::*,
};

use crate::{
    app::{AppInput, APP_BROKER},
    jellyfin_api::models::media::Media,
};

pub const EPISODE_THUMBNAIL_SIZE: i32 = 75;

pub(crate) struct Episode {
    media: Media,
    thumbnail: OnceCell<Controller<EpisodeThumbnail>>,
}

#[relm4::component(pub(crate))]
impl SimpleComponent for Episode {
    type Init = Media;
    type Input = ();
    type Output = ();

    view! {
        adw::ActionRow {
            set_title: &model.media.name,
            set_title_lines: 1,
            set_use_markup: false,

            set_subtitle: if let Some(overview) = &model.media.overview {
                // Limit description to first paragraph, otherwise ActionRow
                // doesn't ellipsize the text properly
                overview.split_once('\n').unwrap_or((overview, "")).0
            } else { "" },
            set_subtitle_lines: 3,

            add_suffix = &gtk::Image {
                set_icon_name: Some("go-next-symbolic"),
            },

            set_activatable: true,
            connect_activated[media] => move |_| {
                APP_BROKER.send(AppInput::PlayVideo(media.clone()));
            },
        }
    }

    fn init(
        media: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            media: media.clone(),
            thumbnail: OnceCell::new(),
        };

        let widgets = view_output!();

        let thumbnail = EpisodeThumbnail::builder().launch(media.clone()).detach();
        root.add_prefix(thumbnail.widget());

        model.thumbnail.set(thumbnail).unwrap();

        ComponentParts { model, widgets }
    }
}

#[derive(Debug)]
struct EpisodeThumbnail {
    thumbnail: Option<Texture>,
}
#[derive(Debug)]
enum EpisodeThumbnailCommandOutput {
    LoadThumbnail(VecDeque<u8>),
}

#[relm4::component]
impl Component for EpisodeThumbnail {
    type Init = Media;
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
                    set_width_request: EPISODE_THUMBNAIL_SIZE,
                    set_height_request: EPISODE_THUMBNAIL_SIZE,
                    set_content_fit: gtk::ContentFit::Cover,
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

    fn init(
        media: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let img_url = media.image_tags.primary.clone();

        sender.oneshot_command(async {
            let img_bytes: VecDeque<u8> = reqwest::get(img_url)
                .await
                .expect("Error getting media tile image: {img_url}")
                .bytes()
                .await
                .expect("Error getting media tile image bytes: {img_url}")
                .into_iter()
                .collect();
            EpisodeThumbnailCommandOutput::LoadThumbnail(img_bytes)
        });

        let model = Self { thumbnail: None };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
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
                    EPISODE_THUMBNAIL_SIZE,
                    EPISODE_THUMBNAIL_SIZE,
                )
                .unwrap();

                let offset_x = ((pixbuf.width() - EPISODE_THUMBNAIL_SIZE) / 2).abs() as f64;

                pixbuf.scale(
                    &resized,
                    0,
                    0,
                    EPISODE_THUMBNAIL_SIZE,
                    EPISODE_THUMBNAIL_SIZE,
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
