use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ToValue};

mod imp {
    use std::cell::RefCell;

    use crate::config;

    use super::*;

    use adw::subclass::prelude::{EntryRowImpl, PreferencesRowImpl};
    use glib::{subclass::Signal, ParamSpecBoolean, ParamSpecString};
    use gtk::glib::ParamSpec;
    use once_cell::sync::Lazy;

    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences/custom-format-row.ui")]
    pub struct CustomFormatRow {
        pub settings: gtk::gio::Settings,
        pub settings_key: RefCell<String>,
        pub default_format: RefCell<String>,
        pub editable: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for CustomFormatRow {
        const NAME: &'static str = "CustomFormatRow";
        type ParentType = adw::EntryRow;
        type Type = super::CustomFormatRow;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                settings_key: RefCell::new(String::new()),
                default_format: RefCell::new(String::new()),
                editable: RefCell::new(false),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CustomFormatRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("copied-text")
                        .param_types([String::static_type()])
                        .build(),
                    Signal::builder("text-edited")
                        .param_types([String::static_type()])
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::builder("settings-key").build(),
                    ParamSpecString::builder("default-format").build(),
                    ParamSpecBoolean::builder("editable")
                        .default_value(false)
                        .build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
            match pspec.name() {
                "settings-key" => {
                    let input_value = value.get::<String>().unwrap();
                    self.settings_key.replace(input_value);
                }
                "default-format" => {
                    let input_value = value.get::<String>().unwrap();
                    self.default_format.replace(input_value);
                }
                "editable" => {
                    let input_value = value.get::<bool>().unwrap();
                    self.editable.replace(input_value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "settings-key" => self.settings_key.borrow().to_value(),
                "default-format" => self.default_format.borrow().to_value(),
                "editable" => self.editable.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_properties();
        }
    }

    impl WidgetImpl for CustomFormatRow {}
    impl EntryRowImpl for CustomFormatRow {}
    impl PreferencesRowImpl for CustomFormatRow {}
    impl ListBoxRowImpl for CustomFormatRow {}
}

glib::wrapper! {
    pub struct CustomFormatRow(ObjectSubclass<imp::CustomFormatRow>)
    @extends gtk::Widget, adw::EntryRow;
}

#[gtk::template_callbacks]
impl CustomFormatRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[])
    }

    /// Get the name of the format settings.
    /// Might returns `None` if the widget was just initialized on the property was not
    fn settings_key(&self) -> String {
        self.property("settings-key")
    }

    /// Returns the `default-format`.
    fn default_format(&self) -> String {
        self.property::<String>("default-format")
    }

    /// Set the visible text.
    fn set_text(&self, text: String) {
        self.set_property("text", text);
    }

    /// Reset order when the reset button is pressed.
    #[template_callback]
    fn on_reset_pressed(&self, _: &gtk::Button) {
        log::debug!("Reset custom format");
        self.imp().settings.reset(&self.settings_key());
        self.set_text(self.default_format());
    }

    /// Saves the custom format to the settings.
    /// Called when the apply button is pressed.
    #[template_callback]
    fn on_apply(&self) {
        log::debug!("Activate");
        let text = self.property::<String>("text");
        log::debug!("Text: {}", text);
        if text.trim() != self.property::<String>("default-format") {
            //save format
            match self.imp().settings.set_string(&self.settings_key(), &text) {
                Ok(_) => {}
                Err(err) => log::error!("Failed to set setting {}: {}", self.settings_key(), err),
            }
        } else {
            log::debug!("Format is the same, not updating")
        }
    }

    /// Bind the properties to the target values.
    /// This sets the text to the default value.
    fn setup_properties(&self) {
        self.connect_notify(Some("settings-key"), |widget, _| {
            let custom_format = widget
                .imp()
                .settings
                .string(&widget.settings_key())
                .to_string();
            if custom_format.is_empty() {
                widget.set_text(widget.default_format());
            } else {
                widget.set_text(custom_format);
            }
            log::debug!("Text: {}", widget.property::<String>("text"))
        });
    }
}
