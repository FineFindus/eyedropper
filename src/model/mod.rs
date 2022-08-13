use core::fmt;

use crate::utils;

/// Position of the alpha value for hex strings.
///
/// In most cases (for example hex strings in the browser) the alpha value is
/// the last two characters of the hex string. But in some cases it is the first two characters.
/// For example Android Color Values use this format.
///
/// Defaults to no alpha value
#[derive(Debug, Default, PartialEq)]
pub enum AlphaPosition {
    Start,
    #[default]
    None,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub alpha: u8,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    /// Create a new Color object without alpha values.
    ///
    /// This consist of red, green and blue values. The `alpha` value is set to it's maximum by default.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha: 255,
        }
    }

    /// Create a new Color object with an alpha value.
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Returns the color as in hex form.
    ///
    /// The alpha position will indicate where the alpha value is stored.
    pub fn to_hex_string(&self, alpha_position: AlphaPosition) -> String {
        match alpha_position {
            AlphaPosition::Start => format!(
                "#{:02x}{:02x}{:02x}{:02x}",
                self.alpha, self.red, self.green, self.blue
            ),
            AlphaPosition::End => {
                format!(
                    "#{:02x}{:02x}{:02x}{:02x}",
                    self.red, self.green, self.blue, self.alpha
                )
            }
            AlphaPosition::None => format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue),
        }
    }

    /// Create a color from a hex string.
    ///
    /// The hex color optionally start with '#'.
    /// It returns an error, if the given string (ignoring the #) is not
    /// 6 or 8 characters long, or cannot be parsed as a hex string.
    ///
    /// The alpha_position indicates where the alpha values is stored. View [AlphaPosition] for more information.
    /// If the the has less than 8 chars, and thus cannot contain a alpha value it will be handled the same as being given
    /// `AlphaPosition::None`.
    pub fn from_hex(hex_color: &str, alpha_position: AlphaPosition) -> Result<Color, ColorError> {
        let mut hex_color = hex_color.to_owned();
        //remove #
        if hex_color.starts_with('#') {
            hex_color = hex_color.replace("#", "");
        }

        if hex_color.len() == 6 || hex_color.len() == 8 {
            //read alpha values first
            let alpha = if hex_color.len() == 8 {
                //only check alpha values if string has 8 chars
                match alpha_position {
                    AlphaPosition::Start => utils::hex_value(&mut hex_color)?,
                    AlphaPosition::End => u8::from_str_radix(hex_color.split_at(6).1, 16)?,
                    AlphaPosition::None => 255,
                }
            } else {
                255
            };
            //get color from hex values
            let red = utils::hex_value(&mut hex_color)?;
            let green = utils::hex_value(&mut hex_color)?;
            let blue = utils::hex_value(&mut hex_color)?;
            let color = Color::rgba(red, green, blue, alpha);
            Ok(color)
        } else {
            Err(ColorError::HexConversion(String::from(
                "Could not convert color, color is not a hex color",
            )))
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            alpha: 1,
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[RGB ({:03}, {:03}, {:03}); Hex (#{:02x}{:02x}{:02x})]",
            self.red, self.green, self.blue, self.red, self.green, self.blue,
        )
    }
}

impl From<gtk::gdk::RGBA> for Color {
    fn from(color: gtk::gdk::RGBA) -> Self {
        Color::rgba(
            (255f32 * color.red()) as u8,
            (255f32 * color.green()) as u8,
            (255f32 * color.blue()) as u8,
            (255f32 * color.alpha()) as u8,
        )
    }
}

impl Into<gtk::gdk::RGBA> for Color {
    fn into(self) -> gtk::gdk::RGBA {
        gtk::gdk::RGBA::new(
            self.red as f32 / 255f32,
            self.green as f32 / 255f32,
            self.blue as f32 / 255f32,
            self.alpha as f32 / 255f32,
        )
    }
}

#[derive(Debug)]
pub enum ColorError {
    HexConversion(String),
}

impl From<std::num::ParseIntError> for ColorError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::HexConversion(error.to_string())
    }
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
