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

use super::home_sections::latest::HomeSectionLatest;

pub struct Home {}

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
        let model = Home {};

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
        &self,
        root: &gtk::Box,
        display_preferences: DisplayPreferences,
        api_client: Arc<ApiClient>,
        user_views: UserViews,
    ) {
        for section in display_preferences.home_sections {
            if let HomeSection::LatestMedia = section {
                let section = HomeSectionLatest::builder()
                    .launch((api_client.clone(), user_views.clone()))
                    .detach();
                root.append(section.widget());
            }
        }
    }
}
