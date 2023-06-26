#![allow(unused_imports)]
use std::any::Any;
use std::sync::Arc;
use std::unimplemented;

use gtk::prelude::*;
use relm4::{
    gtk, prelude::*, Component, ComponentParts, ComponentSender, Controller, SimpleComponent,
};

use crate::jellyfin_api::api::views::UserViews;
use crate::jellyfin_api::api_client::ApiClient;
use crate::jellyfin_api::models::display_preferences::{DisplayPreferences, HomeSection};
use crate::jellyfin_api::models::media::Media;

use super::home_sections::continue_watching::HomeSectionContinueWatching;
use super::home_sections::latest::HomeSectionLatest;

enum HomeSectionController {
    ContinueWatching(Controller<HomeSectionContinueWatching>),
    Latest(Controller<HomeSectionLatest>),
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
    pub user_views: UserViews,
}

#[relm4::component(pub)]
impl SimpleComponent for Home {
    type Input = HomeInput;
    type Output = HomeOutput;
    type Init = HomeInit;

    view! {
        gtk::Box {
            set_valign: gtk::Align::Start,
            set_vexpand: true,
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 20,
        }
    }

    fn init(
        init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let mut model = Home { _sections: vec![] };

        let widgets = view_output!();

        model.display_sections(
            root,
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
        root: &gtk::Box,
        display_preferences: DisplayPreferences,
        api_client: Arc<ApiClient>,
        user_views: UserViews,
    ) {
        for section in display_preferences.home_sections {
            match section {
                HomeSection::ContinueWatching => {
                    let section = HomeSectionContinueWatching::builder()
                        .launch(api_client.clone())
                        .detach();
                    root.append(section.widget());
                    self._sections
                        .push(HomeSectionController::ContinueWatching(section));
                }
                HomeSection::LatestMedia => {
                    let section = HomeSectionLatest::builder()
                        .launch((api_client.clone(), user_views.clone()))
                        .detach();
                    root.append(section.widget());
                    self._sections.push(HomeSectionController::Latest(section));
                }
                _ => {}
            }
        }
    }
}
