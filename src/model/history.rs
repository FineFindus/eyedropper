use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Object;

use crate::colors::color::Color;

mod imp {
    use std::cell::Cell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        Properties,
    };
    use gtk::gdk;

    use super::*;

    #[derive(Debug, Properties)]
    #[properties(wrapper_type = super::HistoryObject)]
    pub struct HistoryObject {
        #[property(get, set)]
        pub color: Cell<gdk::RGBA>,
    }

    impl Default for HistoryObject {
        fn default() -> Self {
            Self {
                color: Cell::new(gdk::RGBA::BLACK),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HistoryObject {
        const NAME: &'static str = "HistoryObject";
        type Type = super::HistoryObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for HistoryObject {}
}

glib::wrapper! {
    pub struct HistoryObject(ObjectSubclass<imp::HistoryObject>);
}

impl HistoryObject {
    pub fn new(color: Color) -> Self {
        let color: gtk::gdk::RGBA = color.into();
        Object::builder().property("color", color).build()
    }
}
