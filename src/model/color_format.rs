use glib::{Object, ObjectExt};

mod imp {

    use std::cell::RefCell;

    use glib::{
        subclass::{prelude::ObjectImpl, types::ObjectSubclass},
        ParamSpec, ParamSpecString, ToValue, Value,
    };
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(Debug, Default)]
    pub struct ColorFormatObject {
        pub identifier: RefCell<String>,
        pub label: RefCell<String>,
        pub example: RefCell<String>,
        pub settings_name: RefCell<String>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatObject {
        const NAME: &'static str = "ColorFormatObject";
        type Type = super::ColorFormatObject;
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ColorFormatObject {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("identifier").build(),
                    ParamSpecString::builder("label").build(),
                    ParamSpecString::builder("example").build(),
                    ParamSpecString::builder("settings-name").build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "identifier" => {
                    let input_value = value.get::<String>().unwrap();
                    self.identifier.replace(input_value);
                }
                "label" => {
                    let input_value = value.get::<String>().unwrap();
                    self.label.replace(input_value);
                }
                "example" => {
                    let input_value = value.get::<String>().unwrap();
                    self.example.replace(input_value);
                }
                "settings-name" => {
                    let input_value = value.get::<String>().unwrap();
                    self.settings_name.replace(input_value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "identifier" => self.identifier.borrow().to_value(),
                "label" => self.label.borrow().to_value(),
                "example" => self.example.borrow().to_value(),
                "settings-name" => self.settings_name.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct ColorFormatObject(ObjectSubclass<imp::ColorFormatObject>);
}

impl ColorFormatObject {
    pub fn new(identifier: String, label: String, format: String, settings_name: &str) -> Self {
        Object::new(&[
            ("identifier", &identifier),
            ("label", &label),
            ("example", &format),
            ("settings-name", &settings_name),
        ])
        .expect("Failed to create `ColorFormatObject`.")
    }

    pub fn identifier(&self) -> String {
        self.property("identifier")
    }

    pub fn label(&self) -> String {
        self.property("label")
    }

    pub fn example(&self) -> String {
        self.property("example")
    }

    pub fn settings_name(&self) -> String {
        self.property("settings_name")
    }
}
