use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {
    use std::cell::RefCell;

    use crate::config;

    use super::*;

    use adw::subclass::prelude::{EntryRowImpl, PreferencesRowImpl};
    use glib::{subclass::Signal, ParamSpec, Properties, Value};
    use once_cell::sync::Lazy;

    #[derive(gtk::CompositeTemplate, Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences/custom-format-row.ui")]
    #[properties(wrapper_type = super::CustomFormatRow)]
    pub struct CustomFormatRow {
        pub settings: gtk::gio::Settings,
        #[property(get, set)]
        pub settings_key: RefCell<String>,
        #[property(get, set)]
        pub default_format: RefCell<String>,
        #[property(get, set, default = false)]
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
            Self::derived_properties()
        }
        fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
            Self::derived_set_property(self, _id, _value, _pspec)
        }
        fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
            Self::derived_property(self, _id, _pspec)
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
        glib::Object::new::<Self>()
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
