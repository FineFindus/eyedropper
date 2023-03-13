use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::{map_res, opt},
    sequence::Tuple,
    IResult,
};

use super::{color::Color, position::AlphaPosition};

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

pub fn hex_color(input: &str, alpha_position: AlphaPosition) -> IResult<&str, Color> {
    let (input, _) = opt(tag("#"))(input)?;

    if (alpha_position != AlphaPosition::None && input.trim().len() > 8) || input.trim().len() > 6 {
        return Err(nom::Err::Error(nom::error::Error::new(
            "Length is greater than the allowed maximum",
            nom::error::ErrorKind::LengthValue,
        )));
    }

    let (input, first_alpha) = if alpha_position == AlphaPosition::Start && input.len() >= 8 {
        hex_primary(input)?
    } else {
        (input, 255)
    };

    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;

    let alpha = match alpha_position {
        AlphaPosition::None => 255,
        AlphaPosition::Start => first_alpha,
        AlphaPosition::End => opt(hex_primary)(input)?.1.unwrap_or(255),
    };

    Ok((input, Color::rgba(red, green, blue, alpha)))
}
