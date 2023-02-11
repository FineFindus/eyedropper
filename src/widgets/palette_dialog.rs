use adw::{prelude::*, subclass::prelude::*};
use gettextrs::pgettext;
use gtk::{glib, CompositeTemplate};

use crate::colors::{color::Color, formatter::ColorFormatter, position::AlphaPosition};

mod imp {

    use std::cell::RefCell;

    use glib::{
        subclass::{self, Signal},
        ParamSpec,
    };
    use gtk::glib::ParamSpecBoxed;
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, CompositeTemplate)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/palette-window.ui")]
    pub struct PaletteDialog {
        pub color: RefCell<gtk::gdk::RGBA>,
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
                color: RefCell::new(gtk::gdk::RGBA::BLACK),
                palettes_list: TemplateChild::default(),
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
            static PROPERTIES: Lazy<Vec<ParamSpec>> =
                Lazy::new(|| vec![ParamSpecBoxed::builder::<gtk::gdk::RGBA>("color").build()]);
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
            match pspec.name() {
                "color" => {
                    let input_value = value.get::<gtk::gdk::RGBA>().unwrap();
                    self.color.replace(input_value);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "color" => self.color.borrow().to_value(),
                _ => unimplemented!(),
            }
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
            .property("color", &gtk::gdk::RGBA::from(color))
            .build();
        dialog.setup_palettes_list();
        dialog
    }

    /// Returns the given color as a [Color] struct instead of [gtk::gdk::RGBA]
    fn color(&self) -> Color {
        Color::from(self.property::<gtk::gdk::RGBA>("color"))
    }

    ///Setup the list by adding a the generate color palettes
    fn setup_palettes_list(&self) {
        let imp = self.imp();
        let palettes = &imp.palettes_list;

        let color: Color = self.color();
        palettes.append(&self.create_palette_row(
            &pgettext("Name for tints (lighter variants) of the color", "Tints"),
            color.tints(0.15, 5),
        ));
        palettes.append(&self.create_palette_row(
            &pgettext("Name for shades (darker variants) of the color", "Shades"),
            color.shades(0.15, 5),
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
        gtk_macros::spawn!(glib::clone!(@weak self as window => async move {
            let color = window.color();
            //capacity for all palettes
            let mut colors = Vec::with_capacity(28);

            colors.append(&mut color.tints(0.15, 5));
            colors.append(&mut color.shades(0.15, 5));
            colors.append(&mut vec![color, color.complementary_color()]);
            colors.append(&mut color.split_complementary_color());
            colors.append(&mut color.triadic_colors());
            colors.append(&mut color.tetradic_colors());
            colors.append(&mut color.analogous_colors(6));

            window.save_to_file(&pgettext("Name of the save palette file, do not add a file extension, .gpl will be added automatically", "palettes"), colors).await;
        }));
    }

    /// Saves a list of colors as a GIMP palette files (.gpl).
    ///
    /// The colors will be saved without alpha values under the name `Untitled`.
    /// This opens up a user prompt to ask where to save the file and then write said file, if the user cancels, the file will not be saved.
    pub async fn save_to_file(&self, name: &str, colors: Vec<Color>) {
        let file_chooser = gtk::FileChooserNative::builder()
            .transient_for(self)
            .action(gtk::FileChooserAction::Save)
            .modal(true)
            .create_folders(true)
            .build();

        file_chooser.set_current_name(&format!("{}.gpl", name));
        let palette = Color::gpl_file(name, colors);

        file_chooser.connect_response(
            glib::clone!(@weak self as window, @strong palette => move |file_chooser, response| {
                if response == gtk::ResponseType::Accept {
                    if let Some(path) = file_chooser.file().and_then(|file| file.path()) {
                        log::debug!("Selected path: {}", path.display());
                        std::fs::write(path, &palette).expect("Failed to write GIMP palette file");
                    }
                } else {
                    log::error!("Failed to save file: {}", response);
                }
            }),
        );

        file_chooser.show();
    }
}
