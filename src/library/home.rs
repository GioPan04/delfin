#![allow(unused_imports)]
use std::any::Any;
use std::sync::Arc;
use std::unimplemented;

use gtk::prelude::*;
use relm4::{
    gtk, prelude::*, Component, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::jellyfin_api::api::views::UserView;
use crate::jellyfin_api::api_client::ApiClient;
use crate::jellyfin_api::models::display_preferences::{DisplayPreferences, HomeSection};
use crate::utils::constants::MAX_LIBRARY_WIDTH;

use super::home_sections::continue_watching::HomeSectionContinueWatching;
use super::home_sections::latest::HomeSectionLatest;
use super::home_sections::next_up::HomeSectionNextUp;

enum HomeSectionController {
    ContinueWatching(Controller<HomeSectionContinueWatching>),
    Latest(Controller<HomeSectionLatest>),
    NextUp(Controller<HomeSectionNextUp>),
}

pub struct Home {
    _sections: Vec<HomeSectionController>,
}

#[derive(Debug)]
pub enum HomeInput {}

#[derive(Debug)]
pub enum HomeOutput {}

pub struct HomeInit {
    pub api_client: Arc<ApiClient>,
    pub display_preferences: DisplayPreferences,
    pub user_views: Vec<UserView>,
}

#[relm4::component(pub)]
impl SimpleComponent for Home {
    type Input = HomeInput;
    type Output = HomeOutput;
    type Init = HomeInit;

    view! {
        adw::Clamp {
            set_maximum_size: MAX_LIBRARY_WIDTH,
            set_tightening_threshold: 1000,

            #[name = "sections_container"]
            gtk::Box {
                set_valign: gtk::Align::Start,
                set_vexpand: true,
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 20,
            }
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Home { _sections: vec![] };

        let widgets = view_output!();
        let sections_container = &widgets.sections_container;

        model.display_sections(
            sections_container,
            init.display_preferences,
            init.api_client,
            init.user_views,
        );

        ComponentParts { model, widgets }
    }
}

impl Home {
    fn display_sections(
        &mut self,
        sections_container: &gtk::Box,
        display_preferences: DisplayPreferences,
        api_client: Arc<ApiClient>,
        user_views: Vec<UserView>,
    ) {
        for section in display_preferences.home_sections {
            match section {
                HomeSection::ContinueWatching => {
                    let section = HomeSectionContinueWatching::builder()
                        .launch(api_client.clone())
                        .detach();
                    sections_container.append(section.widget());
                    self._sections
                        .push(HomeSectionController::ContinueWatching(section));
                }
                HomeSection::LatestMedia => {
                    let section = HomeSectionLatest::builder()
                        .launch((api_client.clone(), user_views.clone()))
                        .detach();
                    sections_container.append(section.widget());
                    self._sections.push(HomeSectionController::Latest(section));
                }
                HomeSection::NextUp => {
                    let section = HomeSectionNextUp::builder()
                        .launch(api_client.clone())
                        .detach();
                    sections_container.append(section.widget());
                    self._sections.push(HomeSectionController::NextUp(section));
                }
                _ => {}
            }
        }
    }
}
