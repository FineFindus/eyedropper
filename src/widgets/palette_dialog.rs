use adw::{prelude::*, subclass::prelude::*};
use gettextrs::pgettext;
use gtk::{glib, CompositeTemplate};

use crate::colors::{color::Color, formatter::ColorFormatter, position::AlphaPosition};

mod imp {

    use std::cell::Cell;

    use glib::{
        subclass::{self, Signal},
        ParamSpec, Properties, Value,
    };
    use once_cell::sync::Lazy;

    use crate::config;

    use super::*;

    #[derive(Debug, CompositeTemplate, Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/palette-window.ui")]
    #[properties(wrapper_type = super::PaletteDialog)]
    pub struct PaletteDialog {
        #[property(get, set)]
        pub color: Cell<gtk::gdk::RGBA>,
        #[template_child]
        pub palettes_list: TemplateChild<gtk::ListBox>,
        pub settings: gtk::gio::Settings,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PaletteDialog {
        const NAME: &'static str = "PaletteDialog";
        type Type = super::PaletteDialog;
        type ParentType = adw::Window;

        fn new() -> Self {
            Self {
                color: Cell::new(gtk::gdk::RGBA::BLACK),
                palettes_list: TemplateChild::default(),
                settings: gtk::gio::Settings::new(config::APP_ID),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();

            klass.install_action("export", Some("s"), |widget, _, variant| {
                let value = variant.unwrap().get::<String>();
                widget.save_to_file(value);
            });
        }

        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PaletteDialog {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("palette-clicked")
                    //since we need to send multiple vars (vec of colors), but
                    //afaik there is no way to achieve that.
                    //So instead we send a string of hex color and split it back to color
                    .param_types([String::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }
        fn set_property(&self, _id: usize, _value: &Value, _pspec: &ParamSpec) {
            Self::derived_set_property(self, _id, _value, _pspec)
        }
        fn property(&self, _id: usize, _pspec: &ParamSpec) -> Value {
            Self::derived_property(self, _id, _pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();
        }
    }
    impl WindowImpl for PaletteDialog {}
    impl WidgetImpl for PaletteDialog {}
    impl AdwWindowImpl for PaletteDialog {}
}
glib::wrapper! {
    pub struct PaletteDialog(ObjectSubclass<imp::PaletteDialog>)
        @extends gtk::Widget, gtk::Window, adw::Window;
}

#[gtk::template_callbacks]
impl PaletteDialog {
    pub fn new(color: Color) -> Self {
        let dialog = glib::Object::builder::<PaletteDialog>()
            .property("color", gtk::gdk::RGBA::from(color))
            .build();
        dialog.setup_palettes_list();
        dialog
    }

    /// Show palettes in the dialog.
    fn palettes(&self) -> Vec<Color> {
        let color: Color = self.color().into();
        //capacity for all palettes
        let mut colors = Vec::with_capacity(28);

        let quantity = self.imp().settings.uint("shades-tints-quantity").max(1) as usize;

        colors.append(&mut color.tints(0.15, quantity));
        colors.append(&mut color.shades(0.15, quantity));
        colors.append(&mut vec![color, color.complementary_color()]);
        colors.append(&mut color.split_complementary_color());
        colors.append(&mut color.triadic_colors());
        colors.append(&mut color.tetradic_colors());
        colors.append(&mut color.analogous_colors(6));

        colors
    }

    ///Setup the list by adding a the generate color palettes
    fn setup_palettes_list(&self) {
        let imp = self.imp();
        let palettes = &imp.palettes_list;

        let quantity = self.imp().settings.uint("shades-tints-quantity").max(1) as usize;

        let color: Color = self.color().into();
        palettes.append(&self.create_palette_row(
            &pgettext("Name for tints (lighter variants) of the color", "Tints"),
            color.tints(0.15, quantity),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext("Name for shades (darker variants) of the color", "Shades"),
            color.shades(0.15, quantity),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext(
                "Name for the opposite/complementary color (e.g. blue ⇔ yellow)",
                "Complementary",
            ),
            vec![color, color.complementary_color()],
        ));
        palettes.append(&self.create_palette_row(
            &pgettext(
                "The name of the color palette. Consist of the two opposite colors (e.g. blue ⇔ orange / green)",
                "Split-Complementary",
            ),
            color.split_complementary_color(),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext(
                "Name of the color palette, which would form a triangle above the color wheel",
                "Triadic",
            ),
            color.triadic_colors(),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext("The name of the color palette.", "Tetradic"),
            color.tetradic_colors(),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext(
                "Color palette consisting of six slight shifted colors",
                "Analogous",
            ),
            color.analogous_colors(6),
        ));
    }

    /// Create a new row with the title and the colors.
    fn create_palette_row(&self, title: &str, colors: Vec<Color>) -> adw::ActionRow {
        let palette_box = gtk::Box::builder().build();

        //add two invisible spacer bins
        palette_box.append(
            &adw::Bin::builder()
                .width_request(32)
                .height_request(32)
                .margin_end(5)
                .valign(gtk::Align::Center)
                .css_classes(vec!["invisible".to_owned()])
                .build(),
        );

        palette_box.append(
            &adw::Bin::builder()
                .width_request(32)
                .height_request(32)
                .margin_end(5)
                .valign(gtk::Align::Center)
                .css_classes(vec!["invisible".to_owned()])
                .build(),
        );

        for color in colors.clone() {
            let formatter = ColorFormatter::with_color(color);
            let color_hex = formatter.hex_code();

            let class_name = format!("colorbin-{}", color_hex.replace('#', ""));
            let color_box = adw::Bin::builder()
                .width_request(32)
                .height_request(32)
                .margin_end(5)
                .valign(gtk::Align::Center)
                .build();

            let css_provider = gtk::CssProvider::new();

            if let Some(display) = gtk::gdk::Display::default() {
                #[allow(deprecated)] //https://github.com/gtk-rs/gtk4-rs/issues/1317
                gtk::StyleContext::add_provider_for_display(&display, &css_provider, 400);
            }

            css_provider.load_from_data(&format!(
                ".{} {{background-color: {};border-radius: 6px;}}",
                class_name, color_hex
            ));
            color_box.add_css_class(&class_name);
            palette_box.append(&color_box);
        }

        let row = adw::ActionRow::builder()
            .activatable(true)
            .title(title)
            .build();
        row.add_suffix(&palette_box);

        //Convert color to a string so the signal can emit it
        let color_string = colors
            .into_iter()
            .map(|color| {
                let formatter = ColorFormatter::with_alpha_position(color, AlphaPosition::None);
                formatter.hex_code()
            })
            .collect::<Vec<String>>()
            .join(" ");

        row.connect_activated(glib::clone!(@weak self as dialog => move |_| {
            //close window and add palette
            dialog.close();
            dialog.emit_by_name("palette-clicked", &[&color_string])
        }));

        row
    }

    /// Save all palettes to a palette file.
    /// Called when the icon button in the headerbar is clicked.
    #[template_callback]
    fn on_save_clicked(&self) {
        self.save_to_file(None);
    }

    /// Opens a dialog to save a palette file. The file format is determined from the file extension.
    /// An extension can be suggested to the user via the `suggested_extension` parameter.
    pub fn save_to_file(&self, suggested_extension: Option<String>) {
        let colors = self.palettes();

        let mut file_name = String::from("eyedropper_palette");
        if let Some(extension) = suggested_extension {
            file_name.push('.');
            file_name.push_str(&extension);
        }

        let file_chooser = gtk::FileDialog::builder()
            .initial_name(file_name)
            .modal(true)
            .build();

        file_chooser.save(
            Some(self),
            gtk::gio::Cancellable::NONE,
            glib::clone!(@weak self as window => move |res| {
                match res.ok().and_then(|file| file.path()) {
                    Some(path) => {
                        log::debug!("Selected path: {}", path.display());

                        let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("Eyedropper Palette");
                        let palette = match path.extension().and_then(|extension| extension.to_str()) {
                            Some("gpl") => ColorFormatter::gpl_file(file_name, colors),
                            Some("txt") => ColorFormatter::paint_dot_net_file(file_name, colors),
                            Some("pal") => ColorFormatter::pal_file(colors),
                            Some("ase") => ColorFormatter::ase_file(colors),
                            Some("soc") => ColorFormatter::soc_file(colors),
                            _ => {
                                //default to exporting the hex colors
                                ColorFormatter::hex_file(colors)
                            },
                        };
                        std::fs::write(path, palette).expect("Failed to write palette file");
                    },
                    None => log::error!("Failed to save file"),
                }
            }),
        );
    }
}
