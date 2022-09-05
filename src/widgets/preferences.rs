use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

mod imp {

    use adw::subclass::{prelude::PreferencesWindowImpl, window::AdwWindowImpl};

    use crate::config;

    use super::*;

    // Object holding the state
    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences.ui")]
    pub struct PreferencesWindow {
        pub settings: gtk::gio::Settings,
        #[template_child()]
        pub alpha_pos_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub hex_model_switch: TemplateChild<gtk::Switch>,
        #[template_child()]
        pub rgb_model_switch: TemplateChild<gtk::Switch>,
        #[template_child()]
        pub hsl_model_switch: TemplateChild<gtk::Switch>,
        #[template_child()]
        pub hsv_model_switch: TemplateChild<gtk::Switch>,
        #[template_child()]
        pub cmyk_model_switch: TemplateChild<gtk::Switch>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                alpha_pos_box: TemplateChild::default(),
                hex_model_switch: TemplateChild::default(),
                rgb_model_switch: TemplateChild::default(),
                hsl_model_switch: TemplateChild::default(),
                hsv_model_switch: TemplateChild::default(),
                cmyk_model_switch: TemplateChild::default(),
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
    impl ObjectImpl for PreferencesWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_signals();
        }
    }

    impl WidgetImpl for PreferencesWindow {}
    impl WindowImpl for PreferencesWindow {}
    impl AdwWindowImpl for PreferencesWindow {}
    impl PreferencesWindowImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
    @extends gtk::Widget, gtk::Window, adw::Window, adw::PreferencesWindow;
}

impl PreferencesWindow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create a PreferencesWindow")
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        imp.settings
            .bind("alpha-position", &*imp.alpha_pos_box, "selected")
            .build();

        imp.settings
            .bind("show-hex-model", &*imp.hex_model_switch, "state")
            .build();

        imp.settings
            .bind("show-rgb-model", &*imp.rgb_model_switch, "state")
            .build();

        imp.settings
            .bind("show-hsl-model", &*imp.hsl_model_switch, "state")
            .build();

        imp.settings
            .bind("show-hsv-model", &*imp.hsv_model_switch, "state")
            .build();

        imp.settings
            .bind("show-cmyk-model", &*imp.cmyk_model_switch, "state")
            .build();
    }
}
