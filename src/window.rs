use gettextrs::{gettext, pgettext};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::application::App;
use crate::colors::color::Color;
use crate::colors::color_names;
use crate::colors::formatter::ColorFormatter;
use crate::colors::illuminant::Illuminant;
use crate::colors::position::AlphaPosition;
use crate::config::{APP_ID, PROFILE};
use crate::model::history::HistoryObject;
use crate::widgets::color_format_row::ColorFormatRow;
use crate::widgets::palette_dialog::PaletteDialog;

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
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub format_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub color_button: TemplateChild<gtk::ColorButton>,
        #[template_child]
        pub color_picker_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub hex_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub rgb_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub hsl_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub hsv_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub cmyk_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub xyz_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub cie_lab_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub hwb_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub hcl_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub name_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub lms_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub hunter_lab_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub history_list: TemplateChild<gtk::ListBox>,
        pub history: RefCell<Option<gio::ListStore>>,
        pub settings: gio::Settings,
        pub color: RefCell<Option<Color>>,
    }

    impl Default for AppWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                stack: TemplateChild::default(),
                color_button: TemplateChild::default(),
                color_picker_button: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                format_box: TemplateChild::default(),
                hex_row: TemplateChild::default(),
                rgb_row: TemplateChild::default(),
                hsl_row: TemplateChild::default(),
                hsv_row: TemplateChild::default(),
                cmyk_row: TemplateChild::default(),
                xyz_row: TemplateChild::default(),
                cie_lab_row: TemplateChild::default(),
                hunter_lab_row: TemplateChild::default(),
                hwb_row: TemplateChild::default(),
                hcl_row: TemplateChild::default(),
                name_row: TemplateChild::default(),
                lms_row: TemplateChild::default(),
                history_list: TemplateChild::default(),
                history: Default::default(),
                settings: gio::Settings::new(APP_ID),
                color: RefCell::new(None),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AppWindow {
        const NAME: &'static str = "AppWindow";
        type Type = super::AppWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for AppWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            // Devel Profile
            if PROFILE == "Devel" {
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
            obj.setup_history();
            obj.setup_callbacks();
            obj.set_order();
            obj.load_visibility_settings();
            obj.set_stack();
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        // Save window state on delete event
        fn close_request(&self) -> gtk::Inhibit {
            //save current window size
            if let Err(err) = self.obj().save_window_size() {
                log::warn!("Failed to save window state, {}", &err);
            }

            // Pass close request on to the parent
            self.parent_close_request()
        }
    }

    impl ApplicationWindowImpl for AppWindow {}
    impl AdwApplicationWindowImpl for AppWindow {}
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::Widget, gtk::Window,  gtk::ApplicationWindow,
        @implements gio::ActionMap, gio::ActionGroup, gtk::Root;
}

#[gtk::template_callbacks]
impl AppWindow {
    pub fn new(app: &App) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(&adw::Toast::new(text));
    }

    /// The currently picked color.
    ///
    /// Returns `None` if no color has been picked yet.
    fn color(&self) -> Option<Color> {
        *self.imp().color.borrow()
    }

    /// Set the stack to either show the main page or the placeholder,
    /// depending on if a color is chosen.
    fn set_stack(&self) {
        if self.color().is_some() {
            self.imp().stack.set_visible_child_name("main");
        } else {
            self.imp().stack.set_visible_child_name("placeholder");
        }
    }

    /// Returns the history list store object.
    fn history(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .history
            .borrow()
            .clone()
            .expect("Could not get current history.")
    }

    /// Clear the history by removing all items from the list.
    pub fn clear_history(&self) {
        //clear history
        let history = self.history();
        history.remove_all();
    }

    /// Setup the history by setting up a model
    fn setup_history(&self) {
        // Create new model
        let model = gio::ListStore::new(HistoryObject::static_type());

        // Get state and set model
        self.imp().history.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = gtk::NoSelection::new(Some(&self.history()));
        self.imp().history_list.bind_model(
            Some(&selection_model),
            glib::clone!(@weak self as window => @default-panic, move |obj| {
                let history_object = obj.downcast_ref().expect("The object is not of type `HistoryObject`.");
                let history_item = window.create_history_item(history_object);
                history_item.upcast()
            }),
        );

        // Assure that the history list is only visible when it is supposed to
        self.set_history_list_visible(&self.history());
        self.history().connect_items_changed(
            glib::clone!(@weak self as window => move |items, _, _, _| {
                window.set_history_list_visible(items);
            }),
        );
    }

    /// Assure that history is only visible
    /// if the number of items is greater than 0
    fn set_history_list_visible(&self, history: &gio::ListStore) {
        self.imp().history_list.set_visible(history.n_items() > 0);
    }

    /// Create a new history item
    fn create_history_item(&self, history_object: &HistoryObject) -> gtk::Button {
        let color_button = gtk::Button::builder().margin_end(2).margin_start(2).build();

        // color button with the history color
        let color = history_object.color();
        let mut formatter = ColorFormatter::with_color(color);
        let color_hex = formatter.hex_code();

        let class_name = format!("history-button-{}", color_hex.replace('#', ""));

        let css_provider = gtk::CssProvider::new();

        if let Some(display) = gtk::gdk::Display::default() {
            gtk::StyleContext::add_provider_for_display(&display, &css_provider, 400);
        }

        css_provider.load_from_data(
            format!(
                ".{} {{background-color: {};border-radius: 6px;}}",
                class_name,
                // ignore alpha values, they are not displayed properly
                color_hex
            )
            .as_bytes(),
        );
        color_button.add_css_class(&class_name);

        let tooltip = if color.alpha != 255 {
            formatter.alpha_position = AlphaPosition::End;
            formatter.hex_code()
        } else {
            color_hex
        };
        color_button.set_tooltip_text(Some(&tooltip));

        //switch to color when clicked
        color_button.connect_clicked(
            glib::clone!(@weak self as window, @weak history_object => move |_, | {
                window.set_color(history_object.color());
                //remove from history when clicking on it
                match window.history().find(&history_object) {
                    Some(index) => window.history().remove(index),
                    None => log::error!("Failed to find index for {}", history_object.color()),
                }
            }),
        );

        color_button
    }

    /// Save the window size when closing the window
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

    ///Load the last saved window size and apply it
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

    /// Update color when their visibility changes.
    fn load_visibility_settings(&self) {
        let imp = self.imp();
        let settings = &imp.settings;

        //update order
        settings.connect_changed(
            Some("format-order"),
            glib::clone!(@weak self as window => move |_, _| {
                log::debug!("Updating format order");
                window.set_order();
            }),
        );

        // update the color by setting it again
        let update_color = glib::clone!(@weak self as window => move |_: &gio::Settings, _:&str| {
            if let Some(color) = window.color() {
            window.set_color(color);
            }
        });

        //update hex row with new alpha position
        settings.connect_changed(Some("alpha-position"), update_color.clone());

        //update colors that use observer values in their calculation
        settings.connect_changed(Some("cie-illuminants"), update_color.clone());
        settings.connect_changed(Some("cie-standard-observer"), update_color.clone());

        // update color name
        settings.connect_changed(Some("show-color-name"), update_color.clone());

        //update precision
        settings.connect_changed(Some("use-default-precision"), update_color.clone());
        settings.connect_changed(Some("precision-digits"), update_color.clone());

        // update for custom formats
        settings.connect_changed(Some("custom-format-rgb"), update_color.clone());
        settings.connect_changed(Some("custom-format-hsl"), update_color.clone());
        settings.connect_changed(Some("custom-format-hsv"), update_color.clone());
        settings.connect_changed(Some("custom-format-cmyk"), update_color.clone());
        settings.connect_changed(Some("custom-format-xyz"), update_color.clone());
        settings.connect_changed(Some("custom-format-cie-lab"), update_color.clone());
        settings.connect_changed(Some("custom-format-hwb"), update_color.clone());
        settings.connect_changed(Some("custom-format-hcl"), update_color.clone());
        settings.connect_changed(Some("custom-format-lms"), update_color.clone());
        settings.connect_changed(Some("custom-format-hunter-lab"), update_color);

        imp.hex_row.set_settings_name("show-hex-format");
        imp.rgb_row.set_settings_name("show-rgb-format");
        imp.hsl_row.set_settings_name("show-hsl-format");
        imp.hsv_row.set_settings_name("show-hsv-format");
        imp.cmyk_row.set_settings_name("show-cmyk-format");
        imp.xyz_row.set_settings_name("show-xyz-format");
        imp.cie_lab_row.set_settings_name("show-cie-lab-format");
        imp.hwb_row.set_settings_name("show-hwb-format");
        imp.hcl_row.set_settings_name("show-hcl-format");
        imp.lms_row.set_settings_name("show-lms-format");
        imp.lms_row.set_settings_name("show-hunter-lab-format");
        imp.name_row.set_settings_name("show-color-name");

        //update name when it changes
        let update_color_names = glib::clone!(@weak self as window => move |settings: &gio::Settings, _: &str| {
            log::debug!("Updating color names");
            if let Some(color) = window.color() {
                let name = color_names::name(color,
                    settings.boolean("name-source-basic"),
                    settings.boolean("name-source-extended"),
                    settings.boolean("name-source-gnome-palette"),
                    settings.boolean("name-source-xkcd"),
                );
                window.
                imp().name_row.set_text(name.unwrap_or_else(|| pgettext(
                    "Information that no name for the color could be found",
                    "Not named",
                )));
            }
        });

        settings.connect_changed(Some("name-source-basic"), update_color_names.clone());
        settings.connect_changed(Some("name-source-extended"), update_color_names.clone());
        settings.connect_changed(Some("name-source-xkcd"), update_color_names.clone());

        let show_name_model = settings.boolean("show-color-name");
        if show_name_model {
            if let Some(color) = self.color() {
                //update field to show name
                let name = color_names::name(
                    color,
                    settings.boolean("name-source-basic"),
                    settings.boolean("name-source-extended"),
                    settings.boolean("name-source-gnome-palette"),
                    settings.boolean("name-source-xkcd"),
                );
                imp.name_row.set_text(name.unwrap_or_else(|| {
                    pgettext(
                        "Information that no name for the color could be found",
                        "Not named",
                    )
                }));
            }
        }
    }

    /// Insert the formats in the order in which they are saved in the settings.
    fn set_order(&self) {
        let imp = self.imp();
        let format_box = &imp.format_box;
        //clear box

        //remove all children that are color model entries
        format_box
            .observe_children()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatRow>)
            .for_each(|row| format_box.remove(row));

        //parse current order
        let order = imp.settings.get::<Vec<String>>("format-order");
        log::debug!("Format-Order: {:?}", order);

        //insert items in order
        for item in order {
            let child = match item.to_lowercase().as_str() {
                "hex" => &imp.hex_row,
                "rgb" => &imp.rgb_row,
                "hsl" => &imp.hsl_row,
                "hsv" => &imp.hsv_row,
                "cmyk" => &imp.cmyk_row,
                "xyz" => &imp.xyz_row,
                "cielab" => &imp.cie_lab_row,
                "hwb" => &imp.hwb_row,
                "hcl" => &imp.hcl_row,
                "name" => &imp.name_row,
                "lms" => &imp.lms_row,
                "hunterlab" => &imp.hunter_lab_row,
                _ => {
                    log::error!("Failed to find format: {}", item);
                    continue;
                }
            };

            format_box.append(&child.get());
        }
    }

    /// Opens a dialog with different palettes.
    ///
    /// When a palette is clicked it will be added to the history list.
    #[template_callback]
    fn open_palette_dialog(&self) {
        //safe to unwrap, if the user opens this dialog, the color button must be clicked
        let palette_dialog = PaletteDialog::new(self.color().unwrap());
        palette_dialog.set_transient_for(Some(self));
        palette_dialog.show();

        //when a palette is chosen, add all colors of the palette in reverse order to the history
        palette_dialog.connect_closure(
            "palette-clicked",
            false,
            glib::closure_local!(@watch self as window => move |_: PaletteDialog, palette: String| {
                log::debug!("Palette: {palette}");

                palette
                .split(' ')
                .for_each(|slice| match Color::from_hex(slice, AlphaPosition::None) {
                    Ok(color) => window.set_color(color),
                    Err(_) => {
                        log::error!("Failed to parse color {}", slice);
                        window.show_toast(&gettext("Failed to get palette color"))
                },
            });
            }),
        );
    }

    /// Pick a color from the desktop using [ashpd].
    ///
    /// It will show a toast when failing to pick a color, for example when the user cancels the action.
    #[template_callback]
    pub fn pick_color(&self) {
        log::debug!("Picking a color using the color picker");
        gtk_macros::spawn!(glib::clone!(@weak self as window => async move {

        let connection = ashpd::zbus::Connection::session().await.expect("Failed to open connection to zbus");
        let proxy = ashpd::desktop::screenshot::ScreenshotProxy::new(&connection).await.expect("Failed to open screenshot proxy");
        match proxy.pick_color(&ashpd::WindowIdentifier::default()).await {
            Ok(color) => window.set_color(Color::from(color)),
            Err(err) => {
                log::error!("{}", err);
                window.show_toast(&gettext("Failed to pick a color"));
            },
        };
        }));
    }

    /// Update the current color to the given color.
    /// The previous color will be added to the history list.
    pub fn set_color(&self, color: Color) {
        //append previous color to history
        if self.color() != Some(color) && self.color().is_some() {
            let history_item = HistoryObject::new(self.color().unwrap());
            self.history().insert(0, &history_item);
        }

        let imp = self.imp();
        let settings = &imp.settings;
        imp.color.replace(Some(color));

        //stop showing placeholder, when a color is set
        self.set_stack();

        imp.color_button.set_rgba(&color.into());

        let formatter = ColorFormatter::new(
            //observer is saved as an int (for technical reasons), so convert it back to an bool
            settings.int("cie-standard-observer") == 1,
            Illuminant::from(settings.int("cie-illuminants") as u32),
            AlphaPosition::from(settings.int("alpha-position") as u32),
            settings.boolean("use-default-precision"),
            settings.uint("precision-digits") as usize,
            color,
        );

        log::info!("Changing color: {}", formatter.hex_code());

        imp.hex_row.set_text(formatter.hex_code());

        imp.rgb_row.set_text(formatter.rgb());

        imp.hsl_row.set_text(formatter.hsl());

        imp.hsv_row.set_text(formatter.hsv());

        imp.cmyk_row.set_text(formatter.cmyk());

        imp.xyz_row.set_text(formatter.xyz());

        imp.cie_lab_row.set_text(formatter.cie_lab());

        imp.hwb_row.set_text(formatter.hwb());

        imp.hcl_row.set_text(formatter.hcl());

        //only update when necessary, to avoid searches, that might be a bit more costly
        if imp.name_row.is_visible() {
            let name = color_names::name(
                color,
                settings.boolean("name-source-basic"),
                settings.boolean("name-source-extended"),
                settings.boolean("name-source-gnome-palette"),
                settings.boolean("name-source-xkcd"),
            );
            imp.name_row.set_text(name.unwrap_or_else(|| {
                pgettext(
                    "Information that no name for the color could be found",
                    "Not named",
                )
            }));
        }

        imp.lms_row.set_text(formatter.lms());

        imp.hunter_lab_row.set_text(formatter.hunter_lab());
    }

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        //show a toast when copying values
        let show_toast_closure = glib::closure_local!(@watch self as window => move |_: ColorFormatRow, text: String| {
            //Translators: Do not replace the {}. These are used as placeholders for the copied values
            window.show_toast(&gettext("Copied: “{}”").replace("{}", &text));
        });

        imp.hex_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.rgb_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.hsl_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.hsv_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.cmyk_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.xyz_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.cie_lab_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.hwb_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.hcl_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.name_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.lms_row
            .connect_closure("copied-text", false, show_toast_closure.clone());
        imp.hunter_lab_row
            .connect_closure("copied-text", false, show_toast_closure);

        imp.hex_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |_: ColorFormatRow, color: String| {
                log::debug!("Changed hex entry: {color}");
                let hex_alpha_position = AlphaPosition::from(window.imp().settings.int("alpha-position") as u32);

                //to avoid a endless set-color loop, only set the color if it is different
                if let Some(current_color) = window.color(){

                match Color::from_hex(&color, hex_alpha_position) {
                    Ok(color) => if color != current_color{ window.set_color(color) },
                    Err(_) => log::debug!("Failed to parse color: {color}"),
                }
            }
            }),
        );

        imp.name_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |_: ColorFormatRow, name: String| {
                log::debug!("Changed name entry: {name}");
                //do not search for unnamed colors
                if name != pgettext(
                    "Information that no name for the color could be found",
                    "Not named",
                ) {

                //to avoid a endless set-color loop, only set the color if it is different
                if let Some(current_color) = window.color(){

                match color_names::color(&name.trim().to_lowercase(),
                        window.imp().settings.boolean("name-source-basic"),
                        window.imp().settings.boolean("name-source-extended"),
                        window.imp().settings.boolean("name-source-gnome-palette"),
                        window.imp().settings.boolean("name-source-xkcd")) {
                    Some(color) => if color != current_color { window.set_color(color) },
                    None => log::debug!("Failed to find color for name: {name}"),
                }
                }
            }
            }),
        );
    }
}
