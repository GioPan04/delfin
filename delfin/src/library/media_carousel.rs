use std::sync::Arc;

use adw::prelude::*;
use gtk::glib;
use jellyfin_api::types::BaseItemDto;
use relm4::{
    component::{AsyncComponent, AsyncComponentController, AsyncController},
    prelude::*,
    ComponentParts,
};

use crate::{globals::SHIFT_STATE, jellyfin_api::api_client::ApiClient};

use super::media_tile::{MediaTile, MediaTileDisplay};

const MIN_PADDING: i32 = 16;

pub(crate) struct MediaCarousel {
    media_tiles: Vec<AsyncController<MediaTile>>,
    pages: Vec<gtk::Box>,
}

pub(crate) struct MediaCarouselInit {
    pub(crate) media: Vec<BaseItemDto>,
    pub(crate) media_tile_display: MediaTileDisplay,
    pub(crate) api_client: Arc<ApiClient>,
    pub(crate) label: String,
}

#[derive(Debug)]
pub(crate) enum MediaCarouselInput {
    Resize(i32),
    Left,
    Right,
}

impl MediaTileDisplay {
    fn min_height(&self) -> i32 {
        self.height() + 80
    }
}

#[relm4::component(pub(crate))]
impl Component for MediaCarousel {
    type Init = MediaCarouselInit;
    type Input = MediaCarouselInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 16,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                #[name = "title"]
                gtk::Label {
                    set_label: label.as_str(),
                    add_css_class: "title-2",
                    set_halign: gtk::Align::Start,
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_halign: gtk::Align::End,
                    set_hexpand: true,
                    add_css_class: "linked",
                    #[watch]
                    set_visible: carousel.n_pages() > 1,

                    gtk::Button {
                        set_icon_name: "left",
                        add_css_class: "flat",
                        connect_clicked[sender] => move |_| {
                            sender.input(MediaCarouselInput::Left);
                        },
                    },

                    gtk::Button {
                        set_icon_name: "right",
                        add_css_class: "flat",
                        connect_clicked[sender] => move |_| {
                            sender.input(MediaCarouselInput::Right);
                        },
                    },
                },
            },

            #[name = "breakpoints"]
            adw::BreakpointBin {
                set_size_request: (
                    media_tile_display.width() * 2 + MIN_PADDING,
                    media_tile_display.min_height(),
                ),
                set_hexpand: true,

                #[wrap(Some)]
                set_child = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    #[name = "carousel"]
                    adw::Carousel {
                        set_allow_scroll_wheel: false,
                        set_focusable: true,

                        add_controller = gtk::EventControllerScroll {
                            // TODO: Might need a separate controller for Horizontal scrolling that doesn't check if shift is pressed
                            set_flags: gtk::EventControllerScrollFlags::VERTICAL,
                            connect_scroll[sender] => move |_, _dx, dy| {
                                let shift_state = SHIFT_STATE.read();
                                if shift_state.pressed() {
                                    sender.input(
                                        if dy > 0.0 {
                                            MediaCarouselInput::Right
                                        } else {
                                            MediaCarouselInput::Left
                                        }
                                    );
                                    return glib::Propagation::Stop;
                                }
                                glib::Propagation::Proceed
                            },
                        },
                    },
                    #[name = "carousel_indicator"]
                    adw::CarouselIndicatorLines {},
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let MediaCarouselInit {
            api_client,
            media,
            media_tile_display,
            label,
        } = init;

        let widgets = view_output!();
        let breakpoints = &widgets.breakpoints;
        let carousel = &widgets.carousel;
        let carousel_indicator = &widgets.carousel_indicator;

        carousel_indicator.set_carousel(Some(carousel));

        let media_tiles: Vec<AsyncController<MediaTile>> = media
            .iter()
            .map(|media| {
                MediaTile::builder()
                    .launch((media.clone(), media_tile_display, api_client.clone()))
                    .detach()
            })
            .collect();

        let model = MediaCarousel {
            media_tiles,
            pages: vec![],
        };

        add_breakpoints(breakpoints, &sender, media_tile_display);

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        let carousel: &adw::Carousel = &widgets.carousel;

        match message {
            MediaCarouselInput::Resize(tiles_per_page) => {
                // Remove existing pages
                for page in &self.pages {
                    while page.first_child().is_some() {
                        page.remove(&page.first_child().unwrap());
                    }
                    carousel.remove(page);
                }
                self.pages.clear();

                let media_tile_chunks: Vec<&[AsyncController<MediaTile>]> =
                    self.media_tiles.chunks(tiles_per_page as usize).collect();

                for chunk in media_tile_chunks {
                    let page = gtk::Box::builder()
                        .orientation(gtk::Orientation::Horizontal)
                        .homogeneous(true)
                        .hexpand(true)
                        .build();

                    // Not a full page, we don't want the tiles to be spaced out across the screen
                    if chunk.len() < tiles_per_page as usize {
                        page.set_halign(gtk::Align::Start);
                        page.set_spacing(MIN_PADDING);
                    }

                    for tile in chunk {
                        page.append(tile.widget());
                    }

                    carousel.append(&page);
                    self.pages.push(page);
                }

                self.update_view(widgets, sender);
            }
            MediaCarouselInput::Left => {
                let cur_pos = carousel.position() as i32;
                let mut pos = cur_pos - 1;
                if pos < 0 {
                    pos = carousel.n_pages() as i32 - 1;
                }

                let next_page = carousel.nth_page(pos as u32);
                carousel.scroll_to(&next_page, true);
            }
            MediaCarouselInput::Right => {
                let cur_pos = carousel.position() as i32;
                let mut pos = cur_pos + 1;
                if pos > (carousel.n_pages() as i32) - 1 {
                    pos = 0;
                }

                let next_page = carousel.nth_page(pos as u32);
                carousel.scroll_to(&next_page, true);
            }
        }
    }
}

fn add_breakpoints(
    breakpoints: &adw::BreakpointBin,
    sender: &ComponentSender<MediaCarousel>,
    media_tile_display: MediaTileDisplay,
) {
    for tiles_per_page in 2..=8 {
        let required =
            (media_tile_display.width() * tiles_per_page) + (MIN_PADDING * (tiles_per_page - 1));

        let breakpoint = adw::Breakpoint::new(adw::BreakpointCondition::new_length(
            adw::BreakpointConditionLengthType::MinWidth,
            required as f64,
            adw::LengthUnit::Px,
        ));
        breakpoint.connect_apply({
            let sender = sender.clone();
            move |_| {
                sender.input(MediaCarouselInput::Resize(tiles_per_page));
            }
        });

        breakpoints.add_breakpoint(breakpoint);
    }
}
