use std::time::Duration;

use gettextrs::gettext;
use glib::translate::IntoGlib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, prelude::ObjectExt};

use crate::colors::color::Color;

mod imp {
    use std::cell::RefCell;

    use crate::colors::{self, formatter::ColorFormatter};

    use super::*;

    use glib::subclass::Signal;
    use once_cell::sync::Lazy;

    #[derive(Default, Debug, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/com/github/finefindus/eyedropper/ui/color-format-row.ui")]
    #[properties(wrapper_type = super::ColorFormatRow)]
    pub struct ColorFormatRow {
        #[template_child]
        pub entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub format_button: TemplateChild<gtk::Button>,
        #[property(set, get)]
        pub tooltip: RefCell<String>,
        #[property(set, get)]
        pub color: RefCell<String>,
        #[property(set, get, builder(colors::Notation::default()))]
        pub color_format: RefCell<colors::Notation>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ColorFormatRow {
        const NAME: &'static str = "ColorFormatRow";
        type ParentType = gtk::Box;
        type Type = super::ColorFormatRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for ColorFormatRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("copied-text")
                    .param_types([String::static_type()])
                    .build()]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.set_visible(false);

            self.entry
                .connect_activate(glib::clone!(@weak obj => move |entry| {
                    let text = entry.buffer().text();
                    let Ok(color) = obj.color_format().parse(text.as_str()) else {
                        log::debug!("Failed to parse color: {}", text);
                        obj.show_error();
                        return;
                    };
                    obj.display_color(color);
                    obj.show_success();

                    let hex = ColorFormatter::with_color(color).hex_code();
                    obj.activate_action("win.set-color", Some(&hex.to_variant())).expect("Failed to set color");
                }));

            self.entry
                .connect_changed(glib::clone!(@weak obj => move |_entry| {
                    obj.switch_button(obj.text_changed());
                }));
        }

        fn dispose(&self) {
            self.dispose_template();
        }
    }

    impl WidgetImpl for ColorFormatRow {}
    impl BoxImpl for ColorFormatRow {}
}

glib::wrapper! {
    pub struct ColorFormatRow(ObjectSubclass<imp::ColorFormatRow>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

#[gtk::template_callbacks]
impl ColorFormatRow {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }

    /// Updates the displayed color to the given color.
    ///
    /// The displayed color format is determined by the `color_format` of
    /// the widget.
    pub fn display_color(&self, color: Color) {
        let color = self.color_format().as_str(color);
        self.set_color(color);
    }

    /// Switches the button next to the entry.
    ///
    /// If `show_apply` is set to true, the button will be an apply button with a checkmark icon
    /// and the `suggested-action` color.
    /// If set to false, the button will show the normal copy symbol.
    pub fn switch_button(&self, show_apply: bool) {
        let button = &self.imp().format_button;
        if show_apply {
            button.set_icon_name("check-plain-symbolic");
            button.add_css_class("suggested-action");
        } else {
            button.set_icon_name("edit-copy-symbolic");
            button.remove_css_class("suggested-action");
        }
    }

    /// Checks if entry has been edited and is not empty.
    fn text_changed(&self) -> bool {
        let entry_text = self.imp().entry.buffer().text();
        !entry_text.trim().is_empty() && entry_text.trim() != self.color().trim()
    }

    /// Animates the specified style cass, by applying it for 350ms.
    ///
    /// This can be used with the [`error`](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/named-colors.html#error-colors)
    /// and the [`success`](https://gnome.pages.gitlab.gnome.org/libadwaita/doc/1-latest/named-colors.html#success-colors) color for entries.
    fn animate_style_class(&self, style_class: &'static str) {
        let main_context = glib::MainContext::default();
        main_context.spawn_local(
            glib::clone!(@weak self as widget @strong style_class => async move {
                widget.add_css_class(style_class);
                glib::timeout_future(Duration::from_millis(350)).await;
                widget.remove_css_class(style_class);
            }),
        );
    }

    /// Indicate an error/invalid input.
    ///
    /// To visualize the success, the `error` libadwaita style class is applied
    /// for a few milliseconds.
    pub fn show_error(&self) {
        self.animate_style_class("error");
    }

    /// Indicate success/valid input.
    ///
    /// To visualize the success, the `success` libadwaita style class is applied
    /// for a few milliseconds.
    pub fn show_success(&self) {
        self.animate_style_class("success");
    }

    /// Callback when the button next to the entry is pressed.
    ///
    /// Depending on, if the entry has been edited, the button will either
    /// - apply the edited entry by emitting a activate signal
    /// - or copy the text to the users clipboard and show a toast with the copied text.
    #[template_callback]
    fn on_button_pressed(&self, _button: &gtk::Button) {
        let text = self.imp().entry.buffer().text();
        if self.text_changed() {
            log::debug!("Applied entry content: {}", text);
            self.switch_button(false);
            self.imp().entry.emit_activate();
        } else {
            log::debug!("Copied text: {text}");
            let clipboard = self.clipboard();
            clipboard.set_text(&text);
            self.activate_action(
                "win.show-toast",
                Some(
                    &(
                        gettext("Copied “{}”").replace("{}", &text),
                        adw::ToastPriority::High.into_glib(),
                    )
                        .to_variant(),
                ),
            )
            .expect("Failed to show toast");
        }
    }
}
