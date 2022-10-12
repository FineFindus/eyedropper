use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

mod imp {
    use std::cell::RefCell;

    use super::*;

    use glib::{subclass::Signal, ParamSpecBoolean, ParamSpecString};
    use gtk::glib::ParamSpec;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/color-format-row.ui")]
    pub struct ColorFormatRow {
        #[template_child]
        pub entry: TemplateChild<gtk::Entry>,
        pub color: RefCell<String>,
        pub editable: RefCell<bool>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatRow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ColorFormatRow";
        type ParentType = gtk::Box;
        type Type = super::ColorFormatRow;

        fn new() -> Self {
            Self {
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

    // Trait shared by all GObjects
    impl ObjectImpl for ColorFormatRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder(
                        "copied-text",
                        &[String::static_type().into()],
                        <()>::static_type().into(),
                    )
                    .build(),
                    Signal::builder(
                        "text-edited",
                        &[String::static_type().into()],
                        <()>::static_type().into(),
                    )
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

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
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

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "text" => self.color.borrow().to_value(),
                "editable" => self.editable.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
            obj.setup_properties();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ColorFormatRow {}

    // Trait shared by all boxes
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
        glib::Object::new::<Self>(&[]).expect("Failed to create a ColorFormatRow")
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
                format_row.emit_by_name("text-edited", &[&text.to_value()])
            }));
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
