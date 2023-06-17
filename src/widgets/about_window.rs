use std::path::Path;

use crate::config;
use crate::config::{APP_ID, VERSION};
use adw::prelude::*;
use adw::AboutWindow;
use gettextrs::gettext;
use gettextrs::pgettext;
use glib::object::IsA;
use gtk::{Application, License, Window};

//code 'inspired' by https://gitlab.com/news-flash/news_flash_gtk/-/blob/master/src/about_dialog.rs

//this is non-translatable information, so it can be const
pub const COPYRIGHT: &str = "Copyright © 2022 - 2023 FineFindus";
pub const WEBSITE: &str = "https://github.com/finefindus/eyedropper/";
pub const ISSUE_TRACKER: &str = "https://github.com/finefindus/eyedropper/issues/new/choose";
pub const DEVELOPERS: &[&str] = &["FineFindus https://github.com/FineFindus"];
pub const DESIGNERS: &[&str] = &["FineFindus https://github.com/FineFindus"];
pub const ARTISTS: &[&str] = &["bertob https://github.com/bertob"];

#[derive(Clone, Debug)]
pub struct EyedropperAbout;

impl EyedropperAbout {
    pub fn show<A: IsA<Application> + AdwApplicationExt, W: IsA<Window> + GtkWindowExt>(
        app: &A,
        window: &W,
        portal_error: Option<ashpd::Error>,
    ) {
        //translatable strings which will be retrieved via gettext
        let details = Self::details();

        //translatable changelog
        let changelog = Self::changelog();
        let changelog_version = config::VERSION;

        let debug_info = Self::debug_info(portal_error);

        let about_window = AboutWindow::builder()
            .application(app)
            .transient_for(window)
            .modal(true)
            .application_icon(APP_ID)
            .application_name(gettext("Eyedropper"))
            .developer_name(DEVELOPERS[0].split(' ').collect::<Vec<&str>>()[0])
            .developers(DEVELOPERS)
            .designers(DESIGNERS)
            .artists(ARTISTS)
            // Translators: This should not be translate, Please enter your credits here instead (format: "Name https://example.com" or "Name <email@example.com>", no quotes)
            .translator_credits(gettext("translator-credits"))
            .license_type(License::Gpl30)
            .version(VERSION)
            .website(WEBSITE)
            .issue_url(ISSUE_TRACKER)
            .comments(details)
            .copyright(COPYRIGHT)
            .release_notes(changelog)
            .release_notes_version(changelog_version)
            .debug_info(debug_info)
            .debug_info_filename("eyedropper_debug_info")
            .build();
        about_window.set_visible(true);
    }

    /// Build the details page text out of single components,
    /// which are also used in the store description, etc...
    ///
    /// These should already be used in the metainfo file, this makes keeping them in sync easier
    fn details() -> String {
        let mut details = String::new();

        //heading with summary
        details.push_str("<b>");
        details.push_str(&gettext("Pick and format colors"));
        details.push_str("</b>\n\n");

        details.push_str(&gettext("Pick any color from your screen and view it in different formats. Change the picked color or go back to a previously picked color from the history list. Generate a list of different shades from the picked color."));
        details.push_str("\n\n");

        //feature list
        details.push_str("<b>");
        details.push_str(&gettext("Features"));
        details.push_str("</b>\n\n");

        let features = vec![
            gettext("Pick a Color"),
            gettext("Enter a color in Hex-Format"),
            gettext("Parse RGBA/ARGB Hex-Colors"),
            gettext("View colors in Hex, RGB, HSV, HSL, CMYK, XYZ and CIE-Lab format"),
            gettext("Customize which formats appear as well as their order"),
            gettext("Generate a palette of different shades"),
        ];

        //push to features as a list with new lines
        for feature in features {
            details.push_str("- ");
            details.push_str(&feature);
            details.push('\n');
        }

        details
    }

    /// Returns the changelog for the last major version.
    fn changelog() -> String {
        let mut changelog = String::new();

        changelog.push_str(&pgettext(
            "The changelog of the current version. Should be similar to the CHANGELOG.md file at the project root. Please ensure that the tags have a matching closing tag. Since xgettext does not recognize rust multiline strings, this should be on a single line using line breaks (\\n) for new lines.",
            "<p>A minor release with many small QoL improvements.</p><p>Added:</p>\n<ul><li>Show a toast with an Undo option when clearing the history</li><li>Use Ctrl-Q to close the window</li><li>Use a default minimum window size, content should no longer unnecessarily cut off</li><li>New icon by @bertob</li><li>Export palettes as Adobe Swatch Exchange (ASE), hex, PAL, Paint.NET (txt) files</li><li>Minor changes to the store metadata</li></ul><p>Changed:</p>\n<ul><li>Hunter-Lab now displays 0 instead of NaN</li><li>Improved tooltips for copying colors</li><li>Palette window now uses a button with a more explicit text instead of an icon</li><li>Reworded project description</li><li>Updated all screenshots</li></ul><p>Fixed:</p>\n<ul><li>Separator is now hidden when the history list is hidden</li><li>Improved editing experience when change the Hex color/name</li></ul>",
        ));

        changelog
    }

    ///Returns useful information for debugging the application.
    fn debug_info(portal_error: Option<ashpd::Error>) -> String {
        let mut information = String::new();

        //information about the app
        information.push_str(&format!("Eyedropper: {}\n", config::VERSION));
        information.push_str(&format!("Profile: {}\n", config::PROFILE));
        information.push_str(&format!(
            "Backend: {}\n",
            Self::backend().unwrap_or_else(|| "Failed to get backend".to_owned())
        ));

        //used OS infos
        information.push_str("OS:\n");
        information.push_str(&format!(
            " - Name: {:?}\n",
            gtk::glib::os_info("NAME").unwrap_or_else(|| "Failed to get OS NAME".into())
        ));
        information.push_str(&format!(
            " - Version: {:?}\n",
            gtk::glib::os_info("VERSION").unwrap_or_else(|| "Failed to get VERSION".into())
        ));
        //used libraries version
        information.push_str("Libraries:\n");
        information.push_str(&format!(
            " - GTK: {}.{}.{}\n",
            gtk::major_version(),
            gtk::minor_version(),
            gtk::micro_version()
        ));
        information.push_str(&format!(
            " - Libadwaita: {}.{}.{}\n",
            adw::major_version(),
            adw::minor_version(),
            adw::micro_version()
        ));
        information.push('\n');

        information.push_str(&format!("Sandbox: {}\n", Self::sandbox_info()));
        information.push('\n');

        //add potential portal error
        information.push_str(&format!("Portal error: {:?}", portal_error));
        information.push('\n');

        information
    }

    ///Returns the used display server
    fn backend() -> Option<String> {
        let display = gtk::gdk::Display::default()?;
        //get display backend
        Some(
            match display.backend() {
                gtk::gdk::Backend::Wayland => "Wayland",
                gtk::gdk::Backend::X11 => "X11",
                gtk::gdk::Backend::Win32 => "Win32",
                gtk::gdk::Backend::MacOS => "MacOS",
                gtk::gdk::Backend::Broadway => "Broadway",
            }
            .to_owned(),
        )
    }

    /// Returns info about the sandbox the app is using.
    ///
    /// If it is running inside the flatpak sandbox, info about it is returned, otherwise only
    /// the information if the `GTK_USE_PORTAL` environment is set to `1`.  
    fn sandbox_info() -> String {
        let mut info = String::new();
        if Path::new("/.flatpak-info").exists() {
            info.push_str("Flatpak Info:\n");
            let info_file = std::fs::read_to_string("/.flatpak-info").unwrap_or_default();
            info_file
                .split('\n')
                .filter_map(|line| line.split_once('='))
                .for_each(|(name, value)| match name {
                    "name" => info.push_str(&format!(" - Name: {}\n", value)),
                    "runtime" => info.push_str(&format!(" - Runtime: {}\n", value)),
                    "runtime-commit" => info.push_str(&format!(" - Runtime commit: {}\n", value)),
                    "arch" => info.push_str(&format!(" - Arch: {}\n", value)),
                    "flatpak-version" => info.push_str(&format!(" - Flatpak Version: {}\n", value)),
                    "devel" => info.push_str(&format!(" - Devel: {}\n", value)),
                    "LANG" => info.push_str(&format!(" - Language: {}\n", value)),
                    _ => {}
                });
        } else {
            let gtk_portal_env = std::env::var("GTK_USE_PORTAL")
                .map(|v| v == "1")
                .unwrap_or(false);
            info.push_str(&format!(" - GTK_USE_PORTAL: {}\n", gtk_portal_env));
        }

        info
    }
}
