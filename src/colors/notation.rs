use std::str::FromStr;

use gettextrs::gettext;
use gtk::{gio, glib, prelude::SettingsExt};
use palette::IntoColor;

use crate::{
    colors::{cmyk::Cmyka, hunterlab::HunterLab},
    config,
    widgets::preferences::color_format::ColorFormatObject,
};

use super::{
    color::{Color, ColorError},
    color_names::{self, ColorNameSources},
    parser,
    position::AlphaPosition,
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, glib::Enum)]
#[enum_type(name = "Notation")]
pub enum Notation {
    #[default]
    Hex,
    Rgb,
    Hsl,
    Hsv,
    Cmyk,
    Xyz,
    Lab,
    Hwb,
    Hcl,
    Name,
    Lms,
    HunterLab,
    Oklab,
    Oklch,
}

impl Notation {
    pub fn parse(&self, input: &str, name_sources: ColorNameSources) -> Result<Color, ColorError> {
        let settings = gio::Settings::new(config::APP_ID);
        let (_, color) = match self {
            Notation::Hex => parser::hex_color(
                input,
                AlphaPosition::from(settings.int("alpha-position") as u32),
            ),
            Notation::Rgb => parser::rgb(input),
            Notation::Hsl => parser::hsl(input),
            Notation::Hsv => parser::hsv(input),
            Notation::Cmyk => parser::cmyk(input),
            Notation::Xyz => parser::xyz(input),
            Notation::Lab => parser::cielab(input),
            Notation::Hwb => parser::hwb(input),
            Notation::Hcl => parser::lch(input),
            Notation::Lms => parser::lms(input),
            Notation::HunterLab => parser::hunter_lab(input),
            Notation::Oklab => parser::oklab(input),
            Notation::Oklch => parser::oklch(input),
            Notation::Name => {
                return color_names::color(input, name_sources)
                    .ok_or(ColorError::ParsingError("No name found".to_owned()));
            }
        }?;
        Ok(color)
    }

    pub fn as_str(
        &self,
        color: Color,
        alpha_position: AlphaPosition,
        rgb_decimal_notation: bool,
        precision: usize,
        name_sources: ColorNameSources,
    ) -> String {
        let percent = |value: f32| (value * 100.0).round();
        let pretty_percent = |value: f32| match value {
            1.0 => "1".to_string(),
            0.0 => "0".to_string(),
            _ => format!("{:.2}", value),
        };

        match self {
            Notation::Hex => {
                let hex = |value: f32| format!("{:02X}", (value * 255.0) as u8);
                let (r, g, b, a) = (
                    hex(color.red),
                    hex(color.green),
                    hex(color.blue),
                    hex(color.alpha),
                );
                match alpha_position {
                    AlphaPosition::Start => format!("#{}{}{}{}", a, r, g, b),
                    AlphaPosition::End => format!("#{}{}{}{}", r, g, b, a),
                    AlphaPosition::None => format!("#{}{}{}", r, g, b),
                }
            }
            Notation::Rgb => {
                let format_rgb = |val: f32| {
                    if rgb_decimal_notation {
                        format!("{:.2}", val)
                    } else {
                        format!("{}", (val * 255.0).round() as u8)
                    }
                };
                let (r, g, b, a) = (
                    format_rgb(color.red),
                    format_rgb(color.green),
                    format_rgb(color.blue),
                    pretty_percent(color.alpha),
                );
                match alpha_position {
                    AlphaPosition::End => {
                        format!("rgba({}, {}, {}, {})", r, g, b, a)
                    }
                    _ => format!("rgb({}, {}, {})", r, g, b),
                }
            }
            Notation::Hsl => {
                let hsl: palette::Hsl = color.color.into_color();
                let (h, s, l) = (
                    hsl.hue.into_positive_degrees(),
                    percent(hsl.saturation),
                    percent(hsl.lightness),
                );
                match alpha_position {
                    AlphaPosition::End => format!(
                        "hsla({}, {}%, {}%, {})",
                        h,
                        s,
                        l,
                        pretty_percent(color.alpha)
                    ),
                    _ => format!("hsl({}, {}%, {}%)", h, s, l),
                }
            }
            Notation::Hsv => {
                let hsv: palette::Hsv = color.color.into_color();
                format!(
                    "hsv({}, {}%, {}%)",
                    hsv.hue.into_positive_degrees(),
                    percent(hsv.saturation),
                    percent(hsv.value)
                )
            }
            Notation::Cmyk => {
                let cmyk: Cmyka = color.color.into_color();
                format!(
                    "cmyk({}%, {}%, {}%, {}%)",
                    percent(cmyk.cyan),
                    percent(cmyk.magenta),
                    percent(cmyk.yellow),
                    percent(cmyk.k)
                )
            }
            Notation::Xyz => {
                let xyz: palette::Xyz = color.color.into_color();
                format!(
                    "XYZ({:.precision$}, {:.precision$}, {:.precision$})",
                    xyz.x * 100.0,
                    xyz.y * 100.0,
                    xyz.z * 100.0,
                )
            }
            Notation::Lab => {
                let lab: palette::Lab = color.color.into_color();
                format!(
                    "lab({:.precision$}, {:.precision$}, {:.precision$})",
                    lab.l, lab.a, lab.b,
                )
            }
            Notation::Hwb => {
                let hwb: palette::Hwb = color.color.into_color();
                format!(
                    "hwb({}, {}%, {}%)",
                    hwb.hue.into_positive_degrees(),
                    percent(hwb.whiteness),
                    percent(hwb.blackness)
                )
            }
            Notation::Hcl => {
                let lch: palette::Lch = color.color.into_color();
                format!(
                    "lch({:.precision$}, {:.precision$}, {:.precision$})",
                    lch.l,
                    lch.chroma,
                    lch.hue.into_positive_degrees(),
                )
            }
            Notation::Lms => {
                let (l, m, s) = color.to_lms();
                format!(
                    "L: {:.precision$}, M: {:.precision$}, S: {:.precision$}",
                    l, m, s,
                )
            }
            Notation::HunterLab => {
                let lab: HunterLab = color.color.into_color();
                format!(
                    "L: {:.precision$}, a: {:.precision$}, b: {:.precision$}",
                    lab.l, lab.a, lab.b,
                )
            }
            Notation::Oklab => {
                let oklab: palette::Oklab = color.color.into_color();
                match alpha_position {
                    AlphaPosition::End => format!(
                        "oklab({}% {:.precision$} {:.precision$} / {})",
                        percent(oklab.l),
                        oklab.a,
                        oklab.b,
                        pretty_percent(percent(color.alpha) / 100.0),
                    ),
                    _ => format!(
                        "oklab({}% {:.precision$} {:.precision$})",
                        percent(oklab.l),
                        oklab.a,
                        oklab.b,
                    ),
                }
            }
            Notation::Oklch => {
                let oklch: palette::Oklch = color.color.into_color();
                match alpha_position {
                    AlphaPosition::End => format!(
                        "oklch({}% {:.precision$} {:.precision$} / {})",
                        percent(oklch.l),
                        oklch.chroma,
                        oklch.hue.into_positive_degrees(),
                        pretty_percent(percent(color.alpha) / 100.0),
                    ),
                    _ => format!(
                        "oklch({}% {:.precision$} {:.precision$})",
                        percent(oklch.l),
                        oklch.chroma,
                        oklch.hue.into_positive_degrees(),
                    ),
                }
            }
            Notation::Name => {
                color_names::name(color, name_sources).unwrap_or_else(|| gettext("Not named"))
            }
        }
    }

    pub fn display_copy_string(&self) -> String {
        gettext(match self {
            Notation::Hex => "Copy Hex Code",
            Notation::Rgb => "Copy RGB",
            Notation::Hsl => "Copy HSL",
            Notation::Hsv => "Copy HSV",
            Notation::Cmyk => "Copy CMYK",
            Notation::Xyz => "Copy XYZ",
            Notation::Lab => "Copy CIELAB",
            Notation::Hwb => "Copy HWB",
            Notation::Hcl => "Copy CIELCh / HCL",
            Notation::Lms => "Copy LMS",
            Notation::HunterLab => "Copy Hunter Lab",
            Notation::Oklab => "Copy Oklab",
            Notation::Oklch => "Copy Oklch",
            Notation::Name => "Copy Name",
        })
    }

    pub fn to_color_format_object(self, identifier: String, color: Color) -> ColorFormatObject {
        ColorFormatObject::new(
            identifier,
            match self {
                Notation::Hex => gettext("Hex Code"),
                Notation::Rgb => "RGB".to_string(),
                Notation::Hsl => "HSL".to_string(),
                Notation::Hsv => "HSV".to_string(),
                Notation::Cmyk => "CMYK".to_string(),
                Notation::Xyz => "XYZ".to_string(),
                Notation::Lab => "CIELAB".to_string(),
                Notation::Hwb => "HWB".to_string(),
                Notation::Hcl => "CIELCh / HCL".to_string(),
                Notation::Lms => "LMS".to_string(),
                Notation::HunterLab => "Hunter Lab".to_string(),
                Notation::Oklab => "Oklab".to_string(),
                Notation::Oklch => "Oklch".to_string(),
                Notation::Name => gettext("Name"),
            },
            self.as_str(
                color,
                AlphaPosition::None,
                false,
                2,
                ColorNameSources::empty(),
            ),
        )
    }
}

impl FromStr for Notation {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().trim() {
            "hex" => Self::Hex,
            "rgb" => Self::Rgb,
            "hsl" => Self::Hsl,
            "hsv" => Self::Hsv,
            "cmyk" => Self::Cmyk,
            "xyz" => Self::Xyz,
            "cielab" => Self::Lab,
            "hwb" => Self::Hwb,
            "hcl" => Self::Hcl,
            "name" => Self::Name,
            "lms" => Self::Lms,
            "hunterlab" => Self::HunterLab,
            "oklab" => Self::Oklab,
            "oklch" => Self::Oklch,
            _ => {
                log::error!("Failed to parse notation: {}", s);
                return Err(ColorError::ParsingError(
                    "Failed to get color notation".to_owned(),
                ));
            }
        })
    }
}
