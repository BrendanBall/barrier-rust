extern crate nom;

use nom::number::complete::be_i16;
use nom::bytes::complete::{tag};
use nom::{IResult};

pub fn command(input: &[u8]) -> IResult<&[u8],Command> {
    let (input, _) = dbg!(tag("DMMV")(input)?);
    let (input, x) = be_i16(input)?; 
    let (input, y) = be_i16(input)?; 
    Ok((input, Command::MouseMove(MouseMove{x,y})))
}

#[derive(Debug, PartialEq)]
pub enum Command {
    MouseMove(MouseMove),
}

#[derive(Debug, PartialEq)]
pub struct MouseMove {
    pub x: i16,
    pub y: i16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_moved() {
        // Mouse Moved
        // X Axis: 315
        // Y Axis: 664
        // kMsgDMouseMove        = "DMMV%2i%2i";
        // hex = "44 4d 4d 56 01 3b 02 98"
        const BYTE_ARRAY: [u8; 8] = [0x44, 0x4d, 0x4d, 0x56, 0x01, 0x3b, 0x02, 0x98];
        assert_eq!(command(&BYTE_ARRAY), Ok((&[][..], Command::MouseMove(MouseMove{x:315,y:664}))));
    }
}
