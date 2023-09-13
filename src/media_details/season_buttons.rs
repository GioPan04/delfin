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
        let first_btn = gtk::ToggleButton::builder()
            .label(seasons[0].name.clone())
            .active(true)
            .build();
        first_btn.connect_toggled(btn_toggle_handler(0));
        seasons_box.append(&first_btn);

        for (index, season) in seasons.iter().enumerate().skip(1) {
            let season_btn = gtk::ToggleButton::builder()
                .label(&season.name)
                .group(&first_btn)
                .build();
            season_btn.connect_toggled(btn_toggle_handler(index));
            seasons_box.append(&season_btn);
        }

        ComponentParts { model, widgets }
    }
}
