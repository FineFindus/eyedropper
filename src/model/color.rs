use core::fmt;

use crate::utils;

/// Position of the alpha value for hex strings.
///
/// In most cases (for example hex strings in the browser) the alpha value is
/// the last two characters of the hex string. But in some cases it is the first two characters.
/// For example Android Color Values use this format.
///
/// Defaults to no alpha value
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum AlphaPosition {
    #[default]
    End,
    Start,
    None,
}

//Convert from U32. Needed for converting from the settings AdwComboRow, which use indexes for values.
impl From<u32> for AlphaPosition {
    fn from(u: u32) -> Self {
        match u {
            0 => Self::None,
            1 => Self::End,
            2 => Self::Start,
            _ => Self::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Generate a random color.
    ///
    /// Although teh RGB values will be randomized, the alpha value will be maximized,
    /// so the color will not be transparent.
    pub fn random() -> Self {
        Color::rgb(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        )
    }

    /// Returns the color as in hex form.
    ///
    /// The alpha position will indicate where the alpha value is stored.
    pub fn to_hex_string(self, alpha_position: AlphaPosition) -> String {
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
        .to_ascii_uppercase()
    }

    /// Converts the color to HSV values.
    ///
    /// Formula from <https://en.wikipedia.org/wiki/HSL_and_HSV>
    pub fn to_hsv(self) -> (u16, u8, u8) {
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        //find the max out of 3 values
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        let hue = self.calculate_hue();

        let saturation = utils::round_percent(if max == 0f32 { 0f32 } else { (max - min) / max });

        log::debug!(
            "HSV: {}°, {}%, {}% ",
            hue,
            saturation,
            utils::round_percent(max)
        );
        (hue, saturation, utils::round_percent(max))
    }

    /// Converts the color to HSL values.
    ///
    /// Formula from <https://en.wikipedia.org/wiki/HSL_and_HSV>
    pub fn to_hsl(self) -> (u16, u8, u8) {
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        //find the max out of 3 values
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        let hue = self.calculate_hue();

        let saturation = utils::round_percent(if max == 0f32 || min == 1f32 {
            0f32
        } else {
            (max - min) / (1f32 - (max + min - 1f32).abs())
        });

        let lightness = utils::round_percent((max + min) / 2f32);

        log::debug!("HSL: {}°, {}%, {}% ", hue, saturation, lightness);
        (hue, saturation, lightness)
    }

    /// Calculates the hue of the color.
    ///
    /// This is used when converting from RGB to HSL or HSV.
    /// Formula from <https://en.wikipedia.org/wiki/HSL_and_HSV>
    fn calculate_hue(&self) -> u16 {
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        //find the max out of 3 values
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        let mut hue: f32 = 0f32;

        if max == min {
            hue = 0f32;
        } else if max == red {
            hue = 60f32 * (0f32 + (green - blue) / (max - min))
        } else if max == green {
            hue = 60f32 * (2f32 + (blue - red) / (max - min))
        } else if max == blue {
            hue = 60f32 * (4f32 + (red - green) / (max - min))
        }

        if hue < 0f32 {
            hue += 360f32;
        }

        hue.round() as u16
    }

    /// Returns the CMYK values of the color
    ///
    /// Based on <https://www.easyrgb.com/en/math.php>
    pub fn to_cmyk(self) -> (u8, u8, u8, u8) {
        let mut c = 1f32 - (self.red as f32 / 255f32);
        let mut m = 1f32 - (self.green as f32 / 255f32);
        let mut y = 1f32 - (self.blue as f32 / 255f32);

        let mut k = 1f32;
        if c < k {
            k = c;
        }
        if m < k {
            k = m;
        }
        if y < k {
            k = y;
        }

        if k == 1f32 {
            //only black
            c = 0f32;
            m = 0f32;
            y = 0f32;
        } else {
            c = (c - k) / (1f32 - k);
            m = (m - k) / (1f32 - k);
            y = (y - k) / (1f32 - k);
        }

        (
            utils::round_percent(c),
            utils::round_percent(m),
            utils::round_percent(y),
            utils::round_percent(k),
        )
    }

    /// Return the color as its XYZ equivalent.
    ///
    /// Formula from <http://www.easyrgb.com/en/math.php#text2>
    pub fn to_xyz(self) -> (f32, f32, f32) {
        //normalize color between 0 and 1
        let mut red = self.red as f32 / 255f32;
        let mut green = self.green as f32 / 255f32;
        let mut blue = self.blue as f32 / 255f32;

        if red > 0.04045 {
            red = f32::powf((red + 0.055) / 1.055, 2.4);
        } else {
            red /= 12.92;
        }

        if green > 0.04045 {
            green = f32::powf((green + 0.055) / 1.055, 2.4);
        } else {
            green /= 12.92;
        }

        if blue > 0.04045 {
            blue = f32::powf((blue + 0.055) / 1.055, 2.4);
        } else {
            blue /= 12.92;
        }

        red *= 100f32;
        green *= 100f32;
        blue *= 100f32;

        let x = red * 0.4124 + green * 0.3576 + blue * 0.1805;
        let y = red * 0.2126 + green * 0.7152 + blue * 0.0722;
        let z = red * 0.0193 + green * 0.1192 + blue * 0.9505;

        (x, y, z)
    }

    /// Return the colors as CIELAB vales.
    ///
    /// The color will be first converted to XYZ values and then to CIELAB values.
    /// Formula from <http://www.easyrgb.com/en/math.php>
    pub fn to_cie_lab(&self) -> (f32, f32, f32) {
        //refernce xyz for D65 (sRGB) from http://www.easyrgb.com/en/math.php
        let reference_x = 95.047;
        let reference_y = 100.000;
        let reference_z = 108.883;

        let xyz = self.to_xyz();

        let mut x = xyz.0 / reference_x;
        let mut y = xyz.1 / reference_y;
        let mut z = xyz.2 / reference_z;

        x = if x > 0.008856 {
            f32::powf(x, 1f32 / 3f32)
        } else {
            (7.787 * x) + 16f32 / 116f32
        };

        y = if y > 0.008856 {
            f32::powf(y, 1f32 / 3f32)
        } else {
            (7.787 * y) + 16f32 / 116f32
        };

        z = if z > 0.008856 {
            f32::powf(z, 1f32 / 3f32)
        } else {
            (7.787 * z) + 16f32 / 116f32
        };

        let cie_l = (116f32 * y) - 16f32;
        let cie_a = 500f32 * (x - y);
        let cie_b = 200f32 * (y - z);

        (cie_l, cie_a, cie_b)
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
            hex_color = hex_color.replace('#', "");
        }

        if alpha_position != AlphaPosition::None && hex_color.len() != 8 {
            return Err(ColorError::HexConversion(String::from(
                "Could not convert color, color is not a hex color",
            )));
        }

        if hex_color.len() == 6 || hex_color.len() == 8 {
            //read alpha values first
            let alpha = if hex_color.len() == 8 {
                //only check alpha values if string has 8 chars
                match alpha_position {
                    AlphaPosition::Start => utils::hex_value(&mut hex_color)?,
                    AlphaPosition::End => u8::from_str_radix(hex_color.split_at(6).1, 16)?,
                    _ => 255,
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

impl From<ashpd::desktop::screenshot::Color> for Color {
    fn from(color: ashpd::desktop::screenshot::Color) -> Self {
        Color::rgba(
            (255f64 * color.red()) as u8,
            (255f64 * color.green()) as u8,
            (255f64 * color.blue()) as u8,
            255,
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

impl From<Color> for gtk::gdk::RGBA {
    fn from(color: Color) -> Self {
        gtk::gdk::RGBA::new(
            color.red as f32 / 255f32,
            color.green as f32 / 255f32,
            color.blue as f32 / 255f32,
            color.alpha as f32 / 255f32,
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
        match self {
            ColorError::HexConversion(err) => write!(f, "{}", err),
        }
    }
}
