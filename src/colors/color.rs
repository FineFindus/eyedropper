use core::fmt;

use super::{illuminant::Illuminant, parser, position::AlphaPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    /// Although the RGB values will be randomized, the alpha value will be maximized,
    /// so the color will not be transparent.
    pub fn random() -> Self {
        Color::rgb(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        )
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

    /// Converts the color to HSV values.
    ///
    /// Formula from <https://en.wikipedia.org/wiki/HSL_and_HSV>
    pub fn to_hsv(self) -> (u16, f32, f32) {
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        //find the max out of 3 values
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        let hue = self.calculate_hue();

        let saturation = if max == 0f32 { 0f32 } else { (max - min) / max };

        log::debug!("HSV: {}°, {}%, {}% ", hue, saturation, max);
        (hue, saturation, max)
    }

    /// Returns the [HWB](https://en.wikipedia.org/wiki/HWB_color_model) values of the color.
    ///
    /// The color is converted from RGB according to the formula on the wikipedia page.
    pub fn to_hwb(self) -> (u16, f32, f32) {
        //rescale rgb to be between 0 and 1
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        let hue = self.calculate_hue();
        let white = red.min(green.min(blue));
        let black = 1f32 - red.max(green.max(blue));
        (hue, white, black)
    }

    /// Converts the color to HSL values.
    ///
    /// Formula from <https://en.wikipedia.org/wiki/HSL_and_HSV>
    pub fn to_hsl(self) -> (u16, f32, f32) {
        let red = self.red as f32 / 255f32;
        let green = self.green as f32 / 255f32;
        let blue = self.blue as f32 / 255f32;

        //find the max out of 3 values
        let max = red.max(green.max(blue));
        let min = red.min(green.min(blue));

        let hue = self.calculate_hue();

        let saturation = if max == 0f32 || min == 1f32 {
            0f32
        } else {
            (max - min) / (1f32 - (max + min - 1f32).abs())
        };

        let lightness = (max + min) / 2f32;

        log::debug!("HSL: {}°, {}%, {}% ", hue, saturation, lightness);
        (hue, saturation, lightness)
    }

    /// Returns the CMYK values of the color
    ///
    /// Based on <https://www.easyrgb.com/en/math.php>
    pub fn to_cmyk(self) -> (f32, f32, f32, f32) {
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

        (c, m, y, k)
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

    /// Return the color as Adobe RGB.
    ///
    /// Formula from <http://www.easyrgb.com/en/math.php>
    pub fn _to_adobe_rgb(self) -> (f32, f32, f32) {
        let xyz = self.to_xyz();
        let x = xyz.0 / 100.0;
        let y = xyz.1 / 100.0;
        let z = xyz.2 / 100.0;

        let mut r = x * 2.04137 + y * -0.56495 + z * -0.34469;
        let mut g = x * -0.96927 + y * 1.87601 + z * 0.04156;
        let mut b = x * 0.01345 + y * -0.11839 + z * 1.01541;

        r = r.powf(1.0 / 2.199_218_8);
        g = g.powf(1.0 / 2.199_218_8);
        b = b.powf(1.0 / 2.199_218_8);

        (r * 255f32, g * 255f32, b * 255f32)
    }

    /// Return the colors as CIELAB vales.
    ///
    /// If ten_deg_observer is true, the function will use 10° observer values instead of the 2° ones.
    ///
    /// The color will be first converted to XYZ values and then to CIELAB values.
    /// Formula from <http://www.easyrgb.com/en/math.php>
    pub fn to_cie_lab(self, illuminant: Illuminant, ten_deg_observer: bool) -> (f32, f32, f32) {
        //reference xyz for D65 (sRGB) from http://www.easyrgb.com/en/math.php
        let (reference_x, reference_y, reference_z) = if ten_deg_observer {
            illuminant.ten_degrees()
        } else {
            illuminant.two_degrees()
        };

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

    /// Hunter Lab
    ///
    /// Convert the color to Hunter Lab. The formula is from
    /// <https://www.easyrgb.com/en/math.php>.
    pub fn to_hunter_lab(self, illuminant: Illuminant, ten_deg_observer: bool) -> (f32, f32, f32) {
        //reference xyz for D65 (sRGB) from http://www.easyrgb.com/en/math.php
        let (reference_x, reference_y, reference_z) = if ten_deg_observer {
            illuminant.ten_degrees()
        } else {
            illuminant.two_degrees()
        };

        let (x, y, z) = self.to_xyz();

        let ka = (175.0 / 198.04) * (reference_x + reference_y);
        let kb = (70.0 / 218.11) * (reference_y + reference_z);

        let l = 100.0 * f32::sqrt(y / reference_y);
        let a = ka * (((x / reference_x) - (y / reference_y)) / f32::sqrt(y / reference_y));
        let b = kb * (((y / reference_y) - (z / reference_z)) / f32::sqrt(y / reference_y));

        (
            l,
            if a.is_nan() { 0.0 } else { a },
            if b.is_nan() { 0.0 } else { b },
        )
    }

    /// Convert the color to hcl/ CIELCh
    ///
    /// This steps involves converting the color to CIElab first.
    /// If ten_deg_observer is true, the function will use 10° observer values instead of the 2° ones.
    pub fn to_hcl(self, illuminant: Illuminant, ten_deg_observer: bool) -> (f32, f32, f32) {
        //convert color to lab first
        let (luminance, a, b) = self.to_cie_lab(illuminant, ten_deg_observer);

        let hue = b.atan2(a).to_degrees();
        let chroma = f32::sqrt(a.powi(2) + b.powi(2));

        (
            if hue >= 0.0 { hue } else { hue + 360.0 },
            chroma,
            luminance,
        )
    }

    /// Convert the color to the LMS color space.
    ///
    /// LMS (long, medium short) is a a color space, that
    /// represents the cones in the human eyes.
    ///
    /// To convert to LMS color space, the color is first converted
    /// to XYZ color space and then using the [CIECAT02](https://en.wikipedia.org/wiki/CIECAM02#CAT02) matrix.
    pub fn to_lms(self) -> (f32, f32, f32) {
        let (x, y, z) = self.to_xyz();

        let long = x * 0.7328 + y * 0.4296 + z * -0.1624;
        let medium = x * -0.7036 + y * 1.6975 + z * 0.0061;
        let short = x * 0.0030 + y * 0.0136 + z * 0.9834;

        (long, medium, short)
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
    pub fn from_hex(input: &str, alpha_position: AlphaPosition) -> Result<Color, ColorError> {
        match parser::hex_color(input, alpha_position) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_rgb(input: &str) -> Result<Color, ColorError> {
        match parser::rgb(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_hsl_string(input: &str) -> Result<Color, ColorError> {
        match parser::hsl(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_hsv_string(input: &str) -> Result<Color, ColorError> {
        match parser::hsv(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_cmyk_string(input: &str) -> Result<Color, ColorError> {
        match parser::cmyk(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_xyz_string(input: &str) -> Result<Color, ColorError> {
        match parser::xyz(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    /// Converts the given HSL color to RGB.
    ///
    /// Hue should be 0-360 and s,l 0-1.
    pub fn from_hsl(hue: u16, saturation: f32, lightness: f32) -> Self {
        Self::from_hsla(hue, saturation, lightness, 255)
    }

    /// Converts the given HSL color to RGB, with an additional alpha value.
    ///
    /// Hue should be 0-360 and s,l 0-1.
    pub fn from_hsla(hue: u16, saturation: f32, lightness: f32, alpha: u8) -> Self {
        let red;
        let green;
        let blue;

        //remap hue to be between 0-1
        let hue = hue as f32 / 360f32;
        // let saturation = saturation * 100f32;
        // let lightness = lightness * 100f32;

        if saturation == 0f32 {
            //achromatic
            red = 1f32;
            green = 1f32;
            blue = 1f32;
        } else {
            fn hue2rgb(p: f32, q: f32, t: f32) -> f32 {
                let mut t = t;
                if t < 0f32 {
                    t += 1f32;
                }
                if t > 1f32 {
                    t -= 1f32;
                }
                if t < (1f32 / 6f32) {
                    return p + (q - p) * 6f32 * t;
                }
                if t < (1f32 / 2f32) {
                    return q;
                }
                if t < (2f32 / 3f32) {
                    return p + (q - p) * (2f32 / 3f32 - t) * 6f32;
                }
                p
            }

            let q = if lightness < 0.5 {
                lightness * (1f32 + saturation)
            } else {
                lightness + saturation - (lightness * saturation)
            };

            let p = 2f32 * lightness - q;

            red = hue2rgb(p, q, hue + (1f32 / 3f32));
            green = hue2rgb(p, q, hue);
            blue = hue2rgb(p, q, hue - (1f32 / 3f32));
        }

        Self::rgba(
            (red * 255f32).floor() as u8,
            (green * 255f32).floor() as u8,
            (blue * 255f32).floor() as u8,
            alpha,
        )
    }

    /// Converts the given HSV color to RGB, with an additional alpha value.
    ///
    /// Hue should be 0-360 and s,l 0-1.
    pub fn from_hsva(hue: u16, saturation: f32, value: f32, alpha: u8) -> Self {
        if saturation == 0.0 {
            return Self::rgba(
                (value * 255.0) as u8,
                (value * 255.0) as u8,
                (value * 255.0) as u8,
                alpha,
            );
        }

        //Hue must be < 1
        let mut hue = (hue as f32) / 360.0 * 6.0;

        if hue == 6.0 {
            hue = 0f32;
        }

        let a = value * (1f32 - saturation);
        let b = value * (1f32 - saturation * (hue - hue.floor()));
        let c = value * (1f32 - saturation * (1f32 - (hue - hue.floor())));

        let (red, green, blue) = match hue.floor() as u8 {
            0 => (value, c, a),
            1 => (b, value, a),
            2 => (a, value, c),
            3 => (a, b, value),
            4 => (c, a, value),
            _ => (value, a, b),
        };

        Self::rgba(
            (red * 255f32).round() as u8,
            (green * 255f32).round() as u8,
            (blue * 255f32).round() as u8,
            alpha,
        )
    }

    /// Converts the given CMYK color to RGB.
    pub fn from_cmyk(cyan: f32, magenta: f32, yellow: f32, k: f32) -> Self {
        Self::rgb(
            (255f32 * (1f32 - cyan) * (1f32 - k)).round() as u8,
            (255f32 * (1f32 - magenta) * (1f32 - k)).round() as u8,
            (255f32 * (1f32 - yellow) * (1f32 - k)).round() as u8,
        )
    }

    /// Converts the given XYZ color to RGB.
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        let x = x / 100f32;
        let y = y / 100f32;
        let z = z / 100f32;

        let mut red = x * 3.2406 + y * -1.5372 + z * -0.4986;
        let mut green = x * -0.9689 + y * 1.8758 + z * 0.0415;
        let mut blue = x * 0.0557 + y * -0.2040 + z * 1.0570;

        if red > 0.0031308 {
            red = 1.055 * (red.powf(1.0 / 2.4)) - 0.055;
        } else {
            red = 12.92 * red;
        }

        if green > 0.0031308 {
            green = 1.055 * (green.powf(1.0 / 2.4)) - 0.055;
        } else {
            green = 12.92 * green;
        }
        if blue > 0.0031308 {
            blue = 1.055 * (blue.powf(1.0 / 2.4)) - 0.055;
        } else {
            blue = 12.92 * blue;
        }

        Self::rgb(
            (red * 255f32).round() as u8,
            (green * 255f32).round() as u8,
            (blue * 255f32).round() as u8,
        )
    }

    /// Return n tints (adding pure white) of the color by the tint factor.
    ///
    /// The following formula from <https://maketintsandshades.com/about> will be used to calculate tinted RGB values:
    /// ```
    /// New value = current value + ((255 - current value) x tint factor)
    /// ```
    pub fn tints(&self, factor: f32, n: usize) -> Vec<Self> {
        let mut colors = Vec::with_capacity(n);

        for i in 0..n {
            //New value = current value + ((255 - current value) x tint factor)
            let red = self.red as f32 + ((255f32 - self.red as f32) * (i as f32 * factor));
            let green = self.green as f32 + ((255f32 - self.green as f32) * (i as f32 * factor));
            let blue = self.blue as f32 + ((255f32 - self.blue as f32) * (i as f32 * factor));

            colors.push(Color::rgb(
                red.round() as u8,
                green.round() as u8,
                blue.round() as u8,
            ));
        }
        colors
    }

    /// Return n shades (adding pure black) of the color.
    ///
    /// The following formula from <https://maketintsandshades.com/about> will be used to calculate tinted RGB values:
    /// ```
    /// New value = current value x shade factor
    /// ```
    pub fn shades(&self, factor: f32, n: usize) -> Vec<Self> {
        let mut colors = Vec::with_capacity(n);

        //go reverse, so the lighter stuff comes first
        for i in (0..n).rev() {
            //New value = current value x shade factor
            let red = self.red as f32 * (i as f32 * factor);
            let green = self.green as f32 * (i as f32 * factor);
            let blue = self.blue as f32 * (i as f32 * factor);

            colors.push(Color::rgb(
                red.round() as u8,
                green.round() as u8,
                blue.round() as u8,
            ));
        }
        colors
    }

    /// Returns the complementary/opposite to self.
    pub fn complementary_color(&self) -> Self {
        Color::rgb(255 - self.red, 255 - self.green, 255 - self.blue)
    }

    /// Returns slit complementary colors.
    pub fn split_complementary_color(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let (hue, saturation, lightness) = self.to_hsl();
        colors.push(*self);
        colors.push(Color::from_hsl((hue + 150) % 360, saturation, lightness));
        colors.push(Color::from_hsl((hue + 210) % 360, saturation, lightness));
        colors
    }

    /// Returns triadic colors.
    pub fn triadic_colors(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let (hue, saturation, lightness) = self.to_hsl();
        colors.push(*self);
        colors.push(Color::from_hsl((hue + 120) % 360, saturation, lightness));
        colors.push(Color::from_hsl((hue + 240) % 360, saturation, lightness));
        colors
    }

    /// Returns tetradic colors.
    pub fn tetradic_colors(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let (hue, saturation, lightness) = self.to_hsl();
        colors.push(*self);
        colors.push(Color::from_hsl((hue + 90) % 360, saturation, lightness));
        colors.push(Color::from_hsl((hue + 180) % 360, saturation, lightness));
        colors.push(Color::from_hsl((hue + 270) % 360, saturation, lightness));
        colors
    }

    /// Returns `n` analogous colors, include itself.
    ///
    /// The colors are generated by shifting the hue by 30°.
    pub fn analogous_colors(&self, n: usize) -> Vec<Self> {
        let slices = 30;

        //convert from RGB to HSL
        let (mut hue, saturation, lightness) = self.to_hsl();
        let part = 360 / slices;

        let mut colors = Vec::with_capacity(n);
        colors.push(*self);

        //always shift by at least 1 slice
        for i in 1..n {
            //add hue degrees
            hue = (hue + part * i as u16) % 360;
            colors.push(Self::from_hsl(hue, saturation, lightness));
        }

        colors
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

impl TryFrom<Vec<u8>> for Color {
    type Error = &'static str;

    /// Converts a `Vec<u8>` to a `Color`.
    ///
    /// Converts the vec to a RGBA (length is greater than 3), or a RGB color.
    /// Returns an error if the length is lower than 3.
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match value.len() {
            n if n >= 4 => Ok(Color::rgba(value[0], value[1], value[2], value[3])),
            3 => Ok(Color::rgb(value[0], value[1], value[2])),
            _ => Err("Vec length must be at least 3"),
        }
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

impl From<search_provider::ResultID> for Color {
    fn from(value: search_provider::ResultID) -> Self {
        Self::from_hex(&value, AlphaPosition::None).expect("Failed to get color from ResultID")
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorError {
    HexConversion(String),
    ParsingError(String),
}

impl<I, O, E> From<nom::IResult<I, O, E>> for ColorError {
    fn from(_error: Result<(I, O), nom::Err<E>>) -> Self {
        Self::ParsingError(String::new())
    }
}

impl From<std::num::ParseIntError> for ColorError {
    fn from(error: std::num::ParseIntError) -> Self {
        Self::HexConversion(error.to_string())
    }
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::ParsingError(err) | ColorError::HexConversion(err) => write!(f, "{}", err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hue() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!(220, color.calculate_hue())
    }

    #[test]
    fn test_to_hsv() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((220, 28.0, 25.0), color.to_hsv())
    }

    #[test]
    fn test_to_hwb() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((220, 0.18039216, 0.7490196), color.to_hwb())
    }

    #[test]
    fn test_to_hsl() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((220, 0.16363637, 0.21568629), color.to_hsl())
    }

    #[test]
    fn test_to_cmyk() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((28.0, 19.0, 0.0, 75.0), color.to_cmyk())
    }

    #[test]
    fn test_to_xyz() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((3.2801187, 3.4069908, 5.335223), color.to_xyz())
    }

    #[test]
    fn test_to_cie_lab() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!(
            (21.605232, 0.6957203, -8.349299),
            color.to_cie_lab(Illuminant::D65, false)
        )
    }

    #[test]
    fn test_to_hcl() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!(
            (274.76328, 8.378235, 21.605232),
            color.to_hcl(Illuminant::D65, false)
        )
    }

    #[test]
    fn test_from_hex() {
        assert_eq!(
            Ok(Color::rgb(46, 52, 64)),
            Color::from_hex("#2E3440", AlphaPosition::None)
        )
    }
}
