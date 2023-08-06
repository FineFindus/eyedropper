use adw::{prelude::*, subclass::prelude::*};
use gettextrs::pgettext;
use gtk::{glib, CompositeTemplate};

use crate::colors::{color::Color, formatter::ColorFormatter};

mod imp {

    use std::cell::Cell;

    use glib::{
        subclass::{self, Signal},
        Properties,
    };
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, CompositeTemplate, Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/palette-window.ui")]
    #[properties(wrapper_type = super::PaletteDialog)]
    pub struct PaletteDialog {
        #[property(get, set)]
        pub color: Cell<gtk::gdk::RGBA>,
        #[template_child]
        pub palettes_list: TemplateChild<gtk::ListBox>,
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

    #[glib::derived_properties]
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

        fn constructed(&self) {
            self.parent_constructed();

            self.obj().connect_notify_local(
                Some("color"),
                glib::clone!(@weak self as obj => move |dialog, _| {
                    let palettes = &obj.palettes_list;
                    let color: Color = dialog.color().into();
                    let palette_variants = [
                        (&pgettext("Name for tints (lighter variants) of the color", "Tints"),
                        color.tints(0.15, 5)),
                        (&pgettext("Name for shades (darker variants) of the color", "Shades"),
                        color.shades(0.15, 5)),
                        (&pgettext(
                            "Name for the opposite/complementary color (e.g. blue ⇔ yellow)",
                            "Complementary",
                        ),
                        vec![color, color.complementary_color()]),
                        (&pgettext(
                            "The name of the color palette. Consist of the two opposite colors (e.g. blue ⇔ orange / green)",
                            "Split-Complementary",
                        ),
                        color.split_complementary_color()),
                        (&pgettext(
                            "Name of the color palette, which would form a triangle above the color wheel",
                            "Triadic",
                        ),
                        color.triadic_colors()),
                        (&pgettext("The name of the color palette.", "Tetradic"),
                        color.tetradic_colors()),
                        (&pgettext(
                            "Color palette consisting of six slight shifted colors",
                            "Analogous",
                        ),
                        color.analogous_colors(6)),
                    ];

                    palette_variants
                        .iter()
                        .map(|(title, colors)| dialog.create_palette_row(title, colors))
                        .for_each(|row| palettes.append(&row));
                }),
            );
        }

        fn dispose(&self) {
            self.dispose_template();
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
        dialog
    }

    /// Show palettes in the dialog.
    fn palettes(&self) -> Vec<Color> {
        let color: Color = self.color().into();
        //capacity for all palettes
        let mut colors = Vec::with_capacity(28);

        colors.append(&mut color.tints(0.15, 5));
        colors.append(&mut color.shades(0.15, 5));
        colors.append(&mut vec![color, color.complementary_color()]);
        colors.append(&mut color.split_complementary_color());
        colors.append(&mut color.triadic_colors());
        colors.append(&mut color.tetradic_colors());
        colors.append(&mut color.analogous_colors(6));

        colors
    }

    /// Create a new row with the title and the colors.
    fn create_palette_row(&self, title: &str, colors: &[Color]) -> adw::ActionRow {
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

        for color in colors {
            let formatter = ColorFormatter::with_color(*color);
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
                gtk::style_context_add_provider_for_display(&display, &css_provider, 400);
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
                let formatter = ColorFormatter::with_color(*color);
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
        let mut file_name = String::from("eyedropper_palette");
        if let Some(extension) = suggested_extension {
            file_name.push_str(&format!(".{}", extension));
        }

        let file_chooser = gtk::FileDialog::builder()
            .initial_name(file_name)
            .modal(true)
            .build();

        file_chooser.save(
            Some(self),
            gtk::gio::Cancellable::NONE,
            glib::clone!(@weak self as window => move |res| {
                let Some(path) = res.ok().and_then(|file| file.path()) else {
                    log::error!("Failed to save file");
                    return;
                };
                log::debug!("Selected path: {}", path.display());
                let colors = window.palettes();

                let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("Eyedropper Palette");
                let palette = match path.extension().and_then(|extension| extension.to_str()) {
                    Some("gpl") => ColorFormatter::gpl_file(file_name, &colors),
                    Some("txt") => ColorFormatter::paint_dot_net_file(file_name, &colors),
                    Some("pal") => ColorFormatter::pal_file(&colors),
                    Some("soc") => ColorFormatter::soc_file(&colors),
                    Some("ase") => ColorFormatter::ase_file(colors),
                    _ => ColorFormatter::hex_file(&colors), //default to exporting the hex colors
                };
                std::fs::write(path, palette).expect("Failed to write palette file");
            }),
        );
    }
}
