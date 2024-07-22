use std::str::FromStr;

use crate::colors::color::Color;

// generated color maps from build.rs
include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

/// Returns the corresponding name for a given [`Color`].
///
/// The color is searched in all the enabled palettes, in the order they are listed in.
/// If none is found [`None`] is returned.
pub fn name(
    color: Color,
    basic: bool,
    extended: bool,
    gnome_palette: bool,
    xkcd: bool,
) -> Option<String> {
    let hex = color.hex().to_ascii_lowercase();

    let palettes = [
        (basic, &BASIC_VALUES),
        (extended, &SVG_VALUES),
        (gnome_palette, &GNOME_VALUES),
        (xkcd, &XKCD_VALUES),
    ];

    palettes
        .iter()
        .filter(|&&(flag, _)| flag)
        .find_map(|&(_, palette)| palette.get(&hex).map(|val| val.to_string()))
}

/// Returns the corresponding [`Color`] for a given name.
///
/// The color is searched in all the enabled palettes, in the order they are listed in.
/// If none is found [`None`] is returned.
pub fn color(
    name: &str,
    basic: bool,
    extended: bool,
    gnome_palette: bool,
    xkcd: bool,
) -> Option<Color> {
    let palettes = [
        (basic, &BASIC),
        (extended, &SVG),
        (gnome_palette, &GNOME),
        (xkcd, &XKCD),
    ];

    palettes
        .iter()
        .filter(|&&(flag, _)| flag)
        .filter_map(|&(_, palette)| palette.get(&name.to_ascii_lowercase()))
        .find_map(|val| Color::from_str(val).ok())
}
