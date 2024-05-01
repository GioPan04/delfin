use gtk::{glib, prelude::*, subclass::prelude::*};
use once_cell::sync::Lazy;
use relm4::{gtk, once_cell};

mod imp {
    use std::cell::{Cell, RefCell};

    use glib::Properties;
    use relm4::{adw, gtk::glib::subclass::Signal};

    use super::*;

    static SIGNAL_SEARCH: &str = "search";

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::SearchBar)]
    pub struct SearchBar {
        pub search_bar: gtk::SearchBar,
        pub search_entry: gtk::SearchEntry,
        #[property(get, set)]
        searching: Cell<bool>,
        #[property(get, set)]
        text: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SearchBar {
        const NAME: &'static str = "SearchBar";
        type Type = super::SearchBar;
        type ParentType = gtk::Widget;

        fn class_init(class: &mut Self::Class) {
            class.set_layout_manager_type::<gtk::BoxLayout>();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SearchBar {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            let search_bar = &self.search_bar;
            let search_entry = &self.search_entry;

            search_bar.set_parent(&*obj);

            obj.bind_property("searching", search_bar, "search-mode-enabled")
                .sync_create()
                .bidirectional()
                .build();

            obj.bind_property("text", search_entry, "text")
                .sync_create()
                .bidirectional()
                .build();

            relm4::view! {
                #[local_ref]
                search_bar -> gtk::SearchBar {
                    set_hexpand: true,

                    connect_entry: search_entry,

                    #[wrap(Some)]
                    set_child = &adw::Clamp {
                        #[local_ref]
                        search_entry -> gtk::SearchEntry {
                            set_search_delay: 300,
                            connect_search_changed[obj] => move |search_entry| {
                                obj.emit_by_name::<()>(SIGNAL_SEARCH, &[&search_entry.text().to_string()]);
                            },
                        },
                    },
                }
            }
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder(SIGNAL_SEARCH)
                    .param_types([String::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl WidgetImpl for SearchBar {}
}

glib::wrapper! {
    pub struct SearchBar(ObjectSubclass<imp::SearchBar>)
        @extends gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

impl SearchBar {
    pub fn new() -> Self {
        glib::Object::new()
    }

    pub fn set_key_capture_widget(&self, widget: Option<&impl IsA<gtk::Widget>>) {
        self.imp().search_bar.set_key_capture_widget(widget);
    }
}
