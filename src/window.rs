use std::str::FromStr;

use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use palette::{GetHue, IntoColor, SetHue};

use crate::application::App;
use crate::colors::Notation;
use crate::colors::color::Color;
use crate::config::{APP_ID, PROFILE};
use crate::model::history::HistoryObject;
use crate::widgets::color_format_row::ColorFormatRow;
use crate::widgets::history_item::HistoryItem;
use crate::widgets::placeholder_page::PlaceholderPage;

mod imp {
    use std::cell::{Cell, OnceCell};

    use super::*;

    use adw::subclass::prelude::AdwApplicationWindowImpl;
    use gtk::CompositeTemplate;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/window.ui")]
    pub struct AppWindow {
        #[template_child]
        pub placeholder: TemplateChild<PlaceholderPage>,
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
        pub edit_sheet: TemplateChild<adw::BottomSheet>,
        #[template_child]
        pub color_preview: TemplateChild<gtk::ColorDialogButton>,
        #[template_child]
        pub hue_scale: TemplateChild<gtk::Scale>,
        #[template_child]
        pub saturation_scale: TemplateChild<gtk::Scale>,
        #[template_child]
        pub lightness_scale: TemplateChild<gtk::Scale>,
        #[template_child]
        pub history_list: TemplateChild<gtk::ListBox>,
        pub history: OnceCell<gio::ListStore>,
        pub settings: gio::Settings,
        pub color: Cell<Option<Color>>,
        pub css_provider: gtk::CssProvider,
    }

    impl Default for AppWindow {
        fn default() -> Self {
            Self {
                placeholder: TemplateChild::default(),
                stack: TemplateChild::default(),
                color_button: TemplateChild::default(),
                color_picker_button: TemplateChild::default(),
                toast_overlay: TemplateChild::default(),
                format_box: TemplateChild::default(),
                edit_sheet: TemplateChild::default(),
                hue_scale: TemplateChild::default(),
                saturation_scale: TemplateChild::default(),
                lightness_scale: TemplateChild::default(),
                color_preview: TemplateChild::default(),
                history_list: TemplateChild::default(),
                history: Default::default(),
                settings: gio::Settings::new(APP_ID),
                color: Cell::new(None),
                css_provider: Default::default(),
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

            klass.install_action_async("win.pick-color", None, move |win, _, _var| async move {
                win.pick_color().await;
            });

            klass.install_action(
                "win.show-toast",
                Some(glib::VariantTy::TUPLE),
                move |win, _, var| {
                    if let Some((ref toast, i)) = var.and_then(|v| v.get::<(String, i32)>()) {
                        win.show_toast(toast, adw::ToastPriority::__Unknown(i));
                    }
                },
            );

            klass.install_action(
                "win.set-color",
                Some(glib::VariantTy::STRING),
                move |win, _, var| {
                    let Some(Ok(color)) = var
                        .and_then(|v| v.get::<String>())
                        .map(|v| Color::from_str(&v))
                    else {
                        return;
                    };
                    win.set_color(color);
                },
            );

            klass.install_action(
                "win.remove-item",
                Some(glib::VariantTy::STRING),
                |win, _, var| {
                    let Some(Ok(color)) = var
                        .and_then(|v| v.get::<String>())
                        .map(|v| Color::from_str(&v))
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
                    // the list is shown, which is only the case for 2+ colors
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
                },
            );
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

            let main_context = glib::MainContext::default();
            main_context.spawn_local(glib::clone!(
                #[weak(rename_to = window)]
                self,
                async move {
                    if window.is_color_picker_available().await != Ok(true) {
                        log::warn!("System does not support color picking");
                        window.show_portal_error_page();
                    }

                    // disable history clearing action before an item has been added
                    window.obj().action_set_enabled("app.clear-history", false);
                }
            ));

            // Load latest window state
            obj.load_window_size();
            obj.setup_history();
            obj.order_formats();

            // setup CSS provider to update the edit sheet scale colors
            gtk::style_context_add_provider_for_display(
                &obj.display(),
                &self.css_provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
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

    impl AppWindow {
        /// Check if the system supports color picking.
        async fn is_color_picker_available(&self) -> zbus::Result<bool> {
            let connection = zbus::Connection::session().await?;
            let proxy: zbus::Proxy = zbus::proxy::Builder::new(&connection)
                .interface("org.freedesktop.portal.Screenshot")?
                .path("/org/freedesktop/portal/desktop")?
                .destination("org.freedesktop.portal.Desktop")?
                .build()
                .await?;
            let version = proxy.get_property::<u32>("version").await?;

            // version 2 indicates that the color picker is supported:
            // see: https://github.com/flatpak/xdg-desktop-portal/pull/766,
            Ok(version >= 2)
        }

        /// Shows a warning page, explaining that the system does not support color picking.
        pub fn show_portal_error_page(&self) {
            self.stack.set_visible_child_name("portal-error");
            self.color_picker_button.set_sensitive(false);
        }
    }
}

glib::wrapper! {
    pub struct AppWindow(ObjectSubclass<imp::AppWindow>)
        @extends gtk::Widget, gtk::Window,  gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionMap;
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
        self.imp().toast_overlay.add_toast(toast);
    }

    /// The currently picked color, or `None` if the user hasn't picked one yet.
    fn color(&self) -> Option<Color> {
        self.imp().color.get()
    }

    /// Returns the history list store object.
    fn history(&self) -> &gio::ListStore {
        self.imp().history.get().expect("Failed to get history")
    }

    /// Clear the history by removing all items from the list.
    ///
    /// After clearing it, a toast will be shown with the options to 'undo' the clearing.
    pub fn clear_history(&self) {
        // clear history
        let history = self.history();
        let items = history.snapshot();

        // show toast to undo
        let toast = adw::Toast::builder()
            .title(gettext("Cleared history"))
            .button_label(gettext("Undo"))
            .priority(adw::ToastPriority::High)
            .build();

        toast.connect_button_clicked(glib::clone!(
            #[weak(rename_to = window)]
            self,
            #[strong]
            items,
            move |_toast| {
                window.history().extend_from_slice(&items);
                log::debug!("Undo clicked: {}", items.len());
            }
        ));

        history.remove_all();
        // we cannot use `show_toast` here since that only works for simple text-only toasts
        self.imp().toast_overlay.add_toast(toast);

        if let Some(color) = self.color() {
            let history_item = HistoryObject::new(color);
            self.history().insert(0, &history_item);
        }
    }

    /// Setup the history by setting up a model
    fn setup_history(&self) {
        let model = gio::ListStore::new::<HistoryObject>();

        self.imp()
            .history
            .set(model)
            .expect("Failed to set history model");

        let selection_model = gtk::NoSelection::new(Some(self.history().clone()));
        self.imp()
            .history_list
            .bind_model(Some(&selection_model), move |obj| {
                let history_object = obj
                    .downcast_ref::<HistoryObject>()
                    .expect("The object is not of type `HistoryObject`.");
                let history_item = HistoryItem::new(history_object.color());
                history_item.upcast()
            });

        let gtk_settings = gtk::Settings::default().unwrap();
        gtk_settings.connect_gtk_overlay_scrolling_notify(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |settings| {
                window.adjust_scrollbar_offset(settings);
            }
        ));

        self.history().connect_items_changed(glib::clone!(
            #[weak(rename_to = window)]
            self,
            move |items, _, _, _| {
                // only show history list if there are any items
                let visible = items.n_items() > 1;
                window.imp().history_list.set_visible(visible);
                window.action_set_enabled("app.clear-history", visible);
                window.adjust_scrollbar_offset(&gtk_settings);
            }
        ));
    }

    fn adjust_scrollbar_offset(&self, settings: &gtk::Settings) {
        // FIXME: remove this workaround once https://gitlab.gnome.org/GNOME/gtk/-/issues/6384 is fixed
        // calculate how many history items fit into the list, before it begins scrolling
        let screen_elements = self.imp().history_list.size(gtk::Orientation::Vertical) / 40;
        // width has not been set yet
        if screen_elements == 0 {
            return;
        }
        let history_items = self.history().n_items();
        let scrollbar_width =
            if !settings.is_gtk_overlay_scrolling() && history_items >= screen_elements as u32 {
                // size of the scrollbar
                24
            } else {
                0
            };
        self.imp().history_list.set_margin_end(scrollbar_width);
    }

    /// Save the window size when closing the window
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let imp = self.imp();

        let (width, height) = self.default_size();
        log::debug!("Window Size: {}x{}", width, height);

        imp.settings.set_value(
            "window-dimension",
            &(width, height, self.is_maximized()).to_variant(),
        )?;

        Ok(())
    }

    /// Load the last saved window size and apply it
    fn load_window_size(&self) {
        let imp = self.imp();

        // safe to unwrap, as the settings have default values
        let (width, height, is_maximized): (i32, i32, bool) =
            imp.settings.value("window-dimension").get().unwrap();

        self.set_default_size(width, height);
        self.set_maximized(is_maximized);
    }

    /// Insert the formats in the order in which they are saved in the settings.
    pub fn order_formats(&self) {
        let imp = self.imp();
        let format_box = &imp.format_box;

        format_box
            .observe_children()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatRow>)
            .for_each(|row| format_box.remove(row));

        let order: Vec<String> = imp.settings.get("format-order");
        let visible: Vec<String> = imp.settings.get("visible-formats");
        log::debug!("Formats: {:?}", order);
        log::debug!("Visible: {:?}", visible);

        order
            .iter()
            .filter(|item| visible.contains(item))
            .flat_map(|item| Notation::from_str(item))
            .map(|notation| ColorFormatRow::new(&notation))
            .for_each(|widget| {
                format_box.append(&widget);
                if let Some(color) = self.color() {
                    widget.display_color(color);
                }
            });
    }

    /// Pick a color from the desktop using [ashpd].
    ///
    /// It will show a toast when failing to pick a color, for example when the user cancels the action.
    #[template_callback]
    pub async fn pick_color(&self) {
        log::debug!("Picking a color using the color picker");
        let root = self.root().expect("Failed to get window root");
        let identifier = ashpd::WindowIdentifier::from_native(&root).await;
        let request = ashpd::desktop::screenshot::ColorRequest::default()
            .identifier(identifier)
            .send()
            .await;

        match request.and_then(|req| req.response()) {
            Ok(color) => self.set_color(Color::from(gtk::gdk::RGBA::from(color))),
            Err(err) => {
                log::error!("{}", err);
                if !matches!(
                    err,
                    ashpd::Error::Response(ashpd::desktop::ResponseError::Cancelled)
                ) {
                    self.show_toast(
                        gettext("Failed to pick a color"),
                        adw::ToastPriority::Normal,
                    );
                    // show the warning page to indicate to the user that color picking is
                    // not supported, also disables the color picker to avoid further
                    // errors
                    self.imp().show_portal_error_page();
                }
            }
        };
    }

    /// Set the current color to the given color.
    ///
    /// If the given color is different from the current color,
    /// it will be added to the history. If the history includes the given
    /// color, the preceding occurrence will be removed.
    pub fn set_color(&self, color: Color) {
        if self.color() != Some(color) {
            // TODO: remove check once bug is fixed
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
        imp.color.replace(Some(color));

        //stop showing placeholder page, when a color is set
        imp.stack.set_visible_child_name("main");

        imp.color_button.set_rgba(&color.into());

        imp.format_box
            .observe_children()
            .snapshot()
            .iter()
            .filter_map(Cast::downcast_ref::<ColorFormatRow>)
            .for_each(|row| row.display_color(color));
    }

    /// Opens a bototm sheet with an HSL color picker.
    #[template_callback]
    fn open_sheet(&self) {
        let imp = self.imp();
        imp.color_preview.set_rgba(&self.color().unwrap().into());
        let hsl: palette::Hsl = self.color().unwrap().color.into_color();
        imp.hue_scale
            .set_value(hsl.get_hue().into_positive_degrees() as f64);
        imp.saturation_scale
            .set_value(hsl.saturation as f64 * 100.0);
        imp.lightness_scale.set_value(hsl.lightness as f64 * 100.0);
        imp.edit_sheet.set_open(true);
    }

    /// Updates the preview color and color picker.
    #[template_callback]
    fn on_color_preview_updated(&self, scale: gtk::Scale) {
        let mut hsl: palette::Hsl = Color::from(self.imp().color_preview.rgba())
            .color
            .into_color();

        hsl.set_hue(self.imp().hue_scale.value() as f32);
        hsl.lightness = self.imp().lightness_scale.value() as f32 / 100.0;
        hsl.saturation = self.imp().saturation_scale.value() as f32 / 100.0;

        let gkd_color: gtk::gdk::RGBA = Color::from_palette(hsl).into();
        self.imp().color_preview.set_rgba(&gkd_color);

        if scale != *self.imp().saturation_scale {
            // update gradient of the saturation_scale
            self.imp()
                .css_provider
                .load_from_string(&format!(":root {{ --saturation-color: {}; }}", gkd_color));
        }
    }

    /// Selects the edit color and closes the edit bottom sheet.
    #[template_callback]
    fn on_color_preview_select(&self) {
        let color = Color::from(self.imp().color_preview.rgba());
        self.set_color(color);
        self.imp().edit_sheet.set_open(false);
    }
}
