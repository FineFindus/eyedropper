use core::fmt;
use std::str::FromStr;

use palette::{IntoColor, WithAlpha};

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
        Self::rgba(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            255,
        )
    }

    pub fn from_palette(color: impl palette::IntoColor<palette::Srgba>) -> Self {
        Self(color.into_color())
    }

    pub fn hex(&self) -> String {
        format!(
            "#{:02x}{:02x}{:02x}",
            (self.color.red * 255.0) as u8,
            (self.color.green * 255.0) as u8,
            (self.color.blue * 255.0) as u8,
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
        //TODO: remove this once palette supports LMS in the next version
        let xyz: palette::Xyz = self.color.into_color();

        //TODO: use illuminant depended matrices, found here http://brucelindbloom.com/index.html?Eqn_ChromAdapt.html
        let long = xyz.x * 0.3897 + xyz.y * 0.6890 + xyz.z * -0.0787;
        let medium = xyz.x * -0.2298 + xyz.y * 1.1834 + xyz.z * 0.0464;
        let short = xyz.x * 0.0 + xyz.y * 0.0 + xyz.z * 1.0;

        (long, medium, short)
    }

    pub fn from_lms(long: f32, medium: f32, short: f32, alpha: u8) -> Self {
        let x = long * 1.9102 + medium * -1.1121 + short * 0.2019;
        let y = long * 0.3710 + medium * 0.6291 + short * 0.0;
        let z = long * 0.0 + medium * 0.0 + short * 1.0;

        Color::from_palette(palette::Xyza::new(x, y, z, alpha as f32 / 255.0))
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#{:02x}{:02x}{:02x}",
            (self.color.red * 255.0) as u8,
            (self.color.green * 255.0) as u8,
            (self.color.blue * 255.0) as u8,
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
        gtk::gdk::RGBA::new(color.red, color.green, color.blue, color.alpha)
    }
}

impl FromStr for Color {
    type Err = palette::rgb::FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rgb: palette::Srgb<u8> = s.parse()?;
        Ok(Color::from_palette(
            rgb.with_alpha(1.0).into_format::<f32, f32>(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorError {
    HexConversion(String),
    ParsingError(String),
}

impl From<nom::Err<nom::error::Error<&str>>> for ColorError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::ParsingError(value.to_string())
    }
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
