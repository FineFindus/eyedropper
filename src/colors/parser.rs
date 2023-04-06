use nom::{
    bytes::complete::{is_not, tag, take_while_m_n},
    character::{complete::multispace0, is_digit, is_hex_digit},
    combinator::{map_res, opt},
    error::ParseError,
    multi::separated_list0,
    sequence::{delimited, Tuple},
    IResult,
};

use super::{color::Color, position::AlphaPosition};

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn from_int(input: &str) -> Result<u8, std::num::ParseIntError> {
    input.parse::<u8>()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(
        take_while_m_n(2, 2, |char| is_hex_digit(char as u8)),
        from_hex,
    )(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    delimited(opt(multispace0), inner, opt(multispace0))
}

fn parse_int(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(1, 3, |char| is_digit(char as u8)), from_int)(input)
}

pub fn hex_color(input: &str, alpha_position: AlphaPosition) -> IResult<&str, Color> {
    let (input, _) = opt(ws(tag("#")))(input)?;

    let (input, first_alpha) = if alpha_position == AlphaPosition::Start && input.len() >= 8 {
        hex_primary(input)?
    } else {
        (input, 255)
    };

    let (input, (red, green, blue)) =
        (ws(hex_primary), ws(hex_primary), ws(hex_primary)).parse(input)?;

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

pub fn rgb(input: &str) -> IResult<&str, Color> {
    let (_input, values) = delimited(ws(tag("rgb(")), is_not(")"), tag(")"))(input)?;

    let (input, color) = separated_list0(ws(tag(",")), parse_int)(values)?;
    log::debug!("Color: {:?}", color);
    Ok((input, Color::random()))
}
