use std::path::Path;

use crate::config;
use crate::config::{APP_ID, VERSION};
use adw::{prelude::*, AboutDialog};
use gettextrs::gettext;
use gettextrs::pgettext;
use gtk::License;

pub const COPYRIGHT: &str = "Copyright © 2022 - 2023 FineFindus";
pub const WEBSITE: &str = "https://github.com/finefindus/eyedropper/";
pub const ISSUE_TRACKER: &str = "https://github.com/finefindus/eyedropper/issues/new/choose";
pub const DEVELOPERS: &[&str] = &["FineFindus https://github.com/FineFindus"];
pub const DESIGNERS: &[&str] = &["FineFindus https://github.com/FineFindus"];
pub const ARTISTS: &[&str] = &["bertob https://github.com/bertob", "Brage Fuglseth"];

#[derive(Debug)]
pub struct EyedropperAbout;

impl EyedropperAbout {
    pub fn show(parent: &impl IsA<gtk::Widget>) {
        let details = Self::details();

        //translatable changelog
        let changelog = Self::changelog();
        let changelog_version = config::VERSION;

        let debug_info = Self::debug_info();

        AboutDialog::builder()
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
            .build()
            .present(Some(parent));
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
        pgettext(
            "The changelog of the current version. Should be similar to the CHANGELOG.md file at the project root. Please ensure that the tags have a matching closing tag. Since xgettext does not recognize rust multiline strings, this should be on a single line using line breaks (\\n) for new lines.",
            "<p>A new release with exciting new features and many improvements.</p>\n<ul><li>Allow entering any format</li><li>Display color in overview search</li><li>Export palettes to LibreOffice</li><li>A visual differentiation between color and background</li><li>Improved color conversion</li><li>Visual refinements to match the state of the art of GNOME apps</li><li>Internal code improvements and bug fixes</li></ul>",
        )
    }

    ///Returns useful information for debugging the application.
    fn debug_info() -> String {
        let mut information = String::new();

        //information about the app
        information.push_str(&format!("Eyedropper: {}\n", config::VERSION));
        information.push_str(&format!("Profile: {}\n", config::PROFILE));
        information.push_str(&format!(
            "Backend: {}\n",
            Self::backend().unwrap_or("Failed to get backend")
        ));

        //used OS infos
        information.push_str("OS:\n");
        information.push_str(&format!(
            " - Name: {:?}\n",
            gtk::glib::os_info("NAME").unwrap_or("Failed to get OS NAME".into())
        ));
        information.push_str(&format!(
            " - Version: {:?}\n",
            gtk::glib::os_info("VERSION").unwrap_or("Failed to get VERSION".into())
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

        information
    }

    ///Returns the used display server
    fn backend() -> Option<&'static str> {
        let display = gtk::gdk::Display::default()?;
        //get display backend
        Some(match display.backend() {
            gtk::gdk::Backend::Wayland => "Wayland",
            gtk::gdk::Backend::X11 => "X11",
            gtk::gdk::Backend::Win32 => "Win32",
            gtk::gdk::Backend::MacOS => "MacOS",
            gtk::gdk::Backend::Broadway => "Broadway",
        })
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
                    _ => {}
                });
        } else {
            let gtk_portal_env = std::env::var("GTK_USE_PORTAL")
                .map(|v| v == "1")
                .unwrap_or_default();
            info.push_str(&format!(" - GTK_USE_PORTAL: {}\n", gtk_portal_env));
        }

        info
    }
}
