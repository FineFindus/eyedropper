use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::App;
use crate::config::{APP_ID, PROFILE};
use crate::model::{AlphaPosition, Color};
use crate::widgets::color_model_entry::ColorModelEntry;

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
        pub hex_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub rgb_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub hsl_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub hsv_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub cmyk_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
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
                rgb_entry: TemplateChild::default(),
                hsl_entry: TemplateChild::default(),
                hsv_entry: TemplateChild::default(),
                cmyk_entry: TemplateChild::default(),
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

    pub fn set_color(&self, color: Color) {
        //only update when necessary, to avoid infinite loop
        if *self.imp().color.borrow() != color {
            log::info!(
                "Changing Hex Color: {:?}",
                color.to_hex_string(crate::model::AlphaPosition::End)
            );
            let imp = self.imp();
            imp.color.replace(color);

            let hex_alpha_position =
                AlphaPosition::from(self.imp().settings.int("alpha-position") as u32);

            imp.hex_entry
                .set_color(color.to_hex_string(hex_alpha_position));

            imp.rgb_entry.set_color(format!(
                "rgb({}, {}, {})",
                color.red, color.green, color.blue
            ));

            let hsl = color.to_hsl();
            imp.hsl_entry
                .set_color(format!("hsl({}, {}%, {}%)", hsl.0, hsl.1, hsl.2));

            let hsv = color.to_hsv();
            imp.hsv_entry
                .set_color(format!("hsv({}, {}%, {}%)", hsv.0, hsv.1, hsv.2));

            let cmyk = color.to_cmyk();
            imp.cmyk_entry.set_color(format!(
                "cmyk({}%, {}%, {}%, {}%)",
                cmyk.0, cmyk.1, cmyk.2, cmyk.3
            ));
        }
    }

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        //show a toast when copying values
        let show_toast_closure = glib::closure_local!(@watch self as window => move |_: ColorModelEntry, text: String| {
            window.show_toast(&format!("Copied to clipboard: “{}”", text))
        });

        imp.hex_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.rgb_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.hsl_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.hsv_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.cmyk_entry
            .connect_closure("copied-color", false, show_toast_closure);

        //update hex entry with new alpha position
        self.imp().settings.connect_changed(
            Some("alpha-position"),
            glib::clone!(@weak self as window => move |settings, _| {
                log::debug!("Updating AlphaPosition");
                let color = *window.imp().color.borrow();
                let hex_alpha_position = AlphaPosition::from(settings.int("alpha-position") as u32);
                window.imp().hex_entry.set_color(color.to_hex_string(hex_alpha_position));
            }),
        );
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(&adw::Toast::new(text));
    }
}
