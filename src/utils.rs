use std::num::ParseIntError;

use crate::config;

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

/// Returns the path to the config directory.
///
/// If an director for this app does not exist, it will be created.
///
/// # Errors
/// If creating the subdirectory fails, an io::Error will be returned.
fn config_dir() -> Result<std::path::PathBuf, std::io::Error> {
    let mut path = glib::user_data_dir();
    path.push(config::APP_ID);
    std::fs::create_dir_all(&path)?;
    log::debug!("Opening config dir: {}", path.display());
    Ok(path)
}

/// Returns the path to the JSON file in the config folder containing
/// the history of picked colors.
///
/// If file does not exist it will creating, if creating the file
/// causes an error the error will be returned instead.
pub fn history_file_path() -> Result<std::path::PathBuf, std::io::Error> {
    let mut config_dir = config_dir()?;
    config_dir.push("history.json");
    log::debug!("Opening history.json");
    Ok(config_dir)
}
