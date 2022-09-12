use std::num::ParseIntError;

use gtk::ColorChooser;

/// Convert the first two chars of a string into a hex values.
///
/// The convert chars will be removed after the conversion, modifying the original string.
///
/// If the conversion throws an error, the error will be returned instead.
pub fn hex_value(hex_string: &mut String) -> Result<u8, ParseIntError> {
    let value = u8::from_str_radix(hex_string.split_at(2).0, 16)?;
    log::trace!("Value: {}", hex_string.split_at(2).0);
    //remove converted hex values
    hex_string.remove(0);
    hex_string.remove(0);
    Ok(value)
}

/// Converts a less than 1 float (percent) to a integer percent value.
///
/// # Example
/// ```
/// let value = 0.345f32;
/// assert_eq!(35, round_tenth(value));
/// ```
pub fn round_percent(v: f32) -> u8 {
    (v * 100f32).round() as u8
}

/// Adds the given palette to the ColorChooser.
///
/// If the colors are None, the previous palettes will be cleared.
///
/// Temporarily added to fix <https://github.com/gtk-rs/gtk4-rs/issues/1114>
pub fn add_palette(
    widget: &impl glib::IsA<ColorChooser>,
    orientation: gtk::Orientation,
    colors_per_line: i32,
    colors: Option<&[gtk::gdk::RGBA]>,
) {
    unsafe {
        gtk::ffi::gtk_color_chooser_add_palette(
            glib::translate::ToGlibPtr::to_glib_none(&widget.as_ref()).0,
            glib::translate::IntoGlib::into_glib(orientation),
            colors_per_line,
            colors.unwrap_or_default().len() as libc::c_int,
            match colors {
                Some(val) => val.as_ptr() as *mut gtk::gdk::ffi::GdkRGBA,
                None => std::ptr::null_mut(),
            },
        )
    }
}
