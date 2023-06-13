use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::Object;

use crate::colors::color::Color;

mod imp {
    use std::cell::Cell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, Properties, Value,
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

    impl ObjectImpl for HistoryObject {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
            Self::derived_set_property(self, _id, _value, _pspec)
        }
        fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
            Self::derived_property(self, _id, _pspec)
        }
    }
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
