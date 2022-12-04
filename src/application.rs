use log::{debug, info};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::colors::color::Color;
use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::widgets::about_window::EyedropperAbout;
use crate::widgets::preferences_window::PreferencesWindow;
use crate::window::AppWindow;

mod imp {

    use super::*;
    use adw::subclass::prelude::AdwApplicationImpl;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct App {
        pub window: OnceCell<WeakRef<AppWindow>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for App {
        const NAME: &'static str = "App";
        type Type = super::App;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for App {}

    impl ApplicationImpl for App {
        fn activate(&self) {
            debug!("GtkApplication<App>::activate");
            self.parent_activate();

            let app = self.obj();

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = AppWindow::new(&app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self) {
            debug!("GtkApplication<App>::startup");
            self.parent_startup();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);
            let app = self.obj();

            app.setup_css();
            app.setup_gactions();
            app.setup_accels();
        }
    }

    impl GtkApplicationImpl for App {}
    impl AdwApplicationImpl for App {}
}

glib::wrapper! {
    pub struct App(ObjectSubclass<imp::App>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::builder::<Self>()
            .property("application-id", &Some(APP_ID))
            .property("flags", &gio::ApplicationFlags::empty())
            .property(
                "resource-base-path",
                &Some("/com/github/finefindus/eyedropper/"),
            )
            .build()
    }

    fn main_window(&self) -> AppWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Pick a color using the picker button
        let action_pick_color = gio::ActionEntry::builder("pick_color")
            .activate(move |obj: &Self, _, _| {
                obj.main_window().pick_color();
            })
            .build();

        // Clear the history
        let action_clear_history = gio::ActionEntry::builder("clear_history")
            .activate(|app: &Self, _, _| {
                app.main_window().clear_history();
            })
            .build();

        // Randomize the current color
        let action_random_color = gio::ActionEntry::builder("random_color")
            .activate(|app: &Self, _, _| {
                // Set the color to a random color
                app.main_window().set_color(Color::random());
            })
            .build();

        // Preferences
        let action_preferences = gio::ActionEntry::builder("preferences")
            .activate(|app: &Self, _, _| {
                app.show_preferences_dialog();
            })
            .build();

        // Quit
        let action_quit = gio::ActionEntry::builder("quit")
            .activate(|app: &Self, _, _| {
                // This is needed to trigger the delete event and saving the window state
                app.main_window().close();
                app.quit();
            })
            .build();

        // About
        let action_about = gio::ActionEntry::builder("about")
            .activate(|app: &Self, _, _| {
                app.show_about_dialog();
            })
            .build();

        // It is safe to `unwrap` as we don't pass any parameter type that requires validation
        self.add_action_entries([
            action_pick_color,
            action_clear_history,
            action_random_color,
            action_preferences,
            action_quit,
            action_about,
        ])
        .unwrap();
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.pick_color", &["<Control>p"]);
        self.set_accels_for_action("app.random_color", &["<Control>r"]);
        self.set_accels_for_action("app.preferences", &["<Control>comma"]);
        self.set_accels_for_action("app.quit", &["<Control>q"]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/com/github/finefindus/eyedropper/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about_dialog(&self) {
        EyedropperAbout::show(self, &self.main_window());
    }

    fn show_preferences_dialog(&self) {
        let preferences = PreferencesWindow::new();
        preferences.set_transient_for(Some(&self.main_window()));
        preferences.show();
    }

    pub fn run(&self) {
        info!("Eyedropper ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self);
    }
}
