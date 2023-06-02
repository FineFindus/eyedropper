use glib::{subclass::types::ObjectSubclassIsExt, Object};

use crate::colors::color::Color;

mod imp {
    use std::cell::RefCell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, ParamSpecBoxed, ToValue, Value,
    };
    use gtk::gdk;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(Debug)]
    pub struct HistoryObject {
        pub color: RefCell<gdk::RGBA>,
    }

    impl Default for HistoryObject {
        fn default() -> Self {
            Self {
                color: RefCell::new(gdk::RGBA::BLACK),
            }
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for HistoryObject {
        const NAME: &'static str = "HistoryObject";
        type Type = super::HistoryObject;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for HistoryObject {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> =
                Lazy::new(|| vec![ParamSpecBoxed::builder::<gtk::gdk::RGBA>("color").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "color" => {
                    let input_value = value.get::<gdk::RGBA>().unwrap();
                    self.color.replace(input_value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "color" => self.color.borrow().to_value(),
                _ => unimplemented!(),
            }
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

    pub fn color(&self) -> Color {
        Color::from(*self.imp().color.borrow())
    }
}
