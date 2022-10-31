use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

mod imp {
    use std::cell::RefCell;

    use crate::config;

    use super::*;

    use glib::{subclass::Signal, ParamSpecBoolean, ParamSpecString};
    use gtk::glib::ParamSpec;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/color-format-row.ui")]
    pub struct ColorFormatRow {
        pub settings: gtk::gio::Settings,
        #[template_child]
        pub entry: TemplateChild<gtk::Entry>,
        pub color: RefCell<String>,
        pub editable: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatRow {
        const NAME: &'static str = "ColorFormatRow";
        type ParentType = gtk::Box;
        type Type = super::ColorFormatRow;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                entry: TemplateChild::default(),
                color: RefCell::new(String::new()),
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

    impl ObjectImpl for ColorFormatRow {
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
                    ParamSpecString::builder("text").build(),
                    ParamSpecBoolean::builder("editable")
                        .default_value(false)
                        .build(),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
            match pspec.name() {
                "text" => {
                    let input_value = value.get::<String>().unwrap();
                    self.color.replace(input_value);
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
                "text" => self.color.borrow().to_value(),
                "editable" => self.editable.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
            obj.setup_properties();
            obj.hide();
        }
    }

    impl WidgetImpl for ColorFormatRow {}
    impl BoxImpl for ColorFormatRow {}
}

glib::wrapper! {
    pub struct ColorFormatRow(ObjectSubclass<imp::ColorFormatRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ColorFormatRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[])
    }

    /// Get the currently shown text.
    pub fn text(&self) -> String {
        self.property("text")
    }

    /// Set the currently shown text
    pub fn set_text(&self, text: String) {
        self.set_property("text", &text);
    }

    fn setup_properties(&self) {
        //bind texts
        self.bind_property("text", &*self.imp().entry, "text")
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
        //bind editable
        self.bind_property("editable", &*self.imp().entry, "editable")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        self.bind_property("editable", &*self.imp().entry, "can-focus")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        self.bind_property("editable", &*self.imp().entry, "can-target")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
    }

    fn setup_signals(&self) {
        self.imp()
            .entry
            .connect_changed(glib::clone!(@weak self as format_row => move |entry| {
                let text = entry.buffer().text();
                if format_row.is_visible() && !text.is_empty() {
                    format_row.emit_by_name("text-edited", &[&text.to_value()])
                }
            }));
    }

    /// Set the settings name.
    /// This creates a binding between the widget visibility and the setting.
    pub fn set_settings_name(&self, settings_name: &str) {
        //it seems like it is not possible to bind to the name of a property in a widget so this does the same thing but less pretty
        self.imp()
            .settings
            .bind(settings_name, self, "visible")
            .build();
    }

    /// Callback when the copy button is pressed.
    #[template_callback]
    fn on_copy_pressed(&self, _: &gtk::Button) {
        self.copy_text();
    }

    /// Copy the current text to the users clipboard
    fn copy_text(&self) {
        let clipboard = self.clipboard();
        let text = self.imp().entry.text().to_string();
        log::debug!("Copied text: {text}");
        clipboard.set_text(&text);
        self.emit_by_name("copied-text", &[&text.to_value()])
    }
}
