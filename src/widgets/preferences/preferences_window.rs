use std::str::FromStr;

use gettextrs::gettext;
use gtk::Switch;
use gtk::gdk;
use gtk::gio;
use gtk::gio::ListStore;
use gtk::gio::Menu;
use gtk::gio::MenuItem;
use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::colors::Notation;
use crate::colors::color::Color;

use super::color_format::ColorFormatObject;
use adw::prelude::ActionRowExt;
use adw::prelude::PreferencesDialogExt;

mod imp {

    use std::cell::RefCell;

    use adw::subclass::{dialog::AdwDialogImpl, preferences_dialog::PreferencesDialogImpl};
    use gtk::gio;

    use crate::{colors::color_names::ColorNameSources, config};

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/preferences.ui")]
    pub struct PreferencesWindow {
        pub settings: gtk::gio::Settings,
        #[template_child()]
        pub name_source_page: TemplateChild<adw::NavigationPage>,
        #[template_child()]
        pub alpha_pos_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub rgb_format_box: TemplateChild<adw::ComboRow>,
        #[template_child()]
        pub precision_row: TemplateChild<adw::SpinRow>,
        #[template_child()]
        pub order_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub(super) name_source_basic: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_extended: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_gnome: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_xkcd: TemplateChild<adw::SwitchRow>,
        pub format_order: RefCell<Option<gio::ListStore>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesDialog;

        fn new() -> Self {
            Self {
                settings: gtk::gio::Settings::new(config::APP_ID),
                name_source_page: TemplateChild::default(),
                alpha_pos_box: TemplateChild::default(),
                rgb_format_box: TemplateChild::default(),
                precision_row: TemplateChild::default(),
                order_list: TemplateChild::default(),
                name_source_basic: TemplateChild::default(),
                name_source_extended: TemplateChild::default(),
                name_source_gnome: TemplateChild::default(),
                name_source_xkcd: TemplateChild::default(),
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
            obj.populate_formats();

            self.settings
                .bind("alpha-position", &*self.alpha_pos_box, "selected")
                .build();

            self.settings
                .bind("rgb-notation", &*self.rgb_format_box, "selected")
                .build();

            self.settings
                .bind("precision-digits", &*self.precision_row, "value")
                .build();

            self.bind_setting(&self.name_source_basic, ColorNameSources::Html);
            self.bind_setting(&self.name_source_extended, ColorNameSources::Svg);
            self.bind_setting(&self.name_source_gnome, ColorNameSources::Gnome);
            self.bind_setting(&self.name_source_xkcd, ColorNameSources::Xkcd);
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl PreferencesWindow {
        pub(super) fn bind_setting(&self, obj: &adw::SwitchRow, flag_val: ColorNameSources) {
            self.settings
                .bind("name-sources-flag", obj, "active")
                .mapping(move |value, _variant| {
                    let flag = ColorNameSources::from_bits(value.get::<u32>()?)?;
                    Some(flag.contains(flag_val).to_value())
                })
                .set_mapping(glib::clone!(
                    #[weak(rename_to = window)]
                    self,
                    #[upgrade_or]
                    None,
                    move |value, _variant| {
                        let active = value
                            .get::<bool>()
                            .expect("Failed to get bool from switch active property");
                        let mut color_names_flag = ColorNameSources::from_bits(
                            window.settings.get::<u32>("name-sources-flag"),
                        )?;
                        color_names_flag.set(flag_val, active);

                        Some(color_names_flag.bits().to_variant())
                    }
                ))
                .build();
        }
    }

    impl WidgetImpl for PreferencesWindow {}
    impl AdwDialogImpl for PreferencesWindow {}
    impl PreferencesDialogImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
    @extends gtk::Widget, adw::Dialog, adw::PreferencesDialog,
    @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget;

}

#[gtk::template_callbacks]
impl PreferencesWindow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>()
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
        self.push_subpage(&*self.imp().name_source_page);
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
            .set("format-order", formats)
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
        let model = ListStore::new::<ColorFormatObject>();

        // Get state and set model
        self.imp().format_order.replace(Some(model));

        // Wrap model with selection and pass it to the list view
        let selection_model = gtk::NoSelection::new(Some(self.formats()));
        self.imp().order_list.bind_model(
            Some(&selection_model),
            glib::clone!(
                #[weak(rename_to = widget)]
                self,
                #[upgrade_or_panic]
                move |obj| {
                    let formats_object = obj
                        .downcast_ref()
                        .expect("The object is not of type `ColorFormatObject`.");
                    let hist = widget.create_format_row(formats_object);
                    hist.upcast()
                }
            ),
        );

        // Assure that the formats list is only visible when it is supposed to
        self.set_order_list_visible(&self.formats());
        self.formats().connect_items_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |items, _, _, _| {
                window.set_order_list_visible(items);
            }
        ));
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
            .mapping(glib::clone!(
                #[weak]
                item,
                #[upgrade_or]
                None,
                move |value, _variant| {
                    let visible = value.get::<Vec<String>>().unwrap_or(Vec::with_capacity(0));
                    Some(visible.contains(&item.identifier()).to_value())
                }
            ))
            .set_mapping(glib::clone!(
                #[weak(rename_to = window)]
                self,
                #[weak]
                item,
                #[upgrade_or]
                None,
                move |value, _variant| {
                    let active = value
                        .get::<bool>()
                        .expect("Failed to get bool from switch active property");
                    let mut visible_formats =
                        window.imp().settings.get::<Vec<String>>("visible-formats");

                    if active {
                        visible_formats.push(item.identifier());
                    } else {
                        visible_formats.retain(|format| format != &item.identifier());
                    }

                    Some(visible_formats.to_variant())
                }
            ))
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
        up_action.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            #[weak]
            item,
            move |_, _| {
                if let Some(index) = window.formats().find(&item) {
                    log::debug!("Moving {} up", item.label());
                    window.formats().remove(index);
                    window.formats().insert(index.saturating_sub(1), &item);
                    window.save_format_order();
                }
            }
        ));
        actions.add_action(&up_action);

        let down_action = gio::SimpleAction::new("move-down", None);
        down_action.connect_activate(glib::clone!(
            #[weak(rename_to = window)]
            self,
            #[weak]
            item,
            move |_, _| {
                if let Some(index) = window.formats().find(&item) {
                    log::debug!("Moving {} down", item.label());
                    window.formats().remove(index);
                    //index should not be larger than the largest index
                    window
                        .formats()
                        .insert((index + 1).min(window.formats().n_items()), &item);
                    window.save_format_order();
                }
            }
        ));
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
        let icon_theme = gtk::IconTheme::default();
        icon_theme.add_resource_path("/com/github/finefindus/eyedropper/icons/scalable/actions/");
        let drag_handle = gtk::Image::from_icon_name("list-drag-handle-symbolic");
        drag_handle.add_css_class("drag-handle");
        row.add_prefix(&drag_handle);

        let drag = gtk::DragSource::builder()
            .name("preferences-drag-format")
            .actions(gtk::gdk::DragAction::MOVE)
            .build();

        drag.connect_prepare(glib::clone!(
            #[weak]
            item,
            #[weak(rename_to = widget)]
            row,
            #[upgrade_or]
            None,
            move |source, _, _| {
                let icon = gtk::WidgetPaintable::new(Some(&widget));
                source.set_icon(Some(&icon), 0, 0);
                Some(gdk::ContentProvider::for_value(&item.to_value()))
            }
        ));
        row.add_controller(drag);

        let drop_target = gtk::DropTarget::builder()
            .name("preferences-drag-format")
            .propagation_phase(gtk::PropagationPhase::Capture)
            .actions(gtk::gdk::DragAction::MOVE)
            .build();

        drop_target.set_types(&[ColorFormatObject::static_type()]);

        drop_target.connect_drop(glib::clone!(
            #[weak(rename_to = widget)]
            self,
            #[weak]
            item,
            #[upgrade_or]
            false,
            move |_, value, _, _| {
                let value = value
                    .get::<ColorFormatObject>()
                    .expect("Failed to get index value");

                if item == value {
                    return false;
                }

                match (widget.formats().find(&value), widget.formats().find(&item)) {
                    (Some(source_index), Some(target_index)) => {
                        log::debug!("Source: {} Target: {}", source_index, target_index);
                        widget.formats().remove(source_index);
                        widget.formats().insert(target_index, &value);
                        widget.save_format_order();
                    }
                    (source, target) => log::error!(
                        "Failed to find indices for dragged row, source: {:?}, target: {:?}",
                        source,
                        target
                    ),
                }
                true
            }
        ));
        row.add_controller(drop_target);
        row
    }

    fn populate_formats(&self) {
        //color used as examples
        let example_color = Color::random();

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
            let format = Notation::from_str(&item)
                .expect("Failed to create ColorFormatObject")
                .to_color_format_object(item, example_color);

            self.formats().append(&format);
        }
    }
}
