use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ObjectExt};

mod imp {

    use std::cell::Cell;

    use gettextrs::gettext;

    use crate::colors::{
        color::Color, color_names::ColorNameSources, position::AlphaPosition, Notation,
    };

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/history-item.ui")]
    #[properties(wrapper_type = super::HistoryItem)]
    pub struct HistoryItem {
        #[property(get, set = Self::set_color)]
        pub(super) color: Cell<gtk::gdk::RGBA>,
        #[template_child]
        pub(super) popover: TemplateChild<gtk::PopoverMenu>,
        #[template_child]
        pub(super) right_click_gesture: TemplateChild<gtk::GestureClick>,
        #[template_child]
        pub(super) press_gesture: TemplateChild<gtk::GestureLongPress>,
    }

    impl Default for HistoryItem {
        fn default() -> Self {
            Self {
                color: Cell::new(gtk::gdk::RGBA::TRANSPARENT),
                popover: TemplateChild::default(),
                right_click_gesture: TemplateChild::default(),
                press_gesture: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for HistoryItem {
        const NAME: &'static str = "HistoryItem";
        type ParentType = gtk::Button;
        type Type = super::HistoryItem;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for HistoryItem {
        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for HistoryItem {}
    impl BoxImpl for HistoryItem {}
    impl ButtonImpl for HistoryItem {}

    impl HistoryItem {
        pub(super) fn set_color(&self, color: gtk::gdk::RGBA) {
            self.color.set(color);
            let obj = self.obj();

            let color: Color = color.into();
            let color_hex = color.hex();

            let css_class_name = format!("history-button-{}", color_hex.replace('#', ""));

            let css_provider = gtk::CssProvider::default();

            if let Some(display) = gtk::gdk::Display::default() {
                gtk::style_context_add_provider_for_display(
                    &display,
                    &css_provider,
                    gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
                );
            }

            css_provider.load_from_data(&format!(
                    ".{} {{box-shadow: 5px 5px 5px @shade_color; background-color: {}; border-radius: 6px;}}",
                    css_class_name,
                    // ignore alpha values, they are not displayed properly
                    color_hex
            ));

            obj.add_css_class(&css_class_name);

            let menu = gtk::gio::Menu::new();
            menu.append(
                Some(&gettext("Remove")),
                Some(&format!("win.remove-item('{}')", color_hex)),
            );
            menu.freeze();
            self.popover.set_menu_model(Some(&menu));

            //set the action when the button is clicked
            obj.set_detailed_action_name(&format!("win.set-color('{}')", color_hex));

            let tooltip = if color.alpha != 1.0 {
                color_hex
            } else {
                Notation::Hex.as_str(color, AlphaPosition::None, 2, ColorNameSources::empty())
            };
            obj.set_tooltip_text(Some(&tooltip.to_uppercase()));
        }
    }
}

glib::wrapper! {
    pub struct HistoryItem(ObjectSubclass<imp::HistoryItem>)
    @extends gtk::Box, gtk::Widget, gtk::Button,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Actionable;
}

#[gtk::template_callbacks]
impl HistoryItem {
    pub fn new(color: gtk::gdk::RGBA) -> Self {
        Object::builder().property("color", color).build()
    }

    #[template_callback]
    pub(super) fn show_popover(&self) {
        let imp = self.imp();
        imp.popover.popup();
    }
}
