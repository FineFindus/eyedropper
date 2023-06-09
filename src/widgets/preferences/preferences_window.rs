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

use crate::colors::color::Color;
use crate::colors::formatter::ColorFormatter;

use super::color_format::ColorFormatObject;
use super::name_set_dialog::NameSourcesDialog;

mod imp {

    use std::cell::RefCell;

    use adw::subclass::{prelude::PreferencesWindowImpl, window::AdwWindowImpl};
    use gtk::gio;

    use crate::{config, widgets::preferences::custom_format_row::CustomFormatRow};

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences/preferences.ui")]
    pub struct PreferencesWindow {
        pub settings: gtk::gio::Settings,
        #[template_child()]
        pub alpha_pos_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub standard_observer_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub precision_spin_button: TemplateChild<gtk::SpinButton>,
        #[template_child()]
        pub cie_illuminants_box: TemplateChild<gtk::DropDown>,
        // this exist only to load the CustomFormatRow, otherwise it would crash
        #[template_child()]
        pub _custom_format: TemplateChild<CustomFormatRow>,
        #[template_child()]
        pub order_list: TemplateChild<gtk::ListBox>,
        pub format_order: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                alpha_pos_box: TemplateChild::default(),
                standard_observer_box: TemplateChild::default(),
                cie_illuminants_box: TemplateChild::default(),
                precision_spin_button: TemplateChild::default(),
                _custom_format: TemplateChild::default(),
                order_list: TemplateChild::default(),
                format_order: Default::default(),
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

    impl ObjectImpl for PreferencesWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_order_list();
            obj.setup_settings();
            obj.populate_formats();
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
        glib::Object::new::<Self>()
    }

    fn setup_settings(&self) {
        let imp = self.imp();

        imp.settings
            .bind("alpha-position", &*imp.alpha_pos_box, "selected")
            .build();

        imp.settings
            .bind("cie-illuminants", &*imp.cie_illuminants_box, "selected")
            .build();

        imp.settings
            .bind(
                "cie-standard-observer",
                &*imp.standard_observer_box,
                "selected",
            )
            .build();

        imp.settings
            .bind("precision-digits", &*imp.precision_spin_button, "value")
            .build();
    }

    /// Resets the current order by resetting the setting and repopulating the list.
    #[template_callback]
    fn on_reset_pressed(&self, _button: &gtk::Button) {
        log::debug!("Resetting order");
        self.formats().remove_all();
        self.imp().settings.reset("format-order");
        self.populate_formats();
    }

    /// Shows a dialog letting the use choose which name sets should be used.
    #[template_callback]
    fn on_name_row_activated(&self, _row: &adw::ActionRow) {
        let dialog = NameSourcesDialog::new();
        dialog.set_transient_for(Some(self));
        dialog.set_visible(true);
    }

    /// Returns the formats list store object.
    fn formats(&self) -> gio::ListStore {
        self.imp()
            .format_order
            .borrow()
            .clone()
            .expect("Could not get current formats.")
    }

    /// Saves the current format order to the settings.
    fn save_format_order(&self) {
        let formats = self
            .formats()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatObject>)
            .map(ColorFormatObject::identifier)
            .collect::<Vec<String>>();

        self.imp()
            .settings
            .set("format-order", &formats)
            .expect("Failed to save format-order: {}");
    }

    /// Assure that formats is only visible
    /// if the number of items is greater than 0
    fn set_order_list_visible(&self, formats: &gio::ListStore) {
        self.imp().order_list.set_visible(formats.n_items() > 0);
    }

    ///Setup the format list
    fn setup_order_list(&self) {
        // Create new model
        let model = ListStore::new(ColorFormatObject::static_type());

        // Get state and set model
        self.imp().format_order.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = gtk::NoSelection::new(Some(self.formats()));
        self.imp().order_list.bind_model(
            Some(&selection_model),
            glib::clone!(@weak self as widget => @default-panic, move |obj| {
                let formats_object = obj.downcast_ref().expect("The object is not of type `ColorFormatObject`.");
                let hist = widget.create_format_row(formats_object);
                hist.upcast()
            }),
        );

        // Assure that the formats list is only visible when it is supposed to
        self.set_order_list_visible(&self.formats());
        self.formats().connect_items_changed(
            glib::clone!(@weak self as window => move |items, _, _, _| {
                window.set_order_list_visible(items);
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
            .bind("visible-formats", item, "visible")
            .mapping(
                glib::clone!(@weak item => @default-return None, move |value, _variant| {
                    let visible = value.get::<Vec<String>>().unwrap_or(Vec::with_capacity(0));
                    Some(visible.contains(&item.identifier()).to_value())
                }),
            )
            .set_mapping(
                glib::clone!(@weak self as window, @weak item => @default-return None, move |value, _variant| {
                    let active = value.get::<bool>().expect("Failed to get bool from switch active property");
                    let mut visible_formats= window.imp().settings.get::<Vec<String>>("visible-formats");

                    if active {
                        visible_formats.push(item.identifier());
                    } else {
                        visible_formats.retain(|format| format != &item.identifier());
                    }

                    Some(visible_formats.to_variant())
                }),
            )
            .build();
        item.bind_property("visible", &switch, "active")
            .bidirectional()
            .sync_create()
            .build();

        let row = adw::ActionRow::builder()
            .title(item.label())
            .subtitle(item.example())
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
                    window.save_format_order();
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
                    window.save_format_order();
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
        let drag_handle = gtk::Image::from_icon_name("list-drag-handle-symbolic");
        drag_handle.add_css_class("drag-handle");
        row.add_prefix(&drag_handle);

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
        row.add_controller(drag);

        let drop_target = gtk::DropTarget::builder()
            .name("preferences-drag-format")
            .propagation_phase(gtk::PropagationPhase::Capture)
            .actions(gtk::gdk::DragAction::MOVE)
            .build();

        drop_target.set_types(&[ColorFormatObject::static_type()]);

        drop_target.connect_drop(glib::clone!(@weak self as widget, @weak item => @default-return false, move |_, value, _, _| {

            let value = value.get::<ColorFormatObject>().expect("Failed to get index value");

            if item == value {
                return false;
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

                            widget.save_format_order();
                        },
                        None => log::error!("Failed to find index for {:?}", item)

                    }
                },
                None => log::error!("Failed to find index for {:?}", value),
            }
            true
        }));
        row.add_controller(drop_target);
        row
    }

    fn populate_formats(&self) {
        //color used as examples
        let example_color = Color::rgb(46, 52, 64);

        let formatter = ColorFormatter::with_color(example_color);

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
                self.save_format_order();
                log::debug!("Order with new items: {:?}", order);
            }
        }

        for item in order {
            let format = match item.to_lowercase().as_str() {
                "hex" => ColorFormatObject::new(item, gettext("Hex-Code"), formatter.hex_code()),
                "rgb" => ColorFormatObject::new(item, gettext("RGB"), formatter.rgb()),
                "hsl" => ColorFormatObject::new(item, gettext("HSL"), formatter.hsl()),
                "hsv" => ColorFormatObject::new(item, gettext("HSV"), formatter.hsv()),
                "cmyk" => ColorFormatObject::new(item, gettext("CMYK"), formatter.cmyk()),
                "xyz" => ColorFormatObject::new(item, gettext("XYZ"), formatter.xyz()),
                "cielab" => ColorFormatObject::new(item, gettext("CIELAB"), formatter.cie_lab()),
                "hwb" => ColorFormatObject::new(item, gettext("HWB"), formatter.hwb()),
                "hcl" => ColorFormatObject::new(item, gettext("CIELCh / HCL"), formatter.hcl()),
                "name" => ColorFormatObject::new(
                    item,
                    gettext("Name"),
                    pgettext(
                        "Information that no name for the color could be found",
                        "Not named",
                    ),
                ),
                "lms" => ColorFormatObject::new(item, gettext("LMS"), formatter.lms()),
                "hunterlab" => {
                    ColorFormatObject::new(item, gettext("Hunter Lab"), formatter.hunter_lab())
                }
                _ => {
                    log::error!("Failed to find format: {item}");
                    continue;
                }
            };

            self.formats().append(&format);
        }
    }
}
