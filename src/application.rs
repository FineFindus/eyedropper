use glib::ExitCode;
use log::{debug, info};

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use search_provider::{ResultID, ResultMeta, SearchProviderImpl};

use crate::colors::color::Color;
use crate::colors::formatter::ColorFormatter;
use crate::config::{APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::widgets::about_window::EyedropperAbout;
use crate::widgets::preferences_window::PreferencesWindow;
use crate::window::AppWindow;

mod imp {

    use std::cell::RefCell;

    use crate::config;

    use super::*;
    use adw::subclass::prelude::AdwApplicationImpl;
    use glib::WeakRef;
    use once_cell::sync::OnceCell;
    use search_provider::SearchProvider;

    #[derive(Default)]
    pub struct App {
        pub window: OnceCell<WeakRef<AppWindow>>,
        pub search_provider: RefCell<Option<SearchProvider<super::App>>>,
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

            let ctx = glib::MainContext::default();

            let search_provider_path = config::OBJECT_PATH;
            let search_provider_name = format!("{}.SearchProvider", config::APP_ID);
            log::debug!(
                "Starting search provider as {} on {}",
                search_provider_name,
                search_provider_path
            );

            ctx.spawn_local(glib::clone!(@weak app => async move {
                match SearchProvider::new(app.clone(), search_provider_name, search_provider_path).await {
                    Ok(search_provider) => {
                        app.imp().search_provider.replace(Some(search_provider));
                    },
                    Err(err) => log::debug!("Could not start search provider: {}", err),
                };

            }));

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
        ]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("app.pick_color", &["<Control>p"]);
        self.set_accels_for_action("app.random_color", &["<Control>r"]);
        self.set_accels_for_action("app.preferences", &["<Control>comma"]);
        self.set_accels_for_action("app.quit", &["<Control>w", "<Control>q"]);
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
        EyedropperAbout::show(
            self,
            &self.main_window(),
            self.main_window().imp().portal_error.take(),
        );
    }

    fn show_preferences_dialog(&self) {
        let preferences = PreferencesWindow::new();
        preferences.set_transient_for(Some(&self.main_window()));
        preferences.show();
    }

    pub fn run(&self) -> ExitCode {
        info!("Eyedropper ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }
}

impl SearchProviderImpl for App {
    fn activate_result(&self, identifier: ResultID, _terms: &[String], timestamp: u32) {
        let window = self.main_window();
        window.set_color(Color::from(identifier));
        window.present_with_time(timestamp);
    }

    fn initial_result_set(&self, terms: &[String]) -> Vec<ResultID> {
        terms
            .iter()
            .filter_map(|term| {
                Color::from_hex(term, crate::colors::position::AlphaPosition::None).ok()
            })
            .map(|color| ColorFormatter::with_color(color).hex_code())
            .collect::<Vec<_>>()
    }

    fn result_metas(&self, identifiers: &[ResultID]) -> Vec<ResultMeta> {
        log::debug!("ResultMeta for: {:?}", identifiers);
        identifiers
            .iter()
            .map(|identifier| ResultMeta::builder(identifier.to_owned(), &identifier).build())
            .collect::<Vec<_>>()
    }
}
