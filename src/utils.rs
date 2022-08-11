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
