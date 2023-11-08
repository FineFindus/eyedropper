use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while_m_n},
    character::{
        complete::{digit0, digit1, multispace0},
        is_hex_digit,
    },
    combinator::{map, map_res, opt, recognize, value},
    error::ParseError,
    multi::many_m_n,
    sequence::{delimited, pair, separated_pair, terminated, Tuple},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

use super::{color::Color, illuminant::Illuminant, position::AlphaPosition};

/// Parses a hexadecimal value from a string input and returns the parsed value.
///
/// # Examples
/// ```rust
/// # use crate::colors::parser::hex;
/// let result = hex("FF");
/// assert_eq!(result, Ok(("", 255)));
/// ```
fn hex(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |char| is_hex_digit(char as u8)),
        |str| u8::from_str_radix(str, 16),
    )(input)
}
/// Parses a relative percentage displayed as a decimal, such as `0.5`.
/// The leading digits before the decimal dot are optional.
/// The result is clamped between 0.0 and 1.0.
///
/// # Examples
///
/// ```rust
/// let result = relative_percentage("0.5");
/// assert_eq!(result, Ok(("", 0.5)));
/// ```
fn relative_percentage(input: &str) -> IResult<&str, f32> {
    let (input, digits) = recognize(separated_pair(digit0, tag("."), digit1))(input)?;
    let (_, value) = nom::number::complete::float(digits)?;
    Ok((input, value.clamp(0.0, 1.0)))
}

/// Parses a percentage displayed as a number following a `%`.
/// The result will be clamped between 0 and 1.
///
/// # Examples
/// ```rust
/// let result = percentage("50%");
/// assert_eq!(result, Ok(("", 0.5)));
/// ```
fn percentage(input: &str) -> IResult<&str, f32> {
    let (input, digits) = terminated(digit1, tag("%"))(input)?;
    let (_input, value) = nom::number::complete::float(digits)?;
    Ok((input, (value / 100f32).clamp(0.0, 1.0)))
}

/// Parses a percentage value, such as `-51.6%`.
///
/// The input string should represent a number followed by a `%` symbol.
/// The function handles both positive and negative percentages, as well as fractional parts.
///  The parsed result will be returned as a floating-point number between 0 and 1. It is not clamped, so it is possible
/// that it may be larger.
///
/// # Example
///``` rust
/// let result = parse_percentage("-51.6%");
/// assert_eq!(result, Ok(("", -0.516)));
///```
fn parse_percentage(input: &str) -> IResult<&str, f32> {
    let (input, digits) = map_res(
        terminated(
            separated_pair(pair(opt(tag("-")), digit1), opt(tag(".")), digit0),
            tag("%"),
        ),
        |((sign, int), frac)| {
            let float_str = format!("{}{}.{}", sign.unwrap_or(""), int, frac);
            float_str.parse::<f32>()
        },
    )(input)?;
    Ok((input, (digits / 100f32)))
}
/// Parses different separators used to separate values.
///
/// This function attempts to parse several possible
/// separator values, including `,`, `|`, and `/`.
///
/// # Example
///```rust
/// let result = separator(",");
/// assert_eq!(result, Ok(("", ",")));
/// let result = separator("|");
/// assert_eq!(result, Ok(("", "|")));
/// let result = separator("/");
/// assert_eq!(result, Ok(("", "/")));
///```
fn separator(input: &str) -> IResult<&str, &str> {
    alt((tag(","), tag("|"), tag("/")))(input)
}

/// Parses a CSS-like hue value from a string and returns it as a floating-point number.
///
/// The input string can represent a hue value in either turns or degrees. If the value is specified in turns, it will be multiplied by 360.0 to convert it to degrees. The parsed hue value will be returned as a floating-point number.
///
/// # Examples
///```rust
/// let result = hue("0.5turn");
/// assert_eq!(result, Ok(("", 180.0)));
/// let result = hue("270deg");
/// assert_eq!(result, Ok(("", 270.0)));
/// let result = hue("90°");
/// assert_eq!(result, Ok(("", 90.0)));
///```
fn hue(input: &str) -> IResult<&str, f32> {
    alt((
        map(
            terminated(nom::number::complete::float, tag("turn")),
            |deg| deg * 360.0,
        ),
        terminated(
            nom::number::complete::float,
            opt(alt((tag("deg"), tag("°")))),
        ),
    ))(input)
}

/// Removes whitespace around the given parser, returning the result of the parser.
///
/// Under the hood, it uses [`nom::character::complete::multispace0`] to remove the whitespace.
/// This includes spaces, tabs, carriage returns, and line feeds.
///
/// # Arguments
///
/// * `inner` - The parser to be wrapped and have whitespace removed around it.
///
/// # Examples
///
/// ```
///let inner_parser = tag('a');
///let mut parser = whitespace(inner_parser);
///
/// assert_eq!(parser("   a   "), Ok(("", "a")));
/// ```
pub fn whitespace<I, O, E: ParseError<I>, F>(inner: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    F: Parser<I, O, E>,
    I: InputTakeAtPosition + Clone,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(opt(multispace0), inner, opt(multispace0))
}

pub fn hex_color(input: &str, alpha_position: AlphaPosition) -> IResult<&str, Color> {
    let (input, _) = opt(whitespace(tag("#")))(input)?;

    let (input, first_alpha) = if alpha_position == AlphaPosition::Start && input.len() >= 8 {
        hex(input)?
    } else {
        (input, 255)
    };

    let (input, (red, green, blue)) =
        (whitespace(hex), whitespace(hex), whitespace(hex)).parse(input)?;

    let alpha = match alpha_position {
        AlphaPosition::None => 255,
        AlphaPosition::Start => first_alpha,
        AlphaPosition::End => opt(hex)(input)?.1.unwrap_or(255),
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
/// In case the alpha is not explicitly set, full opacity is assumed.
///
///  The values can be set as
/// - a number in the range of 0 - 255
/// - a float with an optional decimal point or percentage sign
///
/// Mixed value types are allowed.
pub fn rgb(input: &str) -> IResult<&str, Color> {
    let (input, alpha) = whitespace(alt((
        value(AlphaPosition::None, tag("rgb(")),
        value(AlphaPosition::End, tag("rgba(")),
        value(AlphaPosition::Start, tag("argb(")),
    )))(input)?;

    let minimum_length = if alpha == AlphaPosition::None { 3 } else { 4 };

    let (input, mut color_values) = many_m_n(
        minimum_length,
        4,
        terminated(
            whitespace(alt((
                map(alt((percentage, relative_percentage)), |percent| {
                    (percent * 255f32) as u8
                }),
                nom::character::complete::u8,
            ))),
            opt(whitespace(separator)),
        ),
    )(input)?;

    let (input, _output) = opt(whitespace(tag(")")))(input)?;

    //should always be safe to convert, as the length is always at least `minimum_length`, so at least 3
    let color = match alpha {
        AlphaPosition::None | AlphaPosition::End => Color::try_from(color_values),
        AlphaPosition::Start => {
            let alpha = color_values.remove(0);
            color_values.push(alpha);
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
            rgb("argb(100  46 | 52 / 64)")
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
            Ok(("", Color::rgb(127, 127, 127))),
            rgb("rgb(0.5, 0.5, 0.5)")
        );
    }
}

/// Parses a hsl representation of a color.
///
///
/// Mixed value types are allowed.
pub fn hsl(input: &str) -> IResult<&str, Color> {
    let (input, _) = whitespace(alt((tag("hsl("), tag("hsla("))))(input)?;

    let (input, hue) = terminated(whitespace(hue), opt(whitespace(separator)))(input)?;

    let (input, color_values) = many_m_n(
        2,
        2,
        terminated(whitespace(percentage), opt(whitespace(separator))),
    )(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _output) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_hsla(
        hue as u16,
        color_values[0],
        color_values[1],
        alpha.unwrap_or(255),
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_hsl {
    use super::*;

    #[test]
    fn it_parses_basic() {
        assert_eq!(Ok(("", Color::rgb(47, 53, 65))), hsl("hsl(220, 16%, 22%)"));
        assert_eq!(
            Ok(("", Color::rgba(47, 53, 65, 99))),
            hsl("hsl(220, 16%, 22%, 39%)")
        );
        assert_eq!(
            Ok(("", Color::rgba(47, 53, 65, 127))),
            hsl("hsla(220, 16%, 22%, 0.5)")
        );
    }

    #[test]
    fn it_works_with_deg() {
        assert_eq!(Ok(("", Color::rgb(47, 53, 65))), hsl("hsl(220, 16%, 22%)"));
    }
}

/// Parses a hsv representation of a color.
///
///
/// Mixed value types are allowed.
pub fn hsv(input: &str) -> IResult<&str, Color> {
    let (input, _) = whitespace(alt((tag("hsv("), tag("hsva("))))(input)?;

    let (input, hue) = terminated(whitespace(hue), opt(whitespace(separator)))(input)?;

    let (input, color_values) = many_m_n(
        2,
        2,
        terminated(whitespace(percentage), opt(whitespace(separator))),
    )(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _output) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_hsva(
        hue as u16,
        color_values[0],
        color_values[1],
        alpha.unwrap_or(255),
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_hsv {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(Ok(("", Color::rgb(46, 52, 64))), hsv("hsv(220, 28%, 25%)"));
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 127))),
            hsv("hsv(220, 28%, 25%, 50%)")
        );
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 127))),
            hsv("hsva(220, 28%, 25%, 0.5)")
        );
    }
}

/// Parses a cmyk representation of a color.
pub fn cmyk(input: &str) -> IResult<&str, Color> {
    let (input, color_values) = delimited(
        whitespace(tag("cmyk(")),
        many_m_n(
            4,
            4,
            terminated(whitespace(percentage), opt(whitespace(separator))),
        ),
        opt(whitespace(tag(")"))),
    )(input)?;

    let color = Color::from_cmyk(
        color_values[0],
        color_values[1],
        color_values[2],
        color_values[3],
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_cmyk {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            cmyk("cmyk(28%, 19%, 0%, 75%)")
        );
    }
}

/// Parses a xyz representation of a color.
pub fn xyz(input: &str) -> IResult<&str, Color> {
    let (input, color_values) = delimited(
        whitespace(tag_no_case("XYZ(")),
        many_m_n(
            3,
            3,
            terminated(
                whitespace(nom::number::complete::float),
                opt(whitespace(separator)),
            ),
        ),
        opt(whitespace(tag(")"))),
    )(input)?;

    let color = Color::from_xyz(color_values[0], color_values[1], color_values[2], 255);

    Ok((input, color))
}

#[cfg(test)]
mod parse_xyz {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            xyz("XYZ(3.280, 3.407, 5.335)")
        );
    }
}

/// Parses a cielab representation of a color.
pub fn cielab(input: &str, illuminant: Illuminant, ten_deg_observer: bool) -> IResult<&str, Color> {
    let (input, _) = whitespace(alt((tag_no_case("lab("), tag_no_case("cielab("))))(input)?;

    //can either be an percentage or a number between 0 and 100
    let (input, cie_l) = terminated(
        whitespace(alt((
            map(whitespace(parse_percentage), |percentage| {
                percentage * 100.0
            }),
            nom::number::complete::float,
        ))),
        opt(whitespace(separator)),
    )(input)?;

    //both CIE a and CIE b can either be an percentage between -100% and 100% or a number between -125 and 125
    let (input, cie_a_b) = many_m_n(
        2,
        2,
        terminated(
            whitespace(alt((
                map(alt((parse_percentage, percentage)), |percentage| {
                    percentage * 125.0
                }),
                nom::number::complete::float,
            ))),
            opt(whitespace(separator)),
        ),
    )(input)?;

    let (input, alpha) = opt(whitespace(map(
        alt((percentage, relative_percentage)),
        |percentage| (percentage * 255.0) as u8,
    )))(input)?;

    let (input, _) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_cie_lab(
        cie_l.clamp(0.0, 100.0),
        cie_a_b[0].clamp(-125.0, 125.0),
        cie_a_b[1].clamp(-125.0, 125.0),
        alpha.unwrap_or(255),
        illuminant,
        ten_deg_observer,
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_cie_lab {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            cielab(" lab(21.61%, 0.56%,  -6.68%)", Illuminant::default(), false)
        );
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            cielab("lab(21.61, 0.70, -8.35)", Illuminant::default(), false)
        );
    }
}

/// Parses a hwb representation of a color.
pub fn hwb(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("hwb(")(input)?;

    let (input, hue) = terminated(whitespace(hue), opt(whitespace(separator)))(input)?;

    let (input, color_values) = many_m_n(
        2,
        2,
        terminated(whitespace(percentage), opt(whitespace(separator))),
    )(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _output) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_hwba(
        hue as u16,
        color_values[0],
        color_values[1],
        alpha.unwrap_or(255),
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_hwb {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(Ok(("", Color::rgb(46, 52, 64))), hwb("hwb(220, 18%, 75%)"));
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 127))),
            hwb("hwb(220, 18%, 75%, 0.5)")
        );
    }
}

/// Parses a lch representation of a color.
pub fn lch(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("lch(")(input)?;

    let (input, lightness) = terminated(
        alt((
            map(percentage, |percent| percent * 100.0),
            nom::number::complete::float,
        )),
        opt(whitespace(separator)),
    )(input)?;

    let (input, chroma) = terminated(
        alt((
            map(percentage, |percent| percent * 150.0),
            nom::number::complete::float,
        )),
        opt(whitespace(separator)),
    )(input)?;

    let (input, hue) = terminated(hue, opt(whitespace(separator)))(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_lch(lightness, chroma, hue, alpha.unwrap_or(255));

    Ok((input, color))
}

#[cfg(test)]
mod parse_lch {
    use super::*;

    #[test]
    fn it_parses_lch() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            lch("lch(21.605232, 8.378235, 274.76328)")
        );
        assert_eq!(
            Ok(("", Color::rgba(46, 52, 64, 127))),
            lch("lch(21.605232, 8.378235, 274.76328, 0.5)")
        );
    }
}

/// Parses a LMS representation of a color.
pub fn lms(input: &str) -> IResult<&str, Color> {
    let (input, long) = delimited(
        whitespace(tag("L:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;
    let (input, medium) = delimited(
        whitespace(tag("M:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;
    let (input, short) = delimited(
        whitespace(tag("S:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;

    let color = Color::from_lms(long, medium, short, 255);

    Ok((input, color))
}

#[cfg(test)]
mod parse_lms {
    use super::*;

    #[test]
    fn it_parses() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            lms("L: 3.20580, M: 3.52562, S: 5.33522")
        );
    }
}

/// Parses a hunter lab representation of a color.
pub fn hunter_lab(
    input: &str,
    illuminant: Illuminant,
    ten_deg_observer: bool,
) -> IResult<&str, Color> {
    let (input, long) = delimited(
        whitespace(tag("L:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;
    let (input, medium) = delimited(
        whitespace(tag("a:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;
    let (input, short) = delimited(
        whitespace(tag("b:")),
        whitespace(nom::number::complete::float),
        opt(whitespace(separator)),
    )(input)?;

    let color = Color::from_hunter_lab(long, medium, short, illuminant, ten_deg_observer);

    Ok((input, color))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hunter_lab() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            hunter_lab(
                "L: 18.45804, a: 0.41141, b: -5.42239",
                Illuminant::default(),
                false
            )
        );
    }
}

pub fn oklab(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("oklab(")(input)?;

    //lightness can either be an percentage or a number between 0 and 100
    let (input, l) = terminated(
        whitespace(alt((
            map(whitespace(parse_percentage), |percentage| percentage),
            nom::number::complete::float,
        ))),
        opt(whitespace(separator)),
    )(input)?;

    //both a and b can either be an percentage between -100% and 100% or a number between -0.4 and 0.4
    let (input, ok_a_b) = many_m_n(
        2,
        2,
        terminated(
            whitespace(alt((
                map(alt((parse_percentage, percentage)), |percentage| {
                    percentage * 0.4
                }),
                nom::number::complete::float,
            ))),
            opt(whitespace(separator)),
        ),
    )(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_oklab(
        l.clamp(0.0, 1.0),
        ok_a_b[0].clamp(-0.4, 0.4),
        ok_a_b[1].clamp(-0.4, 0.4),
        alpha.unwrap_or(255),
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_oklab {
    use super::*;

    #[test]
    fn parses_oklab() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            oklab("32%, -0.003600, -0.023222")
        );
    }
}

pub fn oklch(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("oklch(")(input)?;

    //lightness can either be an percentage or a number between 0 and 100
    let (input, l) = terminated(
        whitespace(alt((
            map(whitespace(parse_percentage), |percentage| percentage),
            nom::number::complete::float,
        ))),
        opt(whitespace(separator)),
    )(input)?;

    //chroma can be percentage or a value between 0 and 0.4
    let (input, c) = terminated(
        whitespace(alt((
            map(whitespace(parse_percentage), |percentage| percentage * 0.4),
            nom::number::complete::float,
        ))),
        opt(whitespace(separator)),
    )(input)?;

    //hue can be percentage between 0% and 100% or value between 0 and 360
    let (input, h) = terminated(
        whitespace(alt((
            map(whitespace(parse_percentage), |percentage| {
                percentage * 360.0
            }),
            nom::number::complete::float,
        ))),
        opt(whitespace(separator)),
    )(input)?;

    let (input, alpha) = opt(map(
        whitespace(alt((percentage, relative_percentage))),
        |percent| (percent * 255f32) as u8,
    ))(input)?;

    let (input, _) = opt(whitespace(tag(")")))(input)?;

    let color = Color::from_oklch(
        l.clamp(0.0, 1.0),
        c.clamp(0.0, 0.4),
        h.clamp(0.0, 360.0),
        alpha.unwrap_or(255),
    );

    Ok((input, color))
}

#[cfg(test)]
mod parse_oklch {
    use super::*;

    #[test]
    fn parses_oklch() {
        assert_eq!(
            Ok(("", Color::rgb(46, 52, 64))),
            oklch("32%, 0.023499, 261.187836")
        );
    }
}
