use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    gdk, glib,
    prelude::{ObjectExt, ToValue},
};

use crate::model::Color;

mod imp {
    use super::*;
    use std::cell::RefCell;

    use gtk::glib::{subclass::Signal, ParamSpec, ParamSpecBoxed};
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/color-entry.ui")]
    pub struct ColorEntry {
        pub color: RefCell<gdk::RGBA>,
    }

    #[gtk::template_callbacks]
    impl ColorEntry {
        #[template_callback]
        fn on_icon_pressed(&self, pos: gtk::EntryIconPosition, _entry: &gtk::Entry) {
            if pos == gtk::EntryIconPosition::Secondary {
                self.instance().copy_color();
            }
        }
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ColorEntry {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ColorEntry";
        type ParentType = gtk::Entry;
        type Type = super::ColorEntry;

        fn new() -> Self {
            Self {
                color: RefCell::new(gdk::RGBA::BLUE),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            Self::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ColorEntry {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder(
                    // Signal name
                    "copied-color",
                    // Types of the values which will be sent to the signal handler
                    &[String::static_type().into()],
                    // Type of the value the signal handler sends back
                    <()>::static_type().into(),
                )
                .build()]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![ParamSpecBoxed::new(
                    "color",
                    "Color",
                    "Entry Color",
                    gdk::RGBA::static_type(),
                    glib::ParamFlags::READWRITE,
                )]
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
                "color" => {
                    let color = value.get::<Option<gdk::RGBA>>().unwrap().unwrap();
                    self.color.replace(color);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "color" => self.color.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
            obj.set_max_length(9);
            obj.set_width_chars(9);
            obj.set_max_width_chars(9);
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ColorEntry {}

    impl EntryImpl for ColorEntry {}
}

glib::wrapper! {
    pub struct ColorEntry(ObjectSubclass<imp::ColorEntry>)
    @extends gtk::Widget, gtk::Entry, @implements gtk::Editable;
}

impl ColorEntry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create a ColorEntry")
    }

    pub fn color(&self) -> gdk::RGBA {
        self.property("color")
    }

    pub fn set_color(&self, new_color: gdk::RGBA) {
        //only updated value if value has changed, this avoids a loop where everything thinks it changed
        if self.color() != new_color {
            self.set_property("color", &new_color);
        }
    }

    fn setup_signals(&self) {
        self.bind_property("color", self, "text")
            .transform_to(move |_, val| {
                let gdk_color: gdk::RGBA = val.get().unwrap();
                let hex_color = Color::from(gdk_color);
                Some(
                    hex_color
                        .to_hex_string(crate::model::AlphaPosition::End)
                        .to_value(),
                )
            })
            .transform_from(move |_, val| {
                let text: String = val.get().unwrap();
                match Color::from_hex(&text, crate::model::AlphaPosition::End) {
                    Ok(color) => Some(gdk::RGBA::from(color.into()).to_value()),
                    Err(_) => None,
                }
            })
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
    }

    fn copy_color(&self) {
        log::debug!("Coping selected color");
        let clipboard = self.clipboard();
        let color = self.text().to_string();
        clipboard.set_text(&color);
        self.emit_by_name("copied-color", &[&color.to_value()])
    }
}
