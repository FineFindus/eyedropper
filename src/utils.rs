use std::num::ParseIntError;

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
/// assert_eq!(35, round_percent(value));
/// ```
pub fn round_percent(v: f32) -> u8 {
    (v * 100f32).round() as u8
}

/// Returns a prettified string of the given value in range [0; 1].
///
/// This function exists to remove trailing zeros from percentages, for example
/// 0% will be returned as `0` instead of `0.00`
///
///
/// If the value is 1, it will be returned as a "1" string.
/// If it is 0, it will be returned as 0, otherwise it will be returned
/// as the value with two digits after the comma.
///
/// # Examples
///  ```
/// let value = 0f32;
/// assert_eq!("0", &pretty_print_percent(value));
/// ```
pub fn pretty_print_percent(value: f32) -> String {
    if value >= 1f32 {
        String::from("1")
    } else if value == 0f32 {
        String::from("0")
    } else {
        format!("{:.2}", value)
    }
}

/// Converts an integer to its corresponding bool value.
pub fn int_to_bool(value: isize) -> bool {
    if value == 1 {
        true
    } else {
        false
    }
}
