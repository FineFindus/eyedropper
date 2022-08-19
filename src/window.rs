use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::App;
use crate::config::{APP_ID, PROFILE};
use crate::model::Color;
use crate::widgets::color_entry::ColorEntry;

mod imp {
    use std::cell::RefCell;

    use crate::widgets;

    use super::*;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub headerbar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub color_picker_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub hex_entry: TemplateChild<widgets::color_entry::ColorEntry>,
        #[template_child]
        pub red_scale: TemplateChild<widgets::color_scale::ColorScale>,
        #[template_child]
        pub green_scale: TemplateChild<widgets::color_scale::ColorScale>,
        #[template_child]
        pub blue_scale: TemplateChild<widgets::color_scale::ColorScale>,
        #[template_child]
        pub alpha_scale: TemplateChild<widgets::color_scale::ColorScale>,
        pub settings: gio::Settings,
        pub color: RefCell<Color>,
    }

    impl Default for AppWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                color_picker_button: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                hex_entry: TemplateChild::default(),
                red_scale: TemplateChild::default(),
                green_scale: TemplateChild::default(),
                blue_scale: TemplateChild::default(),
                alpha_scale: TemplateChild::default(),
                settings: gio::Settings::new(APP_ID),
                color: RefCell::new(Color::rgba(0, 0, 0, 0)),
            }
        }
    }

    #[gtk::template_callbacks]
    impl AppWindow {
        #[template_callback]
        fn color_picker_button_clicked(&self) {
            self.instance().pick_color();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            Self::bind_template_callbacks(klass);
        }

        // You must call `Widget`'s `init_template()` within `instance_init()`.
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AppWindow {
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

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            if let Err(err) = window.save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request(window)
        }
    }

    impl ApplicationWindowImpl for AppWindow {}
    impl AdwApplicationWindowImpl for AppWindow {}
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

impl AppWindow {
    pub fn new(app: &App) -> Self {
        let window: Self =
            glib::Object::new(&[("application", app)]).expect("Failed to create AppWindow");
        //set scale labels with only initials
        let imp = window.imp();
        imp.red_scale.set_label(String::from("R"));
        imp.green_scale.set_label(String::from("G"));
        imp.blue_scale.set_label(String::from("B"));
        imp.alpha_scale.set_label(String::from("A"));
        //preset a color, so all scales have a set position
        window.set_color(Color::rgba(46, 52, 64, 255));
        window
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

    fn pick_color(&self) {
        log::debug!("Picking a color using the color picker");
        gtk_macros::spawn!(glib::clone!(@weak self as window => async move {

        let connection = ashpd::zbus::Connection::session().await.expect("Failed to open connection to zbus");
        let proxy = ashpd::desktop::screenshot::ScreenshotProxy::new(&connection).await.expect("Failed to open screenshot proxy");
        match proxy.pick_color(&ashpd::WindowIdentifier::default()).await {
            Ok(color) => window.set_color(Color::from(color)),
            Err(_) => window.show_toast("Failed to pick a color"),
        };
        }));
    }

    fn set_color(&self, color: Color) {
        //only update when necessary, to avoid infinite loop
        if self.imp().color.borrow().clone() != color {
            log::info!(
                "Hex Color: {:?}",
                color.to_hex_string(crate::model::AlphaPosition::End)
            );
            let imp = self.imp();
            imp.color.replace(color);

            imp.red_scale.set_color_value(color.red);
            imp.green_scale.set_color_value(color.green);
            imp.blue_scale.set_color_value(color.blue);
            imp.alpha_scale.set_color_value(color.alpha);
            imp.hex_entry.set_color(color.into());
        }
    }

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        //get scales
        let red_scale = imp.red_scale.get().imp().scale.get();
        let green_scale = imp.green_scale.get().imp().scale.get();
        let blue_scale = imp.blue_scale.get().imp().scale.get();
        let alpha_scale = imp.alpha_scale.get().imp().scale.get();

        let on_scale_value_changed = glib::clone!(
            @weak red_scale,
            @weak green_scale,
            @weak blue_scale,
            @weak alpha_scale,
             @weak self as window => move |_scale: &gtk::Scale| {
            let red = red_scale.value() as u8;
            let green = green_scale.value() as u8;
            let blue = blue_scale.value() as u8;
            let alpha = alpha_scale.value() as u8;

            let color = Color::rgba(red, green, blue, alpha);

            window.set_color(color);
        });

        let red_handle = red_scale.connect_value_changed(on_scale_value_changed.clone());
        let green_handle = green_scale.connect_value_changed(on_scale_value_changed.clone());
        let blue_handle = blue_scale.connect_value_changed(on_scale_value_changed.clone());
        let alpha_handle = alpha_scale.connect_value_changed(on_scale_value_changed);

        imp.hex_entry.connect_changed(glib::clone!(
                @weak red_scale,
                @weak green_scale,
                @weak blue_scale,
                @weak alpha_scale,
                @weak self as window => move |entry| {

            //block all signals, so no endless loop with updating signals can be created
            red_scale.block_signal(&red_handle);
            green_scale.block_signal(&green_handle);
            blue_scale.block_signal(&blue_handle);
            alpha_scale.block_signal(&alpha_handle);

            //get color
            let gdk_color = entry.color();
            let color = Color::from(gdk_color);
            window.set_color(color);
            //unblock after updating color
            red_scale.unblock_signal(&red_handle);
            green_scale.unblock_signal(&green_handle);
            blue_scale.unblock_signal(&blue_handle);
            alpha_scale.unblock_signal(&alpha_handle);
        }));

        let captured = self.clone();
        imp.hex_entry.connect_closure(
            "copied-color",
            false,
            glib::closure_local!(move |_: ColorEntry, color: String| {
                captured.show_toast(&format!("Copied color {}", color))
            }),
        );

        //update hex entry with new alpha position
        self.imp().settings.connect_changed(
            Some("alpha-position"),
            glib::clone!(@weak self as window => move |_, _| {
                window.imp()
                    .hex_entry
                    .update_alpha_position()
            }),
        );
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(&adw::Toast::new(text));
    }
}
