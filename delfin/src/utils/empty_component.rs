use gtk::prelude::*;
use relm4::prelude::*;

pub struct EmptyComponent;

#[relm4::component(pub)]
impl SimpleComponent for EmptyComponent {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        gtk::Box {
            set_visible: false,
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = EmptyComponent;
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
}
