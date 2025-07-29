use ashpd::desktop::global_shortcuts::NewShortcut;
use futures::StreamExt;
use gettextrs::gettext;
use glib::ExitCode;
use log::{debug, info};

use adw::prelude::AdwDialogExt;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, gio, glib};
use search_provider::{IconData, ResultID, ResultMeta, SearchProvider, SearchProviderImpl};

use crate::colors::color::Color;
use crate::config::{self, APP_ID, PKGDATADIR, PROFILE, VERSION};
use crate::widgets::about_window::EyedropperAbout;
use crate::widgets::preferences::preferences_window::PreferencesWindow;
use crate::window::AppWindow;

/// Identifier of color picking shortcut.
const SHORTCUT_PICK_COLOR: &str = "EyedropperColorPick";

mod imp {

    use std::cell::OnceCell;

    use super::*;
    use adw::subclass::prelude::AdwApplicationImpl;
    use glib::WeakRef;

    #[derive(Default)]
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

            let ctx = glib::MainContext::default();
            ctx.spawn_local(glib::clone!(
                #[weak]
                app,
                async move {
                    if let Err(err) = app.request_background_access().await {
                        log::error!("Failed to request background access: {err}");
                    }
                    // spawn indefinitely running task
                    futures::future::join(
                        async {
                            if let Err(err) = app.setup_global_shortcuts().await {
                                log::error!("Failed to request global shortcuts: {err}");
                            }
                        },
                        async {
                            if let Err(err) = app.setup_search_provider().await {
                                log::error!("Failed to start search provider: {err}");
                            }
                        },
                    )
                    .await;
                }
            ));

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
            .property("application-id", Some(APP_ID))
            .property(
                "resource-base-path",
                Some("/com/github/finefindus/eyedropper/"),
            )
            .build()
    }

    fn main_window(&self) -> AppWindow {
        self.imp().window.get().unwrap().upgrade().unwrap()
    }

    fn setup_gactions(&self) {
        // Clear the history
        let action_clear_history = gio::ActionEntry::builder("clear-history")
            .activate(|app: &Self, _, _| {
                app.main_window().clear_history();
            })
            .build();

        // Randomize the current color
        let action_random_color = gio::ActionEntry::builder("random-color")
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
                EyedropperAbout::show(&app.main_window());
            })
            .build();

        self.add_action_entries([
            action_clear_history,
            action_random_color,
            action_preferences,
            action_quit,
            action_about,
        ]);
    }

    // Sets up keyboard shortcuts
    fn setup_accels(&self) {
        self.set_accels_for_action("win.pick-color", &["<Control>p"]);
        self.set_accels_for_action("app.random-color", &["<Control>r"]);
        self.set_accels_for_action("app.preferences", &["<Control>comma"]);
        self.set_accels_for_action("app.quit", &["<Control>w", "<Control>q"]);
    }

    fn show_preferences_dialog(&self) {
        let preferences = PreferencesWindow::new();
        preferences.present(Some(&self.main_window()));
        preferences.connect_closed(glib::clone!(
            #[weak(rename_to = app)]
            self,
            move |_| {
                app.main_window().order_formats();
            }
        ));
    }

    pub fn run(&self) -> ExitCode {
        info!("Eyedropper ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }

    /// Returns a [`gdk_pixbuf::Pixbuf`] of circular icon rendered in the given color.
    ///
    /// The color should be in a format, that can be parsed by [`gtk::gdk::RGBA`].
    ///
    /// # Panics
    /// This function may panic, if some of the underlying code return [`None`].
    fn icon(color: gdk::RGBA) -> Result<gtk::gdk_pixbuf::Pixbuf, glib::Error> {
        const SIZE: i32 = 48;

        let display = gdk::Display::default().unwrap();
        let theme = gtk::IconTheme::for_display(&display);
        let paintable = theme.lookup_icon(
            "circle-symbolic",
            &[],
            SIZE,
            1,
            gtk::TextDirection::Ltr,
            gtk::IconLookupFlags::FORCE_SYMBOLIC,
        );

        let snapshot = gtk::Snapshot::new();

        let renderer = gtk::gsk::GLRenderer::new();
        renderer.realize(gdk::Surface::NONE)?;
        paintable.snapshot_symbolic(&snapshot, SIZE.into(), SIZE.into(), &[color]);

        let texture = renderer.render_texture(snapshot.to_node().unwrap(), None);
        renderer.unrealize();

        gtk::gdk_pixbuf::Pixbuf::from_stream(
            &gio::MemoryInputStream::from_bytes(&texture.save_to_png_bytes()),
            gio::Cancellable::NONE,
        )
    }

    /// Requests the system to allow the application to run in the background.
    ///
    /// This is used for receiving global shortcuts, while the app is not opened.
    async fn request_background_access(&self) -> ashpd::Result<()> {
        let response = ashpd::desktop::background::Background::request()
            .reason(&*gettext(
                "Allow color selection while the application runs in the background",
            ))
            .command(["eyedropper", "--gapplication-service"])
            .auto_start(true)
            .dbus_activatable(false)
            .send()
            .await?
            .response()?;
        log::info!(
            "Application has background access: {}",
            response.run_in_background()
        );
        Ok(())
    }

    async fn setup_search_provider(&self) -> zbus::Result<SearchProvider<App>> {
        let search_provider_path = config::OBJECT_PATH;
        let search_provider_name = format!("{}.SearchProvider", config::APP_ID);
        log::debug!(
            "Starting search provider as {} on {}",
            search_provider_name,
            search_provider_path
        );
        SearchProvider::new(self.clone(), search_provider_name, search_provider_path).await
    }

    /// Setup global shortcuts.
    ///
    /// A global shortcut can be used when the application is not focused.
    /// Uses the [Global Shortcuts portal](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html).
    async fn setup_global_shortcuts(&self) -> ashpd::Result<()> {
        let root = self
            .main_window()
            .root()
            .expect("Failed to get window root");
        let identifier = ashpd::WindowIdentifier::from_native(&root).await;

        let global_shortcuts = ashpd::desktop::global_shortcuts::GlobalShortcuts::new().await?;
        let session = global_shortcuts.create_session().await?;

        let request = global_shortcuts
            .bind_shortcuts(
                &session,
                &[
                    NewShortcut::new(SHORTCUT_PICK_COLOR, gettext("Pick a New Color"))
                        .preferred_trigger(Some("CTRL+p")),
                ],
                identifier.as_ref(),
            )
            .await?;

        let shortcuts = global_shortcuts
            .list_shortcuts(&session)
            .await?
            .response()?;

        if shortcuts.shortcuts().is_empty() {
            // request to set shortcuts if none have been set so far
            request.response()?;
        }

        log::debug!("Listening for global shortcuts");
        let mut stream = global_shortcuts.receive_activated().await?;
        while let Some(shortcut) = stream.next().await {
            if shortcut.shortcut_id() == SHORTCUT_PICK_COLOR {
                self.main_window().pick_color().await;
                self.main_window().present();
            }
        }
        session.close().await
    }
}

impl SearchProviderImpl for App {
    fn activate_result(&self, identifier: ResultID, _terms: &[String], _timestamp: u32) {
        self.activate();
        let window = self.main_window();

        if let Ok(rgba) = gdk::RGBA::parse(identifier) {
            window.set_color(rgba.into());
        }

        window.present();
    }

    fn initial_result_set(&self, terms: &[String]) -> Vec<ResultID> {
        terms
            .iter()
            .filter_map(|term| gdk::RGBA::parse(term).ok())
            .map(|color| color.to_string())
            .collect::<Vec<_>>()
    }

    fn result_metas(&self, identifiers: &[ResultID]) -> Vec<ResultMeta> {
        identifiers
            .iter()
            .filter_map(|identifier| {
                Some(
                    ResultMeta::builder(identifier.to_owned(), identifier)
                        .icon_data(IconData::from(
                            &App::icon(gdk::RGBA::parse(identifier).ok()?).ok()?,
                        ))
                        .build(),
                )
            })
            .collect::<Vec<_>>()
    }
}
