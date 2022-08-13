use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

mod imp {
    use std::cell::RefCell;

    use super::*;

    use gtk::glib::ParamSpec;
    use once_cell::sync::Lazy;

    // Object holding the state
    #[derive(gtk::CompositeTemplate)]
    #[template(resource = "/com/benzler/colors/ui/color-scale.ui")]
    pub struct ColorScale {
        #[template_child]
        pub scale: TemplateChild<gtk::Scale>,
        pub color_value: RefCell<u32>, //this is technically a u32, but handle it as an u8
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for ColorScale {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "ColorScale";
        type ParentType = gtk::Box;
        type Type = super::ColorScale;

        fn new() -> Self {
            Self {
                scale: TemplateChild::default(),
                color_value: RefCell::new(0),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for ColorScale {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![glib::ParamSpecUInt::new(
                    "color-value",
                    "Color Value",
                    "Color Value on the scale",
                    0,
                    255,
                    0,
                    glib::ParamFlags::READWRITE | glib::ParamFlags::EXPLICIT_NOTIFY,
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
                "color-value" => {
                    self.color_value.replace(value.get().unwrap());
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "color-value" => self.color_value.borrow_mut().to_value(),
                _ => unimplemented!(),
            }
        }

        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
        }
    }

    // Trait shared by all widgets
    impl WidgetImpl for ColorScale {}

    // Trait shared by all boxes
    impl BoxImpl for ColorScale {}
}

glib::wrapper! {
    pub struct ColorScale(ObjectSubclass<imp::ColorScale>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl ColorScale {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create a ColorScale")
    }

    pub fn color_value(&self) -> u8 {
        self.property::<u32>("color-value") as u8
    }

    pub fn set_color_value(&self, new_value: u8) {
        //only updated value if value has changed, this avoids a loop where everything thinks it changed
        if self.color_value() as u32 != new_value as u32 {
            self.set_property("color-value", &(new_value as u32));
            //manually update scale value, since property binding doesn't work
            self.imp().scale.set_value(new_value as f64);
        }
    }

    fn setup_signals(&self) {
        self.bind_property("color-value", &self.imp().scale.get().adjustment(), "value")
            //transform_to is not working, no idea why
            .transform_from(move |_, val| {
                //scale value to color value
                let value = val.get::<f64>().unwrap() as u32;
                Some(value.to_value())
            })
            .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
            .build();
    }
}
