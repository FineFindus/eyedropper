use std::ops::Deref;

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::ExampleApplication;
use crate::config::{APP_ID, PROFILE};
use crate::model::Color;

mod imp {
    use std::cell::RefCell;

    use crate::widgets;

    use super::*;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/benzler/colors/ui/window.ui")]
    pub struct ExampleApplicationWindow {
        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub hex_entry: TemplateChild<widgets::color_entry::ColorEntry>,
        #[template_child]
        pub red_scale: TemplateChild<widgets::color_scale::ColorScale>,
        pub settings: gio::Settings,
        pub color: RefCell<Color>,
    }

    impl Default for ExampleApplicationWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                hex_entry: TemplateChild::default(),
                red_scale: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
                color: RefCell::new(Color::rgba(46, 52, 64, 255)),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ExampleApplicationWindow {
        const NAME: &'static str = "ExampleApplicationWindow";
        type Type = super::ExampleApplicationWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ExampleApplicationWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
            obj.setup_callbacks();
        }
    }

    impl WidgetImpl for ExampleApplicationWindow {}
    impl WindowImpl for ExampleApplicationWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for ExampleApplicationWindow {}
    impl AdwApplicationWindowImpl for ExampleApplicationWindow {}
}

glib::wrapper! {
    pub struct ExampleApplicationWindow(ObjectSubclass<imp::ExampleApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl ExampleApplicationWindow {
    pub fn new(app: &ExampleApplication) -> Self {
        glib::Object::new(&[("application", app)])
            .expect("Failed to create ExampleApplicationWindow")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();
        log::debug!("Window Size: {}x{}", width, height);

        imp.settings.set_int("window-width", width)?;
        imp.settings.set_int("window-height", height)?;

        imp.settings
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let imp = self.imp();

        let width = imp.settings.int("window-width");
        let height = imp.settings.int("window-height");
        let is_maximized = imp.settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    fn set_color(&self, color: Color) {
        let imp = self.imp();
        imp.color.replace(color);

        imp.red_scale.set_color_value(color.red as u32);
        imp.hex_entry.set_color(color.into());
    }

    // fn change_color(&self, color: Color) {}

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        //get scales
        let red_scale = imp.red_scale.get().imp().scale.get();
        // let bg_entry = imp.bg_entry.get();

        let on_scale_value_changed = glib::clone!(@weak red_scale,
            //@weak bg_entry,
             @weak self as window => move |_scale: &gtk::Scale| {
            let red = red_scale.value() as u8;
            let green = red_scale.value() as u8;
            let blue = red_scale.value() as u8;
            let alpha = red_scale.value() as u8;

            let color = Color::rgba(red, green, blue, alpha);

            window.set_color(color);
        });

        red_scale.connect_value_changed(on_scale_value_changed.clone());
        // let fg_handle = fg_entry.connect_changed(on_entry_changed);

        //clone self to show toast
        let captured = self.clone();

        imp.hex_entry.connect_changed(move |entry| {
            let gdk_color = entry.color();
            let color = Color::from(gdk_color);
            // captured.show_toast(&format!(
            //     "Color: {}",
            //     hex_color.to_hex_string(crate::model::AlphaPosition::End)
            // ))

            // if let Ok(color) = Color::from(&gdk_color) {
            log::info!("Hex Color: {:?}", color);
            if color != captured.imp().color.borrow().to_owned() {
                captured.set_color(color);
            }
            //     log::info!(
            //         "Hex Color as hex: {:?}",
            //         color.to_hex_string(crate::model::AlphaPosition::End)
            //     );
            // } else {
            //     //show error toast
            //     captured.show_toast("Failed to parse color");
            // }
        });
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(&adw::Toast::new(text));
    }
}
