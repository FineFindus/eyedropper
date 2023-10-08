use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ObjectExt};

mod imp {

    use std::cell::Cell;

    use crate::colors::{color::Color, formatter::ColorFormatter, position::AlphaPosition};

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/history-item.ui")]
    #[properties(wrapper_type = super::HistoryItem)]
    pub struct HistoryItem {
        #[property(get, set = Self::set_color)]
        pub(super) color: Cell<gtk::gdk::RGBA>,
    }

    impl Default for HistoryItem {
        fn default() -> Self {
            Self {
                color: Cell::new(gtk::gdk::RGBA::TRANSPARENT),
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
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
        }

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
            let mut formatter = ColorFormatter::with_color(color);
            let color_hex = formatter.hex_code();

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

            //set the action when the button is clicked
            obj.set_detailed_action_name(&format!("win.set-color('{}')", color_hex));

            let tooltip = if color.alpha != 255 {
                formatter.alpha_position = AlphaPosition::End;
                formatter.hex_code()
            } else {
                color_hex
            };
            obj.set_tooltip_text(Some(&tooltip));
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
}
