use glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ObjectExt};

mod imp {

    use std::cell::Cell;

    use crate::colors::{
        color::Color, color_names::ColorNameSources, position::AlphaPosition, Notation,
    };

    use super::*;

    #[derive(Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/history-item.ui")]
    #[properties(wrapper_type = super::HistoryItem)]
    pub struct HistoryItem {
        #[property(get, set = Self::set_color)]
        color: Cell<gtk::gdk::RGBA>,
        #[template_child]
        popover: TemplateChild<gtk::PopoverMenu>,
        #[template_child]
        right_click_gesture: TemplateChild<gtk::GestureClick>,
        #[template_child]
        press_gesture: TemplateChild<gtk::GestureLongPress>,
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
            klass.bind_template_callbacks();
            klass.install_action("history.remove", None, |item, _, _value| {
                item.activate_action(
                    "win.remove-item",
                    Some(&Color::from(item.color()).hex().to_variant()),
                )
                .expect("Failed to call win.set-color action");
            });
            klass.install_action("history.clicked", None, |item, _, _value| {
                item.activate_action(
                    "win.set-color",
                    Some(&Color::from(item.color()).hex().to_variant()),
                )
                .expect("Failed to call win.set-color action");
            });
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

    impl WidgetImpl for HistoryItem {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            // width and height values from GTK Inspector
            let rect = gtk::graphene::Rect::new(0.0, 0.0, 36.0, 34.0);
            snapshot.translate(&gtk::graphene::Point::new(-10.0, -5.0));
            snapshot.push_rounded_clip(&gtk::gsk::RoundedRect::from_rect(rect, 6.0));
            snapshot.append_color(&self.color.get(), &rect);
            snapshot.pop();
            // translate back so that the focus ring is centered correctly
            snapshot.translate(&gtk::graphene::Point::new(10.0, 5.0));
        }
    }

    impl ButtonImpl for HistoryItem {}

    #[gtk::template_callbacks]
    impl HistoryItem {
        pub(super) fn set_color(&self, color: gtk::gdk::RGBA) {
            self.color.set(color);
            self.obj().queue_draw();
        }

        #[template_callback]
        fn show_popover(&self) {
            self.popover.popup();
        }

        #[template_callback]
        fn tooltip(&self, color: gtk::gdk::RGBA) -> String {
            if color.alpha() != 1.0 {
                Color::from(color).hex()
            } else {
                Notation::Hex.as_str(
                    color.into(),
                    AlphaPosition::None,
                    false,
                    2,
                    ColorNameSources::empty(),
                )
            }
        }
    }
}

glib::wrapper! {
    pub struct HistoryItem(ObjectSubclass<imp::HistoryItem>)
    @extends gtk::Widget, gtk::Button,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Actionable;
}

impl HistoryItem {
    pub fn new(color: gtk::gdk::RGBA) -> Self {
        Object::builder().property("color", color).build()
    }
}
