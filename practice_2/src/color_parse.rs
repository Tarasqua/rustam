use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Splits a hex color string into its RGB components.
///
/// Example:
/// ```rust
/// use practice_2::color_parse::{from_hex, Color};
///
/// let color = from_hex("ff");
/// assert_eq!(color, Ok(255));
/// ```
pub fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, Color { red, green, blue }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color() {
        let input = "#ff0000";
        let result = hex_color(input);
        assert_eq!(
            result,
            Ok((
                "",
                Color {
                    red: 255,
                    green: 0,
                    blue: 0
                }
            ))
        );
    }
}
