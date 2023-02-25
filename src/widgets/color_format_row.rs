use std::time::Duration;

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

    use glib::{subclass::Signal, ParamSpec, Value};
    use once_cell::sync::Lazy;

    #[derive(gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/color-format-row.ui")]
    #[properties(wrapper_type = super::ColorFormatRow)]
    pub struct ColorFormatRow {
        pub settings: gtk::gio::Settings,
        #[template_child]
        pub entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub format_button: TemplateChild<gtk::Button>,
        #[property(set, get)]
        pub color: RefCell<String>,
        #[property(set, get)]
        pub tooltip: RefCell<String>,
        #[property(set, get, default = false)]
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
                format_button: TemplateChild::default(),
                tooltip: RefCell::new(String::new()),
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
        glib::Object::new::<Self>()
    }

    /// Get the currently shown text.
    pub fn text(&self) -> String {
        self.property("color")
    }

    /// Set the currently shown text
    pub fn set_text(&self, text: String) {
        self.set_property("color", &text);
    }

    /// Indicate an error with the input occurred by applying the libadwaita error style class
    /// for a short time (250ms), so the entry shows the error for a short moment.
    pub fn show_error(&self) {
        let main_context = glib::MainContext::default();
        main_context.spawn_local(glib::clone!(@weak self as widget => async move {
            widget.add_css_class("error");
            glib::timeout_future_with_priority(glib::PRIORITY_DEFAULT, Duration::from_millis(500)).await;
            widget.remove_css_class("error");
        }));
    }

    /// Indicate success with the input.
    ///
    /// To visualize the success, the `success` libadwaita style class
    /// is applied for a short time (250ms).
    pub fn show_success(&self) {
        let main_context = glib::MainContext::default();
        main_context.spawn_local(glib::clone!(@weak self as widget => async move {
            widget.add_css_class("success");
            glib::timeout_future_with_priority(glib::PRIORITY_DEFAULT, Duration::from_millis(350)).await;
            widget.remove_css_class("success");
        }));
    }

    /// Bind the properties to the target values.
    ///
    /// Binds the `text` properties to the text of the entry, and
    /// the `editable` property to different properties
    /// of the entry to make it (un)-editable
    fn setup_properties(&self) {
        self.bind_property("tooltip", &*self.imp().format_button, "tooltip-text")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        //bind texts
        self.bind_property("color", &*self.imp().entry, "text")
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

    /// Registers a signal for when the text entry is changed to emit
    /// a signal containing the edited text.
    fn setup_signals(&self) {
        self.imp()
            .entry
            .connect_activate(glib::clone!(@weak self as format_row => move |entry| {
                let text = entry.buffer().text();
                if format_row.is_visible() && !text.is_empty() {
                    format_row.emit_by_name("text-edited", &[&text.to_value()])
                }
            }));
    }

    /// Set the settings name.
    ///
    /// This creates a binding between the widget visibility and the setting.
    pub fn set_settings_name(&self, settings_name: &str) {
        //it seems like it is not possible to bind to the name of a property in a widget so this does the same thing but less pretty
        self.imp()
            .settings
            .bind(settings_name, self, "visible")
            .build();
    }

    /// Copy the text to the users clipboard and a signal.
    ///
    /// This is bound as the callback when the copy-icon-button is pressed.
    #[template_callback]
    fn on_copy_pressed(&self, _: &gtk::Button) {
        let text = self.imp().entry.text().to_string();
        log::debug!("Copied text: {text}");
        let clipboard = self.clipboard();
        clipboard.set_text(&text);
        self.emit_by_name("copied-text", &[&text.to_value()])
    }
}
