use core::fmt;

/// Position of the alpha value for hex strings.
///
/// In most cases (for example hex strings in the browser) the alpha value is
/// the last two characters of the hex string. But in some cases it is the first two characters.
/// For example Android Color Values use this format.
#[derive(Debug, Default)]
pub enum AlphaPosition {
    Start,
    #[default]
    End,
}

#[derive(Debug, Clone, Copy)]
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

    ///Returns the color as in hex form, including the alpha position.
    ///
    ///This will not include the alpha values.
    pub fn to_hex_string(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    ///Returns the color as in hex from.
    ///
    ///This will not include the alpha values.
    pub fn to_hex_string_with_alpha(&self, alpha_position: AlphaPosition) -> String {
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
        }
    }

    /// Create a color from a hex string without alpha values.
    ///
    /// The hex color optionally start with '#'.
    /// It returns an error, if the given string (ignoring the #) is not
    /// 6 characters long, or cannot be parsed as a hex string.
    pub fn from_hex(hex_color: &str) -> Result<Color, ColorError> {
        let mut hex_color = hex_color.to_owned();
        //remove #
        if hex_color.starts_with('#') {
            hex_color = hex_color.replace("#", "");
        }

        if hex_color.len() == 6 {
            //red color from hex values
            let red = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
            let green = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
            let blue = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
            let color = Color::rgb(red, green, blue);
            Ok(color)
        } else {
            Err(ColorError::HexConversion(String::from(
                "Could not convert color, color is not a hex color",
            )))
        }
    }

    /// Create a color from a hex string without alpha values.
    ///
    /// The hex color optionally start with '#'.
    /// It returns an error, if the given string (ignoring the #) is not
    /// 8 characters long, or cannot be parsed as a hex string.
    ///
    /// The alpha_position indicates where the alpha values is stored. View [AlphaPosition] for more information.
    pub fn from_hex_with_alpha(
        hex_color: &str,
        alpha_position: AlphaPosition,
    ) -> Result<Color, ColorError> {
        let mut hex_color = hex_color.to_owned();
        //remove #
        if hex_color.starts_with('#') {
            hex_color = hex_color.replace("#", "");
        }

        if hex_color.len() == 8 {
            let alpha = match alpha_position {
                AlphaPosition::Start => u8::from_str_radix(hex_color.split_at(2).0, 16)?,
                AlphaPosition::End => u8::from_str_radix(hex_color.split_at(6).1, 16)?,
            };
            //red color from hex values
            let red = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
            let green = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
            let blue = u8::from_str_radix(hex_color.split_at(2).0, 16)?;
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
