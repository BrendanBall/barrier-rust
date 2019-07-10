extern crate nom;
extern crate hex_literal;

use nom::number::complete::be_i16;
use nom::bytes::complete::{tag};
use nom::{IResult};
use nom::branch::alt;

pub fn command(input: &[u8]) -> IResult<&[u8],Command> {
    alt((mouse_move, key_down, hello))(input)
}

pub fn key_down(input: &[u8]) -> IResult<&[u8],Command> {
    let (input, _) = tag("DKDN")(input)?;
    let (input, key_id) = be_i16(input)?;
    let (input, key_modifier_mask) = be_i16(input)?;
    let (input, key_button) = be_i16(input)?;
    Ok((input, Command::KeyDown(KeyDown{key_id,key_modifier_mask,key_button})))
}

pub fn mouse_move(input: &[u8]) -> IResult<&[u8],Command> {
    let (input, _) = tag("DMMV")(input)?;
    let (input, x) = be_i16(input)?;
    let (input, y) = be_i16(input)?;
    Ok((input, Command::MouseMove(MouseMove{x,y})))
}

pub fn hello(input: &[u8]) -> IResult<&[u8],Command> {
    let (input, _) = tag("Barrier")(input)?;
    let (input, major) = be_i16(input)?;
    let (input, minor) = be_i16(input)?;
    Ok((input, Command::Hello(Hello{server_version: ProtocolVersion{major,minor}})))
}

#[derive(Debug, PartialEq)]
pub enum Command {
    MouseMove(MouseMove),
    KeyDown(KeyDown),
    Hello(Hello),
}

#[derive(Debug, PartialEq)]
pub struct MouseMove {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, PartialEq)]
pub struct KeyDown {
    pub key_id: i16,
    pub key_modifier_mask: i16,
    pub key_button: i16,
}

#[derive(Debug, PartialEq)]
pub struct Hello {
    pub server_version: ProtocolVersion,
}

#[derive(Debug, PartialEq)]
pub struct ProtocolVersion {
    pub major: i16,
    pub minor: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn mouse_move() {
        // Mouse Move
        // X Axis: 315
        // Y Axis: 664
        // kMsgDMouseMove = "DMMV%2i%2i";
        const BYTE_ARRAY: [u8; 8] = hex!("44 4d 4d 56 01 3b 02 98");
        assert_eq!(command(&BYTE_ARRAY), Ok((&[][..], Command::MouseMove(MouseMove{x:315,y:664}))));
    }

    #[test]
    fn key_down() {
        // Key Pressed
        // Key Id: 99
        // Key Modifier Mask: 2
        // Key Button: 54
        // kMsgDKeyDown = "DKDN%2i%2i%2i";
        const BYTE_ARRAY: [u8; 10] = hex!("44 4b 44 4e 00 63 00 02 00 36");
        assert_eq!(command(&BYTE_ARRAY), Ok((&[][..], Command::KeyDown(KeyDown{key_id:99,key_modifier_mask:2,key_button:54}))));
    }

    #[test]
    fn hello() {
        // kMsgHello = "Barrier%2i%2i";
        const BYTE_ARRAY: [u8; 11] = hex!("42 61 72 72 69 65 72 00 01 00 06");
        assert_eq!(command(&BYTE_ARRAY), Ok((&[][..], Command::Hello(Hello{server_version: ProtocolVersion{major:1,minor:6}}))));
    }
}
