extern crate nom;
extern crate hex_literal;

use nom::number::complete::be_i16;
use nom::bytes::complete::{tag};
use nom::{IResult};
use nom::branch::alt;

pub fn command(input: &[u8]) -> IResult<&[u8],Command> {
    alt((mouse_move, key_down))(input)
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

#[derive(Debug, PartialEq)]
pub enum Command {
    MouseMove(MouseMove),
    KeyDown(KeyDown),
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
        // hex = "44 4d 4d 56 01 3b 02 98"
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
        // hex = "44 4b 44 4e 00 63 00 02 00 36"
        const BYTE_ARRAY: [u8; 10] = hex!("44 4b 44 4e 00 63 00 02 00 36");
        assert_eq!(command(&BYTE_ARRAY), Ok((&[][..], Command::KeyDown(KeyDown{key_id:99,key_modifier_mask:2,key_button:54}))));
    }
}
