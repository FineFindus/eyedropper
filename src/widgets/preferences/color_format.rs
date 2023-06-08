use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::{Object, ObjectExt};

mod imp {

    use std::cell::RefCell;

    use super::*;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, Properties, Value,
    };

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::ColorFormatObject)]
    pub struct ColorFormatObject {
        #[property(get, set)]
        pub identifier: RefCell<String>,
        #[property(get, set)]
        pub label: RefCell<String>,
        #[property(get, set)]
        pub example: RefCell<String>,
        #[property(get, set, default = false)]
        pub visible: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatObject {
        const NAME: &'static str = "ColorFormatObject";
        type Type = super::ColorFormatObject;
    }

    impl ObjectImpl for ColorFormatObject {
        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }
}

glib::wrapper! {
    pub struct ColorFormatObject(ObjectSubclass<imp::ColorFormatObject>);
}

impl ColorFormatObject {
    pub fn new(identifier: String, label: String, format: String) -> Self {
        Object::builder()
            .property("identifier", &identifier)
            .property("label", &label)
            .property("example", &format)
            .build()
    }
}
