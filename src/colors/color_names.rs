use gtk::glib;
use std::str::FromStr;

use crate::colors::color::Color;

// generated color maps from build.rs
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

#[glib::flags(name = "ColorNameSource")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ColorNameSources {
    /// Basic colors from the original [html spec](https://www.w3.org/TR/1999/REC-html401-19991224/types.html#h-6.5).
    #[flags_value(name = "HTML", nick = "html")]
    Html = 1,
    /// Extended color names from the [SVG spec](https://johndecember.com/html/spec/colorsvg.html).
    #[flags_value(name = "SVG", nick = "svg")]
    Svg = 2,
    /// Names colors from the [GNOME palette](https://developer.gnome.org/hig/reference/palette.html)
    #[flags_value(name = "GNOME", nick = "gnome")]
    Gnome = 4,
    /// Named colors from the xkcd color survey.
    #[flags_value(name = "xkcd", nick = "xkcd")]
    Xkcd = 8,
}

/// Returns the corresponding name for a given [`Color`].
///
/// The color is searched in all the enabled palettes, in the order they are listed in.
/// If none is found [`None`] is returned.
pub fn name(color: Color, sources: ColorNameSources) -> Option<String> {
    let hex = color.hex().to_ascii_lowercase();

    let palettes = [
        (ColorNameSources::Html, &BASIC_VALUES),
        (ColorNameSources::Svg, &SVG_VALUES),
        (ColorNameSources::Gnome, &GNOME_VALUES),
        (ColorNameSources::Xkcd, &XKCD_VALUES),
    ];

    palettes
        .iter()
        .filter(|&&(flag, _)| sources.contains(flag))
        .find_map(|&(_, palette)| palette.get(&hex).map(|val| val.to_string()))
}

/// Returns the corresponding [`Color`] for a given name.
///
/// The color is searched in all the enabled palettes, in the order they are listed in.
/// If none is found [`None`] is returned.
pub fn color(name: &str, sources: ColorNameSources) -> Option<Color> {
    let palettes = [
        (ColorNameSources::Html, &BASIC),
        (ColorNameSources::Svg, &SVG),
        (ColorNameSources::Gnome, &GNOME),
        (ColorNameSources::Xkcd, &XKCD),
    ];

    palettes
        .iter()
        .filter(|&&(flag, _)| sources.contains(flag))
        .filter_map(|&(_, palette)| palette.get(&name.to_ascii_lowercase()))
        .find_map(|val| Color::from_str(val).ok())
}
