use adw::traits::ActionRowExt;
use gettextrs::gettext;
use gettextrs::pgettext;
use gtk::gdk;
use gtk::gio;
use gtk::gio::ListStore;
use gtk::gio::Menu;
use gtk::gio::MenuItem;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::Switch;

use crate::color::color::Color;
use crate::color::observer::Observer;
use crate::utils;

use self::color_format::ColorFormatObject;

mod imp {

    use std::cell::RefCell;

    use adw::subclass::{prelude::PreferencesWindowImpl, window::AdwWindowImpl};
    use gtk::gio;

    use crate::config;

    use super::*;

    // Object holding the state
    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences.ui")]
    pub struct PreferencesWindow {
        pub settings: gtk::gio::Settings,
        #[template_child()]
        pub alpha_pos_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub observer_degree_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub observer_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub format_list: TemplateChild<gtk::ListBox>,
        pub formats: RefCell<Option<gio::ListStore>>,
    }

    // The central trait for subclassing a GObject
    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        // `NAME` needs to match `class` attribute of template
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                alpha_pos_box: TemplateChild::default(),
                observer_degree_box: TemplateChild::default(),
                observer_box: TemplateChild::default(),
                format_list: TemplateChild::default(),
                formats: Default::default(),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    // Trait shared by all GObjects
    impl ObjectImpl for PreferencesWindow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.setup_format_list();
            obj.setup_settings();
            obj.add_options();
        }
    }

    impl WidgetImpl for PreferencesWindow {}
    impl WindowImpl for PreferencesWindow {}
    impl AdwWindowImpl for PreferencesWindow {}
    impl PreferencesWindowImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
    @extends gtk::Widget, gtk::Window, adw::Window, adw::PreferencesWindow;
}

#[gtk::template_callbacks]
impl PreferencesWindow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>(&[]).expect("Failed to create a PreferencesWindow")
    }

    fn setup_settings(&self) {
        let imp = self.imp();

        imp.settings
            .bind("alpha-position", &*imp.alpha_pos_box, "selected")
            .build();

        imp.settings
            .bind("color-observer", &*imp.observer_box, "selected")
            .build();

        imp.settings
            .bind("ten-degree-observer", &*imp.observer_degree_box, "selected")
            .build();
    }

    /// Resets the current order by resetting the setting and repopulating the list.
    #[template_callback]
    fn on_reset_pressed(&self, _button: &gtk::Button) {
        log::debug!("Resetting order");
        self.formats().remove_all();
        self.imp().settings.reset("format-order");
        self.add_options();
    }

    /// Shows a dialog letting the use choose which name sets should be used.
    #[template_callback]
    fn on_name_row_activated(&self, _row: &adw::ActionRow) {
        let list = gtk::ListBox::builder()
            .margin_top(12)
            .margin_start(12)
            .margin_end(12)
            .margin_bottom(12)
            .css_classes(vec!["boxed-list".to_string()])
            .build();

        list.append(&self.name_set_row(
            &pgettext(
                "Name of the basic color keyword set from https://www.w3.org/TR/css-color-3/#html4",
                "Basic",
            ),
            &gettext("Show color names from the w3c basic color keyword set"),
            "name-source-basic",
        ));
        list.append(&self.name_set_row(
            &pgettext(
                "Name of the extended color keyword set from https://www.w3.org/TR/css-color-3/#svg-color",
                "Extended",
            ),
            &gettext("Show color names from the w3c extended color keyword  set"),
            "name-source-extended",
        ));
        list.append(&self.name_set_row(
            &pgettext("Name of the color set from the xkcd color survey", "xkcd"),
            &gettext("Show color names from the xkcd color survey"),
            "name-source-xkcd",
        ));

        let dialog = gtk::Dialog::builder()
            .transient_for(self)
            .modal(true)
            .child(&list)
            .build();
        dialog.show();
    }

    /// Build an ActionRow for the name setting.
    fn name_set_row(&self, title: &str, subtitle: &str, source: &str) -> adw::ActionRow {
        let switch = gtk::Switch::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();

        self.imp().settings.bind(source, &switch, "state").build();

        let row = adw::ActionRow::builder()
            .title(title)
            .subtitle(subtitle)
            .activatable_widget(&switch)
            .build();
        row.add_suffix(&switch);
        row
    }

    /// Returns the formats list store object.
    fn formats(&self) -> gio::ListStore {
        // Get state
        self.imp()
            .formats
            .borrow()
            .clone()
            .expect("Could not get current formats.")
    }

    fn format_order_list(&self) -> Vec<String> {
        self.formats()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatObject>)
            .map(|format| format.identifier())
            .collect()
    }

    /// Assure that formats is only visible
    /// if the number of items is greater than 0
    fn set_format_list_visible(&self, formats: &gio::ListStore) {
        self.imp().format_list.set_visible(formats.n_items() > 0);
    }

    ///Setup the format list
    fn setup_format_list(&self) {
        // Create new model
        let model = ListStore::new(ColorFormatObject::static_type());

        // Get state and set model
        self.imp().formats.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = gtk::NoSelection::new(Some(&self.formats()));
        self.imp().format_list.bind_model(
            Some(&selection_model),
            glib::clone!(@weak self as widget => @default-panic, move |obj| {
                let formats_object = obj.downcast_ref().expect("The object is not of type `ColorFormatObject`.");
                let hist = widget.create_format_row(formats_object);
                hist.upcast()
            }),
        );

        // Assure that the formats list is only visible when it is supposed to
        self.set_format_list_visible(&self.formats());
        self.formats().connect_items_changed(
            glib::clone!(@weak self as window => move |items, _, _, _| {
                window.set_format_list_visible(items);
            }),
        );
    }

    /// Returns a new format row, consisting of the format title, a format example, as well as a switch and a drag handle.
    /// The row will be set up to support drag and drop.
    fn create_format_row(&self, item: &ColorFormatObject) -> adw::ActionRow {
        let switch = Switch::builder()
            .valign(gtk::Align::Center)
            .can_focus(false)
            .build();

        self.imp()
            .settings
            .bind(&item.settings_name(), &switch, "state")
            .build();

        let row = adw::ActionRow::builder()
            .title(&item.label())
            .subtitle(&item.example())
            .activatable_widget(&switch)
            .build();

        row.add_suffix(&switch);
        row.add_suffix(
            &gtk::Separator::builder()
                .orientation(gtk::Orientation::Vertical)
                .margin_bottom(12)
                .margin_top(12)
                .build(),
        );

        //create actions for accessibility reasons
        let actions = gtk::gio::SimpleActionGroup::new();
        let up_action = gio::SimpleAction::new("move-up", None);
        up_action.connect_activate(
            glib::clone!(@weak self as window, @weak item => move |_, _| {
                if let Some(index) = window.formats().find(&item) {
                    log::debug!("Moving {} up", item.label());
                    window.formats().remove(index);
                    window.formats().insert(index.saturating_sub(1), &item);
                }
            }),
        );
        actions.add_action(&up_action);

        let down_action = gio::SimpleAction::new("move-down", None);
        down_action.connect_activate(
            glib::clone!(@weak self as window, @weak item => move |_, _| {
                if let Some(index) = window.formats().find(&item) {
                    log::debug!("Moving {} down", item.label());
                    window.formats().remove(index);
                    //index should not be larger than the largest index
                    window.formats().insert((index + 1).min(window.formats().n_items()), &item);
                }
            }),
        );
        actions.add_action(&down_action);

        let menu = Menu::new();
        let up_item = MenuItem::new(Some(&gettext("Move Up")), Some("row.move-up"));
        menu.append_item(&up_item);
        let down_item = MenuItem::new(Some(&gettext("Move Down")), Some("row.move-down"));
        menu.append_item(&down_item);

        let menu_button = gtk::MenuButton::builder()
            .valign(gtk::Align::Center)
            .icon_name("view-more-symbolic")
            .menu_model(&menu)
            .build();
        menu_button.add_css_class("flat");
        menu_button.insert_action_group("row", Some(&actions));

        row.add_suffix(&menu_button);

        //drag handle
        let handle = gtk::Image::from_icon_name("list-drag-handle-symbolic");
        handle.add_css_class("drag-handle");
        row.add_prefix(&handle);

        let drag = gtk::DragSource::builder()
            .name("preferences-drag-format")
            .actions(gtk::gdk::DragAction::MOVE)
            .build();

        drag.connect_prepare(
            glib::clone!(@weak self as list, @weak item, @weak row as widget => @default-return None, move |source, _, _| {
                let icon = gtk::WidgetPaintable::new(Some(&widget));
                source.set_icon(Some(&icon), 0, 0);
                Some(gdk::ContentProvider::for_value(&item.to_value()))
            }),
        );
        row.add_controller(&drag);

        let drop_target = gtk::DropTarget::builder()
            .name("preferences-drag-format")
            .propagation_phase(gtk::PropagationPhase::Capture)
            .actions(gtk::gdk::DragAction::MOVE)
            .build();

        drop_target.set_types(&[ColorFormatObject::static_type()]);

        drop_target.connect_drop(glib::clone!(@weak self as widget, @weak item => @default-return false, move |_, value, _, _| {

            let value = value.get::<ColorFormatObject>().expect("Failed to get index value");

            if item == value {
                return  false;
            }

            //remove dragged row
            match widget.formats().find(&value) {
                Some(source_index) => {
                    widget.formats().remove(source_index);

                    match widget.formats().find(&item) {
                        Some(target_index) => {
                            if target_index >= source_index {
                                widget.formats().insert(target_index + 1, &value);
                            } else {
                                widget.formats().insert(target_index, &value);
                            }

                            //update settings with new order
                            match widget.imp().settings.set("format-order", &widget.format_order_list()) {
                                Ok(_) => {},
                                Err(err) => log::error!("Failed to save format-order: {}", err)
                            }
                        },
                        None => log::error!("Failed to find index for {:?}", item)

                    }
                },
                None => log::error!("Failed to find index for {:?}", value),
            }
            true
        }));
        row.add_controller(&drop_target);
        row
    }

    fn add_options(&self) {
        let color = Color::rgb(46, 52, 64);

        let mut order = self.imp().settings.get::<Vec<String>>("format-order");
        log::debug!("Order: {:?}", order);

        let default_order = self
            .imp()
            .settings
            .default_value("format-order")
            .expect("Failed to get default format-order");

        //It is theoretically possible to remove formats from the settings, so they would not show up
        //on the page. I couldn't find any docs about what happens when the defaults are updated, which happens whenever
        //a new format is added, so we just manually check if all formats are in the saved setting
        for (index, item) in default_order
            .array_iter_str()
            .expect("Failed to get default format-order array")
            .enumerate()
        {
            if !order.contains(&item.to_owned()) {
                log::debug!("Saved order does not contain {} at index {}", item, index);
                order.insert(index, item.to_owned());
                //override previously saved order
                match self
                    .imp()
                    .settings
                    .set("format-order", &self.format_order_list())
                {
                    Ok(_) => {}
                    Err(err) => log::error!("Failed to save format-order: {}", err),
                }
            }
        }

        log::debug!("Order with new items: {:?}", order);

        for item in order {
            let format = match item.to_lowercase().as_str() {
                "hex" => ColorFormatObject::new(
                    item,
                    gettext("Hex-Code"),
                    color.to_hex_string(crate::color::color::AlphaPosition::None),
                    "show-hex-model",
                ),
                "rgb" => ColorFormatObject::new(
                    item,
                    gettext("RGB"),
                    color.to_rgb_string(crate::color::color::AlphaPosition::None),
                    "show-rgb-model",
                ),
                "hsl" => ColorFormatObject::new(
                    item,
                    gettext("HSL"),
                    color.to_hsl_string(crate::color::color::AlphaPosition::None),
                    "show-hsl-model",
                ),
                "hsv" => {
                    let hsv = color.to_hsv();
                    ColorFormatObject::new(
                        item,
                        gettext("HSV"),
                        format!("hsv({}, {}%, {}%)", hsv.0, hsv.1, hsv.2),
                        "show-hsv-model",
                    )
                }
                "cmyk" => {
                    let cmyk = color.to_cmyk();
                    ColorFormatObject::new(
                        item,
                        gettext("CMYK"),
                        format!("cmyk({}%, {}%, {}%, {}%)", cmyk.0, cmyk.1, cmyk.2, cmyk.3),
                        "show-cmyk-model",
                    )
                }
                "xyz" => {
                    let xyz = color.to_xyz();
                    ColorFormatObject::new(
                        item,
                        gettext("XYZ"),
                        format!("XYZ({:.3}, {:.3}, {:.3})", xyz.0, xyz.1, xyz.2),
                        "show-xyz-model",
                    )
                }
                "cielab" => {
                    let cie_lab = color.to_cie_lab(Observer::D65, true);
                    ColorFormatObject::new(
                        item,
                        gettext("CIELAB"),
                        format!(
                            "CIELAB({:.2}, {:.2}, {:.2})",
                            cie_lab.0, cie_lab.1, cie_lab.2
                        ),
                        "show-cie-lab-model",
                    )
                }
                "hwb" => {
                    let hwb = color.to_hwb();
                    ColorFormatObject::new(
                        item,
                        gettext("HWB"),
                        format!(
                            "hwb({}, {}%, {}%)",
                            hwb.0,
                            utils::round_percent(hwb.1),
                            utils::round_percent(hwb.2)
                        ),
                        "show-hwb-model",
                    )
                }
                "hcl" => {
                    let hcl = color.to_hcl(Observer::D65, true);
                    ColorFormatObject::new(
                        item,
                        gettext("CIELCh / HCL"),
                        format!("lch({:.2}, {:.2}, {:.2})", hcl.2, hcl.1, hcl.0),
                        "show-hcl-model",
                    )
                }
                "name" => ColorFormatObject::new(
                    item,
                    gettext("Name"),
                    String::from("red"),
                    "show-color-name",
                ),
                _ => {
                    log::error!("Failed to find format: {item}");
                    continue;
                }
            };

            self.formats().append(&format);
        }
    }
}

mod color_format {
    use glib::{Object, ObjectExt};

    mod imp {

        use std::cell::RefCell;

        use glib::{
            subclass::{prelude::ObjectImpl, types::ObjectSubclass},
            ParamSpec, ParamSpecString, ToValue, Value,
        };
        use once_cell::sync::Lazy;

        // Object holding the state
        #[derive(Debug, Default)]
        pub struct ColorFormatObject {
            pub identifier: RefCell<String>,
            pub label: RefCell<String>,
            pub example: RefCell<String>,
            pub settings_name: RefCell<String>,
        }

        // The central trait for subclassing a GObject
        #[glib::object_subclass]
        impl ObjectSubclass for ColorFormatObject {
            const NAME: &'static str = "ColorFormatObject";
            type Type = super::ColorFormatObject;
        }

        // Trait shared by all GObjects
        impl ObjectImpl for ColorFormatObject {
            fn properties() -> &'static [ParamSpec] {
                static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                    vec![
                        ParamSpecString::builder("identifier").build(),
                        ParamSpecString::builder("label").build(),
                        ParamSpecString::builder("example").build(),
                        ParamSpecString::builder("settings-name").build(),
                    ]
                });
                PROPERTIES.as_ref()
            }

            fn set_property(
                &self,
                _obj: &Self::Type,
                _id: usize,
                value: &Value,
                pspec: &ParamSpec,
            ) {
                match pspec.name() {
                    "identifier" => {
                        let input_value = value.get::<String>().unwrap();
                        self.identifier.replace(input_value);
                    }
                    "label" => {
                        let input_value = value.get::<String>().unwrap();
                        self.label.replace(input_value);
                    }
                    "example" => {
                        let input_value = value.get::<String>().unwrap();
                        self.example.replace(input_value);
                    }
                    "settings-name" => {
                        let input_value = value.get::<String>().unwrap();
                        self.settings_name.replace(input_value);
                    }
                    _ => unimplemented!(),
                }
            }

            fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
                match pspec.name() {
                    "identifier" => self.identifier.borrow().to_value(),
                    "label" => self.label.borrow().to_value(),
                    "example" => self.example.borrow().to_value(),
                    "settings-name" => self.settings_name.borrow().to_value(),
                    _ => unimplemented!(),
                }
            }
        }
    }

    glib::wrapper! {
        pub struct ColorFormatObject(ObjectSubclass<imp::ColorFormatObject>);
    }

    impl ColorFormatObject {
        pub fn new(identifier: String, label: String, format: String, settings_name: &str) -> Self {
            Object::new(&[
                ("identifier", &identifier),
                ("label", &label),
                ("example", &format),
                ("settings-name", &settings_name),
            ])
            .expect("Failed to create `ColorFormatObject`.")
        }

        pub fn identifier(&self) -> String {
            self.property("identifier")
        }

        pub fn label(&self) -> String {
            self.property("label")
        }

        pub fn example(&self) -> String {
            self.property("example")
        }

        pub fn settings_name(&self) -> String {
            self.property("settings_name")
        }
    }
}
