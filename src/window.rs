use gettextrs::gettext;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use gtk::{prelude::*, ColorButton};

use crate::application::App;
use crate::config::{APP_ID, PROFILE};
use crate::model::color::{AlphaPosition, Color};
use crate::model::history::HistoryObject;
use crate::utils;
use crate::widgets::color_model_entry::ColorModelEntry;
use crate::widgets::hex_entry::HexEntry;

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
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub format_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub color_button: TemplateChild<gtk::ColorButton>,
        #[template_child]
        pub color_picker_button: TemplateChild<gtk::Button>,
        #[template_child]
        pub hex_entry: TemplateChild<widgets::hex_entry::HexEntry>,
        #[template_child]
        pub rgb_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub hsl_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub hsv_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub cmyk_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub xyz_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub cie_lab_entry: TemplateChild<widgets::color_model_entry::ColorModelEntry>,
        #[template_child]
        pub history_list: TemplateChild<gtk::ListBox>,
        pub history: RefCell<Option<gio::ListStore>>,
        pub settings: gio::Settings,
        pub color: RefCell<Color>,
    }

    impl Default for AppWindow {
        fn default() -> Self {
            Self {
                headerbar: TemplateChild::default(),
                color_button: TemplateChild::default(),
                color_picker_button: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                format_box: TemplateChild::default(),
                hex_entry: TemplateChild::default(),
                rgb_entry: TemplateChild::default(),
                hsl_entry: TemplateChild::default(),
                hsv_entry: TemplateChild::default(),
                cmyk_entry: TemplateChild::default(),
                xyz_entry: TemplateChild::default(),
                cie_lab_entry: TemplateChild::default(),
                history_list: TemplateChild::default(),
                history: Default::default(),
                settings: gio::Settings::new(APP_ID),
                color: RefCell::new(Color::rgba(0, 0, 0, 0)),
            }
        }
    }

    #[gtk::template_callbacks]
    impl AppWindow {
        #[template_callback]
        fn on_color_picker_button_clicked(&self) {
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
                log::debug!("Running in DEVEL mode");
                obj.add_css_class("devel");
            }

            // Load latest window state
            obj.load_window_size();
            obj.setup_history();
            obj.load_settings();
            obj.setup_callbacks();

            obj.set_order();
        }
    }

    impl WidgetImpl for AppWindow {}
    impl WindowImpl for AppWindow {
        // Save window state on delete event
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            //save current window size
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
        window.clear_history();
        window
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
    fn clear_history(&self) {
        //clear history
        let history = self.history();
        history.remove_all();
        //clear added palettes
        utils::add_palette(
            &self.imp().color_button.get(),
            gtk::Orientation::Horizontal,
            0,
            None,
        );

        //re-add palette of the current color
        utils::add_palette(
            &self.imp().color_button.get(),
            gtk::Orientation::Horizontal,
            10,
            Some(
                &self
                    .imp()
                    .color
                    .borrow()
                    .generate_palette(10, 0.1)
                    .into_iter()
                    .map(gtk::gdk::RGBA::from)
                    .collect::<Vec<gtk::gdk::RGBA>>(),
            ),
        );
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
                let hist = window.create_history_item(history_object);
                hist.upcast()
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
        //create a button so that keyboard focus and selecting works.
        //there seem to be a bug, which makes the focus on the button invisible, no idea on how to fix it though
        let color_button = gtk::Button::builder()
            .child(
                &gtk::ColorButton::builder()
                    .rgba(&history_object.color().into())
                    .can_focus(false)
                    .build(),
            )
            .css_name("history-item")
            .build();

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

    fn load_settings(&self) {
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

        //update hex entry with new alpha position
        settings.connect_changed(
            Some("alpha-position"),
            glib::clone!(@weak self as window => move |settings, _| {
                log::debug!("Updating AlphaPosition");
                let color = *window.imp().color.borrow();
                let hex_alpha_position = AlphaPosition::from(settings.int("alpha-position") as u32);
                //update hex to show alpha
                window.imp().hex_entry.set_color(color.to_hex_string(hex_alpha_position));
                //update rgb to switch to argb/rgba
                window.imp().rgb_entry.set_color(match hex_alpha_position {
                    AlphaPosition::None => {
                        format!("rgb({}, {}, {})", color.red, color.green, color.blue)
                    }
                    AlphaPosition::Start => {
                        format!(
                            "argb({}, {}, {}, {})",
                            color.alpha, color.red, color.green, color.blue
                        )
                    }
                    AlphaPosition::End => {
                        format!(
                            "rgba({}, {}, {}, {})",
                            color.red, color.green, color.blue, color.alpha
                        )
                    }
                });
            }),
        );

        //first setup when loading
        let show_hex_model = settings.boolean("show-hex-model");
        imp.hex_entry.set_visible(show_hex_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-hex-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_hex_model = settings.boolean("show-hex-model");
            window.imp().hex_entry.set_visible(show_hex_model);
            }),
        );

        //first setup when loading
        let show_rgb_model = settings.boolean("show-rgb-model");
        imp.rgb_entry.set_visible(show_rgb_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-rgb-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_rgb_model = settings.boolean("show-rgb-model");
            window.imp().rgb_entry.set_visible(show_rgb_model);
            }),
        );

        //first setup when loading
        let show_hsl_model = settings.boolean("show-hsl-model");
        imp.hsl_entry.set_visible(show_hsl_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-hsl-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_hsl_model = settings.boolean("show-hsl-model");
            window.imp().hsl_entry.set_visible(show_hsl_model);
            }),
        );

        //first setup when loading
        let show_hsv_model = settings.boolean("show-hsv-model");
        imp.hsv_entry.set_visible(show_hsv_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-hsv-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_hsv_model = settings.boolean("show-hsv-model");
            window.imp().hsv_entry.set_visible(show_hsv_model);
            }),
        );

        //first setup when loading
        let show_cmyk_model = settings.boolean("show-cmyk-model");
        imp.cmyk_entry.set_visible(show_cmyk_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-cmyk-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_cmyk_model = settings.boolean("show-cmyk-model");
            window.imp().cmyk_entry.set_visible(show_cmyk_model);
            }),
        );

        //first setup when loading
        let show_xyz_model = settings.boolean("show-xyz-model");
        imp.xyz_entry.set_visible(show_xyz_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-xyz-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_xyz_model = settings.boolean("show-xyz-model");
            window.imp().xyz_entry.set_visible(show_xyz_model);
            }),
        );

        //first setup when loading
        let show_cie_lab_model = settings.boolean("show-cie-lab-model");
        imp.cie_lab_entry.set_visible(show_cie_lab_model);
        //refresh when settings change
        settings.connect_changed(
            Some("show-cie-lab-model"),
            glib::clone!(@weak self as window => move |settings, _| {
            let show_cie_lab_model = settings.boolean("show-cie-lab-model");
            window.imp().cie_lab_entry.set_visible(show_cie_lab_model);
            }),
        );
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
            .filter_map(Cast::downcast_ref::<ColorModelEntry>)
            .for_each(|entry| format_box.remove(entry));

        //remove hex entry
        format_box.remove(&imp.hex_entry.get());

        //parse current order
        let order = imp.settings.get::<Vec<String>>("format-order");

        //insert items in order
        for item in order {
            let child = match item.to_lowercase().as_str() {
                "hex" => {
                    //append the hex entry directly, as it is not a ColorModelEntry
                    format_box.append(&imp.hex_entry.get());
                    continue;
                }
                "rgb" => &imp.rgb_entry,
                "hsl" => &imp.hsl_entry,
                "hsv" => &imp.hsv_entry,
                "cmyk" => &imp.cmyk_entry,
                "xyz" => &imp.xyz_entry,
                "cielab" => &imp.cie_lab_entry,
                _ => {
                    log::error!("Failed to find format: {}", item);
                    continue;
                }
            };

            format_box.append(&child.get());
        }
    }
    /// Pick a color from the desktop using [ashpd].
    ///
    /// It will show a toast when failing to pick a color, for example when the user cancels the action.
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

    ///Update the current color to the given color.
    /// The old color will be added to the history list.
    pub fn set_color(&self, color: Color) {
        //only update when necessary, to avoid infinite loop
        if *self.imp().color.borrow() != color {
            //append previous color to history
            let history_item = HistoryObject::new(*self.imp().color.borrow());
            self.history().insert(0, &history_item);

            log::info!(
                "Changing Hex Color: {:?}",
                color.to_hex_string(crate::model::color::AlphaPosition::End)
            );
            let imp = self.imp();
            imp.color.replace(color);

            imp.color_button.set_rgba(&color.into());

            let btn: ColorButton = imp.color_button.get();

            //clear current palette set
            utils::add_palette(&btn, gtk::Orientation::Horizontal, 0, None);

            //generate a palette by shading and tinting the color
            let colors = color
                .generate_palette(10, 0.1)
                .into_iter()
                .map(gtk::gdk::RGBA::from)
                .collect::<Vec<gtk::gdk::RGBA>>();

            //add new palettes
            utils::add_palette(&btn, gtk::Orientation::Horizontal, 10, Some(&colors));

            //add palettes of the last 3 history items
            self.history()
                .snapshot()
                .iter()
                .take(3)
                .filter_map(Cast::downcast_ref::<HistoryObject>)
                .for_each(|item| {
                    //generate a palette by shading and tinting the color
                    let colors = item
                        .color()
                        .generate_palette(10, 0.1)
                        .into_iter()
                        .map(gtk::gdk::RGBA::from)
                        .collect::<Vec<gtk::gdk::RGBA>>();

                    //add new palettes
                    utils::add_palette(&btn, gtk::Orientation::Horizontal, 10, Some(&colors));
                });

            let hex_alpha_position = AlphaPosition::from(imp.settings.int("alpha-position") as u32);

            imp.hex_entry
                .set_color(color.to_hex_string(hex_alpha_position));

            imp.rgb_entry.set_color(match hex_alpha_position {
                AlphaPosition::None => {
                    format!("rgb({}, {}, {})", color.red, color.green, color.blue)
                }
                AlphaPosition::Start => {
                    format!(
                        "argb({}, {}, {}, {})",
                        color.alpha, color.red, color.green, color.blue
                    )
                }
                AlphaPosition::End => {
                    format!(
                        "rgba({}, {}, {}, {})",
                        color.red, color.green, color.blue, color.alpha
                    )
                }
            });

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

            let xyz = color.to_xyz();
            imp.xyz_entry
                .set_color(format!("XYZ({:.3}, {:.3}, {:.3})", xyz.0, xyz.1, xyz.2));

            let cie_lab = color.to_cie_lab();
            imp.cie_lab_entry.set_color(format!(
                "CIELAB({:.2}, {:.2}, {:.2})",
                cie_lab.0, cie_lab.1, cie_lab.2
            ));
        }
    }

    fn setup_callbacks(&self) {
        //load imp
        let imp = self.imp();

        //connect color button to selected color
        imp.color_button.connect_color_set(
            glib::clone!(@weak self as window => move |color_button| {
                window.set_color(color_button.rgba().into());
            }),
        );

        //show a toast when copying values
        let show_toast_closure = glib::closure_local!(@watch self as window => move |_: ColorModelEntry, text: String| {
            //Translators: Do not replace the {}. These are used as placeholders for the copied values
            window.show_toast(&gettext("Copied: “{}”").replace("{}", &text));
        });

        imp.hex_entry.connect_closure(
            "copied-color",
            false,
            glib::closure_local!(@watch self as window => move |_: HexEntry, text: String| {
                window.show_toast(&gettext("Copied: “{}”").replace("{}", &text));
            }),
        );
        imp.rgb_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.hsl_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.hsv_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.cmyk_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.xyz_entry
            .connect_closure("copied-color", false, show_toast_closure.clone());
        imp.cie_lab_entry
            .connect_closure("copied-color", false, show_toast_closure);

        imp.hex_entry.connect_closure(
            "color-changed",
            false,
            glib::closure_local!(@watch self as window => move |_: HexEntry, color: String| {
                log::debug!("Changed hex entry: {color}");
                let hex_alpha_position = AlphaPosition::from(window.imp().settings.int("alpha-position") as u32);
                match Color::from_hex(&color, hex_alpha_position) {
                    Ok(color) => window.set_color(color),
                    Err(_) => log::debug!("Failed to parse color: {color}"),
                }
            }),
        );
    }

    /// Shows a basic toast with the given text.
    fn show_toast(&self, text: &str) {
        self.imp().toast_overlay.add_toast(&adw::Toast::new(text));
    }
}
