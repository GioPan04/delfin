use gtk::prelude::*;
use relm4::prelude::*;

use crate::jellyfin_api::api::shows::Season;

use super::seasons::SeasonsInput;

pub(crate) struct SeasonButtons;

#[relm4::component(pub(crate))]
impl SimpleComponent for SeasonButtons {
    type Init = Vec<Season>;
    type Input = ();
    type Output = SeasonsInput;

    view! {
        gtk::FlowBox {
            add_css_class: "seasons-box",
            set_homogeneous: true,
            set_halign: gtk::Align::Fill,
        }
    }

    fn init(
        seasons: Self::Init,
        seasons_box: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = SeasonButtons;

        let widgets = view_output!();

        let btn_toggle_handler = |index: usize| {
            let sender = sender.clone();
            move |btn: &gtk::ToggleButton| {
                if btn.is_active() {
                    sender.output(SeasonsInput::SeasonActivated(index)).unwrap();
                }
            }
        };

        // First button will be active and is used to group remaining buttons
        let first_btn = create_season_btn(&seasons[0]);
        first_btn.set_active(true);
        first_btn.connect_toggled(btn_toggle_handler(0));
        seasons_box.append(&first_btn);

        for (index, season) in seasons.iter().enumerate().skip(1) {
            let season_btn = create_season_btn(season);
            season_btn.set_group(Some(&first_btn));
            season_btn.connect_toggled(btn_toggle_handler(index));
            seasons_box.append(&season_btn);
        }

        ComponentParts { model, widgets }
    }
}

fn create_season_btn(season: &Season) -> gtk::ToggleButton {
    let btn_contents = gtk::Overlay::new();

    btn_contents.set_child(Some(
        &gtk::Label::builder()
            .label(&season.name)
            .hexpand(true)
            .build(),
    ));

    if !season.user_data.played {
        btn_contents.add_overlay(
            &gtk::Box::builder()
                .css_classes(["season-unplayed-indicator"])
                .width_request(8)
                .height_request(8)
                .halign(gtk::Align::End)
                .valign(gtk::Align::Start)
                .tooltip_text(
                    if let Some(unplayed_item_count) = season.user_data.unplayed_item_count {
                        format!(
                            "This season has {unplayed_item_count} unplayed episode{}",
                            if unplayed_item_count > 1 { "s" } else { "" }
                        )
                    } else {
                        "This season has unplayed episodes".to_string()
                    },
                )
                .build(),
        );
    }

    gtk::ToggleButton::builder().child(&btn_contents).build()
}
