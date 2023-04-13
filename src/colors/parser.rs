use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::{
        complete::{digit1, multispace0},
        is_hex_digit,
    },
    combinator::{map, map_res, opt, value},
    error::ParseError,
    multi::many_m_n,
    sequence::{delimited, terminated, Tuple},
    IResult,
};

use super::{color::Color, position::AlphaPosition};

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |char| is_hex_digit(char as u8)),
        |str| u8::from_str_radix(str, 16),
    )(input)
}

/// Parses values used to specify colors.
///
/// Values can be formatted as a integer value in the range 0 - 255,
/// or as a percent value. Both are returned as a u8.
fn color_value(input: &str) -> IResult<&str, u8> {
    alt((
        map(percentage, |percent| (percent * 255f32) as u8),
        nom::character::complete::u8,
    ))(input)
}

///Parses a percentage `30%` and returns it as a f32 between 0 and 1.
fn percentage(input: &str) -> IResult<&str, f32> {
    let (input, digits) = terminated(digit1, tag("%"))(input)?;
    let (_input, value) = nom::character::complete::u8(digits)?;
    Ok((input, (value as f32 / 100f32).clamp(0.0, 1.0)))
}

/// Removes whitespace around the given parser, returning the result of the parser.
///
/// Under the hood it uses [`nom::character::complete::multispace0`] to remove the whitespace.
/// This includes spaces, tabs, carriage returns and line feeds.
fn whitespace<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(opt(multispace0), inner, opt(multispace0))
}

pub fn hex_color(input: &str, alpha_position: AlphaPosition) -> IResult<&str, Color> {
    let (input, _) = opt(whitespace(tag("#")))(input)?;

    let (input, first_alpha) = if alpha_position == AlphaPosition::Start && input.len() >= 8 {
        hex_primary(input)?
    } else {
        (input, 255)
    };

    let (input, (red, green, blue)) = (
        whitespace(hex_primary),
        whitespace(hex_primary),
        whitespace(hex_primary),
    )
        .parse(input)?;

    let alpha = match alpha_position {
        AlphaPosition::None => 255,
        AlphaPosition::Start => first_alpha,
        AlphaPosition::End => opt(hex_primary)(input)?.1.unwrap_or(255),
    };

    Ok((input, Color::rgba(red, green, blue, alpha)))
}

#[cfg(test)]
mod parse_hex {
    use super::*;

    #[test]
    fn it_parse_hex_without_alpha() {
        assert_eq!(
            Color::rgb(46, 52, 64),
            hex_color("2e3440", AlphaPosition::None).unwrap().1
        );
        assert_eq!(
            Color::rgb(46, 52, 64),
            hex_color("#2e3440", AlphaPosition::None).unwrap().1
        );
    }

    #[test]
    fn it_parse_hex_with_alpha_start() {
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color("282e3440", AlphaPosition::Start).unwrap().1
        );
        assert_eq!(
            Color::rgb(46, 52, 64),
            hex_color("#2e3440", AlphaPosition::None).unwrap().1
        );
    }

    #[test]
    fn it_parse_hex_with_alpha_end() {
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color("2e344028", AlphaPosition::End).unwrap().1
        );
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color("#2e344028", AlphaPosition::End).unwrap().1
        );
    }

    #[test]
    fn success_with_whitespace() {
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color("     #2e344028", AlphaPosition::End).unwrap().1
        );
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color(" # 2e 34 40 28", AlphaPosition::End).unwrap().1
        );
        assert_eq!(
            Color::rgba(46, 52, 64, 40),
            hex_color("2e 34 40 28", AlphaPosition::End).unwrap().1
        );
    }
}

/// Parses a rgb representation of a color.
///
/// This parser accepts CSS like syntax, `rgb`, `rgba`, as well as `argb`.
/// The correct alpha value will be chosen, according to the prepended syntax.
/// Incase the alpha is not explicitly set, full opacity is assumed.
///
///  The values can be set as
/// - a number in the range of 0 - 255
/// - a float with an optional decimal point or percentage sign
///
/// Mixed value types are allowed.
pub fn rgb(input: &str) -> IResult<&str, Color> {
    let (input, alpha) = alt((
        value(AlphaPosition::None, whitespace(tag("rgb("))),
        value(AlphaPosition::End, whitespace(tag("rgba("))),
        value(AlphaPosition::Start, whitespace(tag("argb("))),
    ))(input)?;

    let minimum_length = if alpha == AlphaPosition::None { 3 } else { 4 };

    let (input, mut color_values) = many_m_n(
        minimum_length,
        4,
        terminated(whitespace(color_value), opt(whitespace(tag(",")))),
    )(input)?;

    let (input, _output) = opt(whitespace(tag(")")))(input)?;

    //should always be safe to convert, as the length is always at least `minimum_length`, so at least 3
    let color = match alpha {
        AlphaPosition::None | AlphaPosition::End => Color::try_from(color_values),
        AlphaPosition::Start => {
            color_values.swap(0, 3);
            Color::try_from(color_values)
        }
    }
    .expect("Failed to convert rgb color values to color");

    Ok((input, color))
}

#[cfg(test)]
mod parse_rgb {
    use super::*;

    #[test]
    fn it_parses_basic() {
        assert_eq!(Ok(("", Color::rgb(46, 52, 64))), rgb("rgb(46, 52, 64)"));
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 100))),
            rgb("rgba(46, 52, 64, 100)")
        );
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 100))),
            rgb("argb(100, 46, 52, 64)")
        );
    }
    #[test]
    fn it_parses_percent() {
        assert_eq!(Ok(("", Color::rgb(46, 51, 64))), rgb("rgb(46, 20%, 64)"));
        assert_eq!(
            Ok(("", Color::rgba(45, 51, 63, 255))),
            rgb("rgba(18%, 20%, 25%, 100%)")
        );
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 100))),
            rgb("argb(100, 46, 52, 64)")
        );
    }
}
