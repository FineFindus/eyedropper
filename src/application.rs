use gettextrs::gettext;
use log::{debug, info};

use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::model::color::Color;
use crate::widgets::preferences::PreferencesWindow;
use crate::window::AppWindow;

mod imp {

    use crate::model::color::Color;

    use super::*;
    use adw::subclass::prelude::AdwApplicationImpl;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default)]
    pub struct App {
        pub color: Color,
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
        fn activate(&self, app: &Self::Type) {
            debug!("GtkApplication<App>::activate");
            self.parent_activate(app);

            if let Some(window) = self.window.get() {
                let window = window.upgrade().unwrap();
                window.present();
                return;
            }

            let window = AppWindow::new(app);
            self.window
                .set(window.downgrade())
                .expect("Window already set.");

            app.main_window().present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("GtkApplication<App>::startup");
            self.parent_startup(app);

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

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
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl App {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some(APP_ID)),
            ("flags", &gio::ApplicationFlags::empty()),
            (
                "resource-base-path",
                &Some("/com/github/finefindus/eyedropper/"),
            ),
        ])
        .expect("Application initialization failed...")
    }

    fn main_window(&self) -> AppWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Pick a color using the picker button
        let action_pick_color = gio::SimpleAction::new("pick_color", None);
        action_pick_color.connect_activate(clone!(@weak self as app => move |_, _| {
            app.main_window().pick_color();
        }));
        self.add_action(&action_pick_color);

        // Randomize the current color
        let action_random_color = gio::SimpleAction::new("random_color", None);
        action_random_color.connect_activate(clone!(@weak self as app => move |_, _| {
            // Set the color to a random color
            app.main_window().set_color(Color::random());
        }));
        self.add_action(&action_random_color);

        // Preferences
        let action_preferences = gio::SimpleAction::new("preferences", None);
        action_preferences.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_preferences_dialog();
        }));
        self.add_action(&action_preferences);

        // Quit
        let action_quit = gio::SimpleAction::new("quit", None);
        action_quit.connect_activate(clone!(@weak self as app => move |_, _| {
            // This is needed to trigger the delete event and saving the window state
            app.main_window().close();
            app.quit();
        }));
        self.add_action(&action_quit);

        // About
        let action_about = gio::SimpleAction::new("about", None);
        action_about.connect_activate(clone!(@weak self as app => move |_, _| {
            app.show_about_dialog();
        }));
        self.add_action(&action_about);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        //quit app
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
        let dialog = gtk::AboutDialog::builder()
            .logo_icon_name(APP_ID)
            .license_type(gtk::License::Gpl30)
            .website("https://github.com/finefindus/eyedropper/")
            .version(VERSION)
            .transient_for(&self.main_window())
            // Translators: This should not be translate, Please enter your credits here instead (format: "Name https://example.com" or "Name <email@example.com>", no quotes)
            .translator_credits(&gettext("translator-credits"))
            .modal(true)
            .copyright("© 2022 FineFindus")
            .authors(vec!["FineFindus".into()])
            .artists(vec!["FineFindus".into()])
            .build();

        dialog.present();
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
