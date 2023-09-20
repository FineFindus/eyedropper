use adw::subclass::prelude::*;
use gtk::{glib, prelude::SettingsExtManual, CompositeTemplate};

mod imp {

    use glib::subclass;

    use crate::config;

    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(
        resource = "/com/github/finefindus/eyedropper/ui/preferences/name-sources-dialog.ui"
    )]
    pub struct NameSourcesDialog {
        #[template_child]
        pub(super) name_source_basic: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_extended: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_gnome: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) name_source_xkcd: TemplateChild<adw::SwitchRow>,
        pub settings: gtk::gio::Settings,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for NameSourcesDialog {
        const NAME: &'static str = "NameSourceDialog";
        type Type = super::NameSourcesDialog;
        type ParentType = adw::Window;

        fn new() -> Self {
            Self {
                name_source_basic: TemplateChild::default(),
                name_source_extended: TemplateChild::default(),
                name_source_gnome: TemplateChild::default(),
                name_source_xkcd: TemplateChild::default(),
                settings: gtk::gio::Settings::new(config::APP_ID),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for NameSourcesDialog {
        fn constructed(&self) {
            self.parent_constructed();

            self.settings
                .bind("name-source-basic", &*self.name_source_basic, "active")
                .build();

            self.settings
                .bind(
                    "name-source-extended",
                    &*self.name_source_extended,
                    "active",
                )
                .build();

            self.settings
                .bind(
                    "name-source-gnome-palette",
                    &*self.name_source_gnome,
                    "active",
                )
                .build();

            self.settings
                .bind("name-source-xkcd", &*self.name_source_xkcd, "active")
                .build();
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WindowImpl for NameSourcesDialog {}
    impl WidgetImpl for NameSourcesDialog {}
    impl AdwWindowImpl for NameSourcesDialog {}
}

glib::wrapper! {
    pub struct NameSourcesDialog(ObjectSubclass<imp::NameSourcesDialog>)
        @extends gtk::Widget, gtk::Window, adw::Window;
}

#[gtk::template_callbacks]
impl NameSourcesDialog {
    pub fn new() -> Self {
        let dialog = glib::Object::builder::<NameSourcesDialog>().build();
        dialog
    }
}
