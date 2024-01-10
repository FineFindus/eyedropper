use core::fmt;
use std::f32::consts::PI;

use palette::IntoColor;

use super::{illuminant::Illuminant, parser, position::AlphaPosition};

/// Eyedropper's internal color representation.
///
/// Utility struct to
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Color(palette::Srgba);

impl std::ops::Deref for Color {
    type Target = palette::Srgba;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Color {
    /// Create a new Color object without alpha values.
    ///
    /// This consist of red, green and blue values. The `alpha` value is set to it's maximum by default.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::rgba(red, green, blue, 255)
    }

    /// Create a new Color object with an alpha value.
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(palette::Srgba::new(
            red as f32 / 255.0,
            green as f32 / 255.0,
            blue as f32 / 255.0,
            alpha as f32,
        ))
    }

    /// Generate a random color.
    ///
    /// Although the RGB values will be randomized, the alpha value will be maximized,
    /// so the color will not be transparent.
    pub fn random() -> Self {
        Self::rgb(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
        )
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

    /// Return the color as Adobe RGB.
    ///
    /// Formula from <http://www.easyrgb.com/en/math.php>
    pub fn _to_adobe_rgb(self) -> (f32, f32, f32) {
        let xyz: palette::Xyz = self.color.into_color();
        let x = xyz.x / 100.0;
        let y = xyz.y / 100.0;
        let z = xyz.z / 100.0;

        let mut r = x * 2.04137 + y * -0.56495 + z * -0.34469;
        let mut g = x * -0.96927 + y * 1.87601 + z * 0.04156;
        let mut b = x * 0.01345 + y * -0.11839 + z * 1.01541;

        r = r.powf(1.0 / 2.199_218_8);
        g = g.powf(1.0 / 2.199_218_8);
        b = b.powf(1.0 / 2.199_218_8);

        (r * 255f32, g * 255f32, b * 255f32)
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

        let xyz: palette::Xyz = self.color.into_color();

        let ka = (175.0 / 198.04) * (reference_x + reference_y);
        let kb = (70.0 / 218.11) * (reference_y + reference_z);

        let l = 100.0 * f32::sqrt(xyz.y / reference_y);
        let a =
            ka * (((xyz.x / reference_x) - (xyz.y / reference_y)) / f32::sqrt(xyz.y / reference_y));
        let b =
            kb * (((xyz.y / reference_y) - (xyz.z / reference_z)) / f32::sqrt(xyz.y / reference_y));

        (
            l,
            if a.is_nan() { 0.0 } else { a },
            if b.is_nan() { 0.0 } else { b },
        )
    }

    /// Convert the color to the LMS color space.
    ///
    /// LMS (long, medium short) is a a color space, that
    /// represents the cones in the human eyes.
    ///
    /// The conversion uses the formula form [Fundamentals of Imaging Colour Spaces](https://www.uni-weimar.de/fileadmin/user/fak/medien/professuren/Computer_Graphics/3-ima-color-spaces17.pdf)
    /// The matrix is assumed to be under [Illuminant::E].
    pub fn to_lms(self) -> (f32, f32, f32) {
        let xyz: palette::Xyz = self.color.into_color();

        //TODO: use illuminant depended matrices, found here http://brucelindbloom.com/index.html?Eqn_ChromAdapt.html
        let long = xyz.x * 0.3897 + xyz.y * 0.6890 + xyz.z * -0.0787;
        let medium = xyz.x * -0.2298 + xyz.y * 1.1834 + xyz.z * 0.0464;
        let short = xyz.x * 0.0 + xyz.y * 0.0 + xyz.z * 1.0;

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

    pub fn from_cie_lab_string(
        input: &str,
        illuminant: Illuminant,
        ten_deg_observer: bool,
    ) -> Result<Color, ColorError> {
        match parser::cielab(input, illuminant, ten_deg_observer) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_hwb_string(input: &str) -> Result<Color, ColorError> {
        match parser::hwb(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_hcl_string(input: &str) -> Result<Color, ColorError> {
        match parser::lch(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_lms_string(input: &str) -> Result<Color, ColorError> {
        match parser::lms(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_hunter_lab_string(
        input: &str,
        illuminant: Illuminant,
        ten_deg_observer: bool,
    ) -> Result<Color, ColorError> {
        match parser::hunter_lab(input, illuminant, ten_deg_observer) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_oklab_string(input: &str) -> Result<Color, ColorError> {
        match parser::oklab(input) {
            Ok((_input, color)) => Ok(color),
            Err(err) => {
                log::error!("Failed to parse color: {}", err);
                Err(ColorError::ParsingError(err.to_string()))
            }
        }
    }

    pub fn from_oklch_string(input: &str) -> Result<Color, ColorError> {
        match parser::oklch(input) {
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
            let hue2rgb = |p, q, t| {
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
            };

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

    /// Converts the given HWB color to RGB, with an additional alpha value.
    ///
    /// Hue should be 0-360 and w,b 0-1.
    pub fn from_hwba(hue: u16, white: f32, black: f32, alpha: u8) -> Self {
        if white + black >= 1.0 {
            let gray = ((white / (white + black)) * 255f32).round() as u8;
            return Self::rgba(gray, gray, gray, alpha);
        }

        let mut color = Self::from_hsl(hue, 1.0, 0.5);

        let modify_value = |mut value| {
            value *= 1.0 - white - black;
            value += white;
            value
        };

        color.red = modify_value(color.red);
        color.green = modify_value(color.green);
        color.blue = modify_value(color.blue);
        color.alpha = alpha as f32 / 255.0;

        color
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
    pub fn from_xyz(x: f32, y: f32, z: f32, alpha: u8) -> Self {
        let x = x / 100f32;
        let y = y / 100f32;
        let z = z / 100f32;

        let mut red = x * 3.2406 + y * -1.5372 + z * -0.4986;
        let mut green = x * -0.9689 + y * 1.8758 + z * 0.0415;
        let mut blue = x * 0.0557 + y * -0.2040 + z * 1.0570;

        let remap = |value: &mut f32| {
            if *value > 0.0031308 {
                *value = 1.055 * (value.powf(1.0 / 2.4)) - 0.055;
            } else {
                *value *= 12.92;
            }
        };

        remap(&mut red);
        remap(&mut green);
        remap(&mut blue);

        Self::rgba(
            (red * 255f32).round() as u8,
            (green * 255f32).round() as u8,
            (blue * 255f32).round() as u8,
            alpha,
        )
    }

    /// Converts from CIE Lab to RGB.
    pub fn from_cie_lab(
        l: f32,
        a: f32,
        b: f32,
        alpha: u8,
        illuminant: Illuminant,
        ten_deg_observer: bool,
    ) -> Self {
        //no direct formula exists
        //convert to xyz first, then to rgb
        let mut y = (l + 16.0) / 116.0;
        let mut x = a / 500.0 + y;
        let mut z = y - b / 200.0;

        let remap = |value: &mut f32| {
            if value.powi(3) > 0.008856 {
                *value = value.powi(3)
            } else {
                *value = (*value - 1.0 / 116.0) / 7.787
            }
        };

        remap(&mut y);
        remap(&mut x);
        remap(&mut z);

        let (ref_x, ref_y, ref_z) = if ten_deg_observer {
            illuminant.ten_degrees()
        } else {
            illuminant.two_degrees()
        };

        x *= ref_x;
        y *= ref_y;
        z *= ref_z;

        Self::from_xyz(x, y, z, alpha)
    }

    pub fn from_lch(l: f32, c: f32, h: f32, alpha: u8) -> Self {
        let cie_a = c * (h * PI / 180.0).cos();
        let cie_b = c * (h * PI / 180.0).sin();
        Self::from_cie_lab(l, cie_a, cie_b, alpha, Illuminant::default(), false)
    }

    pub fn from_lms(long: f32, medium: f32, short: f32, alpha: u8) -> Self {
        let x = long * 1.9102 + medium * -1.1121 + short * 0.2019;
        let y = long * 0.3710 + medium * 0.6291 + short * 0.0;
        let z = long * 0.0 + medium * 0.0 + short * 1.0;

        Self::from_xyz(x, y, z, alpha)
    }

    pub fn from_hunter_lab(
        l: f32,
        a: f32,
        b: f32,
        illuminant: Illuminant,
        ten_deg_observer: bool,
    ) -> Self {
        let (ref_x, ref_y, ref_z) = if ten_deg_observer {
            illuminant.ten_degrees()
        } else {
            illuminant.two_degrees()
        };

        let ka = (175.0 / 198.04) * (ref_y + ref_x);
        let kb = (70.0 / 218.11) * (ref_y + ref_z);

        let y = ((l / ref_y).powi(2)) * 100.0;
        let x = (a / ka * (y / ref_y).sqrt() + (y / ref_y)) * ref_x;
        let z = -(b / kb * (y / ref_y).sqrt() - (y / ref_y)) * ref_z;

        Self::from_xyz(x, y, z, 255)
    }

    /// Converts a linear value to a normalized gamma
    fn to_gamma(value: f64) -> f64 {
        if value >= 0.0031308 {
            (1.055) * value.powf(1.0 / 2.4) - 0.055
        } else {
            12.92 * value
        }
    }

    /// Converts from oklab to RGB
    pub fn from_oklab(l: f64, a: f64, b: f64, alpha: u8) -> Self {
        let l_aux = l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
        let m_aux = l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
        let s_aux = l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

        let l = l_aux * l_aux * l_aux;
        let m = m_aux * m_aux * m_aux;
        let s = s_aux * s_aux * s_aux;

        let r = 4.076_741_662_1 * l - 3.307_711_591_3 * m + 0.230_969_929_2 * s;
        let g = -1.268_438_004_6 * l + 2.609_757_401_1 * m - 0.341_319_396_5 * s;
        let b = -0.004_196_086_3 * l - 0.703_418_614_7 * m + 1.707_614_701_0 * s;

        Self::rgba(
            (Self::to_gamma(r) * 255.0).round() as u8,
            (Self::to_gamma(g) * 255.0).round() as u8,
            (Self::to_gamma(b) * 255.0).round() as u8,
            alpha,
        )
    }

    /// Converts from oklch to RGB
    pub fn from_oklch(l: f64, c: f64, h: f64, alpha: u8) -> Self {
        let a = c * h.to_radians().cos();
        let b = c * h.to_radians().sin();

        Self::from_oklab(l, a, b, alpha)
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
        Color::rgb(
            ((1.0 - self.red) * 255.0).round() as u8,
            ((1.0 - self.green) * 255.0).round() as u8,
            ((1.0 - self.blue) * 255.0).round() as u8,
        )
    }

    /// Returns slit complementary colors.
    pub fn split_complementary_color(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let hsl: palette::Hsl = self.color.into_color();
        colors.push(*self);
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 150) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 210) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors
    }

    /// Returns triadic colors.
    pub fn triadic_colors(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let hsl: palette::Hsl = self.color.into_color();
        colors.push(*self);
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 120) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 240) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors
    }

    /// Returns tetradic colors.
    pub fn tetradic_colors(&self) -> Vec<Self> {
        let mut colors = Vec::with_capacity(2);

        let hsl: palette::Hsl = self.color.into_color();
        colors.push(*self);
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 90) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 180) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors.push(Color::from_hsl(
            (hsl.hue.into_positive_degrees() as u16 + 270) % 360,
            hsl.saturation,
            hsl.lightness,
        ));
        colors
    }

    /// Returns `n` analogous colors, include itself.
    ///
    /// The colors are generated by shifting the hue by 30°.
    pub fn analogous_colors(&self, n: usize) -> Vec<Self> {
        let slices = 30;

        //convert from RGB to HSL
        let hsl: palette::Hsl = self.color.into_color();
        let part = 360 / slices;

        let mut colors = Vec::with_capacity(n);
        colors.push(*self);

        //always shift by at least 1 slice
        for i in 1..n {
            //add hue degrees
            let hue = (hsl.hue.into_positive_degrees() as u16 + part * i as u16) % 360;
            colors.push(Self::from_hsl(hue, hsl.saturation, hsl.lightness));
        }

        colors
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[RGB ({:03}, {:03}, {:03}); Hex (#{:02x}{:02x}{:02x})]",
            (self.color.red * 255.0) as u8,
            (self.color.green * 255.0) as u8,
            (self.color.blue * 255.0) as u8,
            (self.color.red * 255.0) as u8,
            (self.color.green * 255.0) as u8,
            (self.color.blue * 255.0) as u8,
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
        gtk::gdk::RGBA::new(color.red, color.green, color.blue, color.alpha)
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
    fn test_to_cmyk() {
        let color = Color::rgb(46, 52, 64);
        assert_eq!((28.0, 19.0, 0.0, 75.0), color.to_cmyk())
    }
}
