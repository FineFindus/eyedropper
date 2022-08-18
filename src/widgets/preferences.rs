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
        #[template_child(id = "dark_mode_switch")]
        pub dark_mode_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub dark_mode_group: TemplateChild<adw::PreferencesGroup>,
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
                dark_mode_switch: TemplateChild::default(),
                dark_mode_group: TemplateChild::default(),
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
            obj.setup_widgets();
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
        // self.bind_property("color-value", &self.imp().scale.get().adjustment(), "value")
        //     //transform_to is not working, no idea why
        //     .transform_from(move |_, val| {
        //         //scale value to color value
        //         let value = val.get::<f64>().unwrap() as u32;
        //         Some(value.to_value())
        //     })
        //     .flags(glib::BindingFlags::SYNC_CREATE | glib::BindingFlags::BIDIRECTIONAL)
        //     .build();
    }

    fn setup_widgets(&self) {
        let imp = self.imp();

        let style_manager = adw::StyleManager::default();
        log::info!(
            "System Supports Color Schemes: {}",
            style_manager.system_supports_color_schemes()
        );
        imp.dark_mode_group
            .set_visible(style_manager.system_supports_color_schemes());

        imp.settings
            .bind("dark-theme", &*imp.dark_mode_switch, "active")
            .build();
    }
}
