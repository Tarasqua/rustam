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

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?; // INFO: looking for a '#' prefix
    // Runs three 'hex_primary' parsers in a row. Each consumes a part of 'input'
    // and passes the remainder to the next. Results are collected into a tuple,
    // then destuctured into red, green, and blue variables.
    let (input, (red, green, blue)) = (hex_primary, hex_primary, hex_primary).parse(input)?; // so that first `hex_primary` takes 2 chars from `input` returning a `u8` value; then `hex_primary` takes 2 chars from the remainder, etc. -> `(red, green, blue)`
    Ok((input, Color { red, green, blue }))
}

fn main() {
    println!("{:?}", hex_color("#2F14DF"))
}

#[test]
fn parse_color() {
    assert_eq!(
        hex_color("#2F14DF"),
        Ok((
            "",
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        ))
    );
}
