use glib::subclass::prelude::*;
use glib::{Object, ObjectExt};

mod imp {

    use std::cell::{Cell, RefCell};

    use super::*;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        Properties,
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
        pub visible: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatObject {
        const NAME: &'static str = "ColorFormatObject";
        type Type = super::ColorFormatObject;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ColorFormatObject {}
}

glib::wrapper! {
    pub struct ColorFormatObject(ObjectSubclass<imp::ColorFormatObject>);
}

impl ColorFormatObject {
    pub fn new(identifier: String, label: &str, format: String) -> Self {
        Object::builder()
            .property("identifier", &identifier)
            .property("label", &label)
            .property("example", &format)
            .build()
    }
}
