use std::time::Duration;

use gettextrs::gettext;
use glib::translate::IntoGlib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{
    glib,
    prelude::{ObjectExt, ToValue},
};

mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    use glib::{subclass::Signal, ParamSpec, Value};
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
        pub color: RefCell<String>,
        #[property(set, get)]
        pub tooltip: RefCell<String>,
        #[property(set, get, default = true)]
        pub editable: Cell<bool>,
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

    impl ObjectImpl for ColorFormatRow {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("copied-text")
                        .param_types([String::static_type()])
                        .build(),
                    Signal::builder("text-edited")
                        .param_types([String::static_type()])
                        .build(),
                ]
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
            let obj = self.obj();
            obj.set_direction(gtk::TextDirection::Ltr);
            obj.setup_signals();
            obj.setup_properties();
            obj.set_visible(false);
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

    /// Get the currently shown text.
    pub fn text(&self) -> String {
        self.property("color")
    }

    /// Set the currently shown text
    pub fn set_text(&self, text: String) {
        self.set_property("color", text);
    }

    /// Switches the button next to the entry.
    ///
    /// If `show_apply` is set to true, the button will be an apply button with a checkmark icon
    /// and the `suggested-action` color.
    /// If set to false, the button will show the normal copy symbol.
    pub fn switch_button(&self, show_apply: bool) {
        let button = &self.imp().format_button;
        if show_apply {
            button.set_icon_name("checkmark-symbolic");
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

    /// Indicate an error with the input occurred by applying the libadwaita error style class
    /// for a short time (250ms), so the entry shows the error for a short moment.
    pub fn show_error(&self) {
        let main_context = glib::MainContext::default();
        main_context.spawn_local(glib::clone!(@weak self as widget => async move {
            widget.add_css_class("error");
            glib::timeout_future_with_priority(glib::PRIORITY_DEFAULT, Duration::from_millis(350)).await;
            widget.remove_css_class("error");
        }));
    }

    /// Indicate success with the input.
    ///
    /// To visualize the success, the `success` libadwaita style class
    /// is applied for a short time (250ms).
    pub fn show_success(&self) {
        let main_context = glib::MainContext::default();
        main_context.spawn_local(glib::clone!(@weak self as widget => async move {
            widget.add_css_class("success");
            glib::timeout_future_with_priority(glib::PRIORITY_DEFAULT, Duration::from_millis(350)).await;
            widget.remove_css_class("success");
        }));
    }

    /// Bind the properties to the target values.
    ///
    /// Binds the `text` properties to the text of the entry, and
    /// the `editable` property to different properties
    /// of the entry to make it (un)-editable
    fn setup_properties(&self) {
        self.bind_property("tooltip", &*self.imp().format_button, "tooltip-text")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        //bind texts
        self.bind_property("color", &*self.imp().entry, "text")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        //bind editable
        self.bind_property("editable", &*self.imp().entry, "editable")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        self.bind_property("editable", &*self.imp().entry, "can-focus")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        self.bind_property("editable", &*self.imp().entry, "can-target")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        self.imp()
            .entry
            .connect_changed(glib::clone!(@weak self as widget => move |_entry| {
                widget.switch_button(widget.text_changed());
            }));
    }

    /// Registers a signal for when the text entry is changed to emit
    /// a signal containing the edited text.
    fn setup_signals(&self) {
        self.imp()
            .entry
            .connect_activate(glib::clone!(@weak self as widget => move |entry| {
                let text = entry.buffer().text();
                if widget.is_visible() && !text.is_empty() {
                    widget.switch_button(false);
                    widget.emit_by_name("text-edited", &[&text.to_value()])
                }
            }));
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
