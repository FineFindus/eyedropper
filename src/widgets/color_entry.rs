use std::str::FromStr;

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

    use gtk::glib::{ParamSpec, ParamSpecBoxed};
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/benzler/colors/ui/color-entry.ui")]
    pub struct ColorEntry {
        pub color: RefCell<gdk::RGBA>,
    }

    #[gtk::template_callbacks]
    impl ColorEntry {
        #[template_callback]
        fn on_icon_pressed(&self, pos: gtk::EntryIconPosition, _entry: &gtk::Entry) {
            if pos == gtk::EntryIconPosition::Secondary {
                self.instance().pick_color();
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
            // obj.set_width_chars(7);
            // obj.set_max_width_chars(7);
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
        log::debug!("Setting new color to {}", Color::from(new_color));
        self.set_property("color", &new_color);
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
                gdk::RGBA::from_str(&text).ok().map(|c| c.to_value())
            })
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
    }

    fn pick_color(&self) {
        log::debug!("Picking a color using the color picker");
        gtk_macros::spawn!(glib::clone!(@weak self as entry => async move {

        let connection = ashpd::zbus::Connection::session().await.expect("Failed to open connection to zbus");
        let proxy = ashpd::desktop::screenshot::ScreenshotProxy::new(&connection).await.expect("Failed to open screenshot proxy");
        match proxy.pick_color(&ashpd::WindowIdentifier::default()).await {
            Ok(color) => entry.set_color(gdk::RGBA::new(color.red() as f32, color.green() as f32, color.blue() as f32, 1f32)),
            Err(err) => log::error!("{}", err),
        };
        }));
    }
}
