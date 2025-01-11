use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ObjectExt};

mod imp {

    use adw::subclass::bin::BinImpl;

    use crate::colors::color::Color;

    use super::*;

    #[derive(Default, Debug, gtk::CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/placeholder-page.ui")]
    pub struct PlaceholderPage {
        #[template_child]
        initial_color_button: TemplateChild<gtk::Button>,
        #[template_child]
        initial_color_entry: TemplateChild<gtk::Entry>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PlaceholderPage {
        const NAME: &'static str = "PlaceholderPage";
        type ParentType = adw::Bin;
        type Type = super::PlaceholderPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PlaceholderPage {
        fn constructed(&self) {
            self.parent_constructed();
            self.initial_color_entry
                .bind_property("text", &*self.initial_color_button, "sensitive")
                .transform_to(|_binding, val: String| Some(!val.is_empty()))
                .invert_boolean()
                .build();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for PlaceholderPage {}
    impl BinImpl for PlaceholderPage {}

    #[gtk::template_callbacks]
    impl PlaceholderPage {
        #[template_callback]
        async fn on_color_entry_active(&self) {
            let text = self.initial_color_entry.text();
            match gtk::gdk::RGBA::parse(&text) {
                Ok(color) => {
                    let color: Color = color.into();
                    self.obj()
                        .activate_action("win.set-color", Some(&color.hex().to_variant()))
                        .expect("Failed to set color");
                }
                Err(_) => {
                    log::debug!("Failed to parse color: {}", text);
                    const STYLE_CLASS: &str = "error";
                    self.initial_color_entry.add_css_class(STYLE_CLASS);
                    glib::timeout_future(std::time::Duration::from_millis(350)).await;
                    self.initial_color_entry.remove_css_class(STYLE_CLASS);
                }
            }
        }
    }
}

glib::wrapper! {
    pub struct PlaceholderPage(ObjectSubclass<imp::PlaceholderPage>)
    @extends gtk::Widget, adw::Bin,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable, gtk::Actionable;
}
