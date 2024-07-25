use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, prelude::SettingsExtManual, CompositeTemplate};

mod imp {

    use glib::subclass;

    use crate::{colors::color_names::ColorNameSources, config};

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

            self.bind_setting(&self.name_source_basic, ColorNameSources::Html);
            self.bind_setting(&self.name_source_extended, ColorNameSources::Svg);
            self.bind_setting(&self.name_source_gnome, ColorNameSources::Gnome);
            self.bind_setting(&self.name_source_xkcd, ColorNameSources::Xkcd);
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }
    impl WindowImpl for NameSourcesDialog {}
    impl WidgetImpl for NameSourcesDialog {}
    impl AdwWindowImpl for NameSourcesDialog {}
    impl NameSourcesDialog {
        pub(super) fn bind_setting(&self, obj: &adw::SwitchRow, flag_val: ColorNameSources) {
            self.settings
                .bind("name-sources-flag", &*obj, "active")
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
