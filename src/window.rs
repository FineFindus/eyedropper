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
use crate::widgets::history_item::HistoryItem;
use crate::widgets::palette_dialog::PaletteDialog;

mod imp {
    use std::cell::{Cell, RefCell};

    use crate::widgets;

    use super::*;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::CompositeTemplate;
    use once_cell::sync::OnceCell;

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
        pub color_button: TemplateChild<gtk::ColorDialogButton>,
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
        pub oklab_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub oklch_row: TemplateChild<widgets::color_format_row::ColorFormatRow>,
        #[template_child]
        pub history_list: TemplateChild<gtk::ListBox>,
        pub history: OnceCell<gio::ListStore>,
        pub settings: gio::Settings,
        pub color: Cell<Option<Color>>,
        pub portal_error: RefCell<Option<ashpd::Error>>,
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
                oklab_row: TemplateChild::default(),
                oklch_row: TemplateChild::default(),
                hwb_row: TemplateChild::default(),
                hcl_row: TemplateChild::default(),
                name_row: TemplateChild::default(),
                lms_row: TemplateChild::default(),
                history_list: TemplateChild::default(),
                history: Default::default(),
                settings: gio::Settings::new(APP_ID),
                color: Cell::new(None),
                portal_error: RefCell::new(None),
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
            Self::Type::bind_template_callbacks(klass);

            klass.install_action("win.show-toast", Some("(si)"), move |win, _, var| {
                if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                    win.show_toast(toast, adw::ToastPriority::__Unknown(i));
                }
            });

            klass.install_action("win.set-color", Some("s"), move |win, _, var| {
                let Some(Ok(color)) = var
                    .and_then(|v| v.get::<String>())
                    .map(|v| Color::from_hex(&v, AlphaPosition::None))
                else {
                    return;
                };
                win.set_color(color);
            });

            klass.install_action("win.remove-item", Some("s"), |win, _, var| {
                let Some(Ok(color)) = var
                    .and_then(|v| v.get::<String>())
                    .map(|v| Color::from_hex(&v, AlphaPosition::None))
                else {
                    return;
                };

                let Some(index) = win.history().find_with_equal_func(|item| {
                    item.downcast_ref::<HistoryObject>().unwrap().color() == color.into()
                }) else {
                    return;
                };

                win.history().remove(index);

                // if the removed item was the current color/first item, show the next color
                // otherwise the current (removed) color would still be shown
                // safe to unwrap as the user should only be able to click the remove option when
                // the list is showm, which is only the case for 2+ colors
                if index == 0 {
                    let next_color = win
                        .history()
                        .item(0)
                        .and_then(|item| {
                            item.downcast_ref::<HistoryObject>()
                                .map(|item| item.color())
                        })
                        .unwrap();
                    win.set_color(next_color.into());
                }
            });
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
            obj.order_formats();
            obj.load_visibility_settings();
            obj.set_stack();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        // Save window state on delete event
        fn close_request(&self) -> glib::Propagation {
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
    fn show_toast(&self, text: impl AsRef<str>, priority: adw::ToastPriority) {
        let toast = adw::Toast::new(text.as_ref());
        toast.set_priority(priority);
        self.imp()
            .toast_overlay
            .add_toast(adw::Toast::new(text.as_ref()));
    }

    /// The currently picked color, or `None` if the user hasn't picked one yet.
    fn color(&self) -> Option<Color> {
        self.imp().color.get()
    }

    /// Set the stack to either show the main page or the placeholder,
    /// depending on if a color is chosen.
    fn set_stack(&self) {
        self.imp()
            .stack
            .set_visible_child_name(self.color().map(|_| "main").unwrap_or("placeholder"));
    }

    /// Returns the history list store object.
    fn history(&self) -> &gio::ListStore {
        self.imp().history.get().expect("Failed to get history")
    }

    /// Clear the history by removing all items from the list.
    ///
    /// After clearing it, a toast will be shown with the options to 'undo' the clearing.
    pub fn clear_history(&self) {
        //clear history
        let history = self.history();
        let items = history.snapshot();

        //show toast to undo
        let toast = adw::Toast::builder()
            .title(gettext("Cleared history"))
            .button_label(gettext("Undo"))
            .priority(adw::ToastPriority::High)
            .build();

        toast.connect_button_clicked(
            glib::clone!(@weak self as window, @strong items => move |_toast| {
                window.history().extend_from_slice(&items);
                log::debug!("Undo clicked: {}", items.len());
            }),
        );

        history.remove_all();
        self.imp().toast_overlay.add_toast(toast);

        if let Some(color) = self.color() {
            let history_item = HistoryObject::new(color);
            self.history().insert(0, &history_item);
        }
    }

    /// Setup the history by setting up a model
    fn setup_history(&self) {
        // Create new model
        let model = gio::ListStore::new::<HistoryObject>();

        // Get state and set model
        self.imp()
            .history
            .set(model)
            .expect("Failed to set history model");

        // Wrap model with selection and pass it to the list view
        let selection_model = gtk::NoSelection::new(Some(self.history().clone()));
        self.imp().history_list.bind_model(
            Some(&selection_model),
            glib::clone!(@weak self as window => @default-panic, move |obj| {
                let history_object = obj.downcast_ref::<HistoryObject>().expect("The object is not of type `HistoryObject`.");
                let history_item = HistoryItem::new(history_object.color());
                history_item.upcast()
            }),
        );

        // Assure that the history list is only visible when it is supposed to
        self.set_history_list_visible(self.history());
        self.history().connect_items_changed(
            glib::clone!(@weak self as window => move |items, _, _, _| {
                window.set_history_list_visible(items);
            }),
        );
    }

    /// Assure that history is only visible
    /// if the number of items is greater than 0
    fn set_history_list_visible(&self, history: &gio::ListStore) {
        let visible = history.n_items() > 1;
        self.imp().history_list.set_visible(visible);
        self.action_set_enabled("app.clear_history", visible);
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

        let default_width = imp
            .settings
            .default_value("window-width")
            .expect("Failed to get width widow-width")
            .get::<i32>()
            .expect("Failed to get width i32");

        let default_height = imp
            .settings
            .default_value("window-height")
            .expect("Failed to get height window-height")
            .get::<i32>()
            .expect("Failed to get height i32");

        let width = imp.settings.int("window-width").max(default_width);
        let height = imp.settings.int("window-height").max(default_height);
        let is_maximized = imp.settings.boolean("is-maximized");
        log::debug!("Window Size: {}x{}", width, height);
        log::debug!("Maximized: {}", is_maximized);

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
                window.order_formats();
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

        //update precision
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
        settings.connect_changed(Some("custom-format-hunter-lab"), update_color.clone());
        settings.connect_changed(Some("custom-format-oklab"), update_color.clone());
        settings.connect_changed(Some("custom-format-oklch"), update_color);

        settings.connect_changed(
            Some("visible-formats"),
            glib::clone!(@weak self as window => move |_settings: &gio::Settings, _: &str| {
                window.order_formats();
            }),
        );

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
                window.imp().name_row.set_text(name.unwrap_or_else(|| pgettext(
                    "Information that no name for the color could be found",
                    "Not named",
                )));
            }
        });

        settings.connect_changed(Some("name-source-basic"), update_color_names.clone());
        settings.connect_changed(Some("name-source-extended"), update_color_names.clone());
        settings.connect_changed(Some("name-source-xkcd"), update_color_names.clone());

        let show_name_model = settings
            .get::<Vec<String>>("visible-formats")
            .contains(&"name".to_owned());
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
    fn order_formats(&self) {
        let imp = self.imp();
        let format_box = &imp.format_box;

        format_box
            .observe_children()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatRow>)
            .for_each(|row| format_box.remove(row));

        let order = imp.settings.get::<Vec<String>>("format-order");
        let visible = imp.settings.get::<Vec<String>>("visible-formats");
        log::debug!("Formats: {:?}", order);
        log::debug!("Visible: {:?}", visible);

        order
            .iter()
            .filter(|item| visible.contains(item))
            .filter_map(|item| self.map_format(item))
            .for_each(|child| {
                format_box.append(&child.get());
                child.set_visible(true);
            });
    }

    /// Returns a reference to the `input` named `ColorFormatRow`, or `None` if it could not be found.
    ///
    /// The checking is done case insensitive.
    ///
    /// # Examples
    ///
    /// ```
    /// if let Some(row) = self.map_format("rgb") {
    ///     row.set_text("RGB");
    /// }
    /// ```
    fn map_format(&self, item: &str) -> Option<&TemplateChild<ColorFormatRow>> {
        let imp = self.imp();
        match item.to_lowercase().as_str() {
            "hex" => Some(&imp.hex_row),
            "rgb" => Some(&imp.rgb_row),
            "hsl" => Some(&imp.hsl_row),
            "hsv" => Some(&imp.hsv_row),
            "cmyk" => Some(&imp.cmyk_row),
            "xyz" => Some(&imp.xyz_row),
            "cielab" => Some(&imp.cie_lab_row),
            "hwb" => Some(&imp.hwb_row),
            "hcl" => Some(&imp.hcl_row),
            "name" => Some(&imp.name_row),
            "lms" => Some(&imp.lms_row),
            "hunterlab" => Some(&imp.hunter_lab_row),
            "oklab" => Some(&imp.oklab_row),
            "oklch" => Some(&imp.oklch_row),
            _ => {
                log::error!("Failed to find format: {}", item);
                None
            }
        }
    }

    /// Opens a dialog with different palettes.
    ///
    /// When a palette is clicked it will be added to the history list.
    #[template_callback]
    fn open_palette_dialog(&self) {
        //safe to unwrap, if the user opens this dialog, the color button must be clicked
        let palette_dialog = PaletteDialog::new(self.color().expect("Failed to get current color"));
        palette_dialog.set_transient_for(Some(self));
        palette_dialog.present();

        //when a palette is chosen, add all colors of the palette in reverse order to the history
        palette_dialog.connect_closure(
            "palette-clicked",
            false,
            glib::closure_local!(@watch self as window => move |_: PaletteDialog, palette: String| {
                log::debug!("Palette: {palette}");

                palette
                .split_ascii_whitespace()
                .for_each(|slice|
                    if let Ok(color) = Color::from_hex(slice, AlphaPosition::None) {
                        window.set_color(color);
                    } else {
                        log::error!("Failed to parse color {}", slice);
                        window.show_toast(gettext("Failed to get palette color"), adw::ToastPriority::Normal);
                    }
                );
            }),
        );
    }

    /// Pick a color from the desktop using [ashpd].
    ///
    /// It will show a toast when failing to pick a color, for example when the user cancels the action.
    #[template_callback]
    pub fn pick_color(&self) {
        log::debug!("Picking a color using the color picker");
        let main_context = glib::MainContext::default();
        main_context.spawn_local(glib::clone!(@weak self as window => async move {

        let root = window.root().expect("Failed to get window root");
        let identifier = ashpd::WindowIdentifier::from_native(&root).await;
        let request = ashpd::desktop::screenshot::Color::request()
            .identifier(identifier)
            .send()
            .await;

        match request.and_then(|req| req.response()) {
            Ok(color) => {
                window.imp().portal_error.replace(None);
                window.set_color(Color::from(color));
            },
            Err(err) => {
                log::error!("{}", err);
                if !matches!(err, ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)) {
                    window.show_toast(gettext("Failed to pick a color"), adw::ToastPriority::Normal);
                    window.imp().portal_error.replace(Some(err));
                }
            },
        };
        }));
    }

    /// Set the current color to the given color.
    ///
    /// If the given color is different from the current color,
    /// it will be added to the history. If the history includes the given
    /// color, the preceding occurrence will be removed.
    pub fn set_color(&self, color: Color) {
        if self.color() != Some(color) {
            if self.history().n_items() > 0 {
                if let Some(i) = self.history().find_with_equal_func(|item| {
                    item.downcast_ref::<HistoryObject>().unwrap().color() == color.into()
                }) {
                    self.history().remove(i);
                }
            }

            let history_item = HistoryObject::new(color);
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

        imp.oklab_row.set_text(formatter.oklab());

        imp.oklch_row.set_text(formatter.oklch());
    }

    fn setup_callbacks(&self) {
        let imp = self.imp();

        imp.hex_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed hex entry: {color}");
                let hex_alpha_position = AlphaPosition::from(window.imp().settings.int("alpha-position") as u32);

                //to avoid a endless set-color loop, only set the color if it is different
                if let Some(current_color) = window.color() {
                    match Color::from_hex(&color, hex_alpha_position) {
                        Ok(color) => if color != current_color {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                    }
                }
            }),
        );

        imp.rgb_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed RGB entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_rgb(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.hsl_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed HSL entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_hsl_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.hsv_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed HSV entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_hsv_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.cmyk_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed CMYK entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_cmyk_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.xyz_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed XYZ entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_xyz_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.cie_lab_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed CIELab entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_cie_lab_string(&color,
                        Illuminant::from(window.imp().settings.int("cie-illuminants") as u32),
                        window.imp().settings.int("cie-standard-observer") == 1,
                    ) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.name_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, name: String| {
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
                    Some(color) => if color != current_color {
                        window.set_color(color);
                        format_row.show_success();
                     },
                    None => {
                        log::debug!("Failed to find color for name: {name}");
                        format_row.show_error();
                    },
                }
            }
        }}),
        );

        imp.hwb_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed HWB entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_hwb_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.hcl_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed HCL entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_hcl_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.lms_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed LMS entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_lms_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.hunter_lab_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed Hunter Lab entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_hunter_lab_string(&color,
                        Illuminant::from(window.imp().settings.int("cie-illuminants") as u32),
                        window.imp().settings.int("cie-standard-observer") == 1) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.oklab_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed Oklab entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_oklab_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );

        imp.oklch_row.connect_closure(
            "text-edited",
            false,
            glib::closure_local!(@watch self as window => move |format_row: ColorFormatRow, color: String| {
                log::debug!("Changed Oklch entry: {color}");

                if let Some(current_color) = window.color(){
                    match Color::from_oklch_string(&color) {
                        Ok(color) if color != current_color => {
                            window.set_color(color);
                            format_row.show_success();
                        },
                        Err(_) => {
                            log::debug!("Failed to parse color: {color}");
                            format_row.show_error();
                        },
                        _ => {}
                    }
                }
            }),
        );
    }
}
