extern crate hex_literal;
extern crate nom;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::multi::length_data;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::IResult;

pub fn parse_frame(input: &[u8]) -> IResult<&[u8], Message> {
    message(input)
}

pub fn message(input: &[u8]) -> IResult<&[u8], Message> {
    alt((
        mouse_move,
        key_down,
        hello,
        keep_alive,
        query_info,
        info_ack,
        reset_options,
        options,
        enter,
        clipboard,
    ))(input)
}

pub fn key_down(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("DKDN")(input)?;
    let (input, key_id) = be_u16(input)?;
    let (input, key_modifier_mask) = be_u16(input)?;
    let (input, key_button) = be_u16(input)?;
    Ok((
        input,
        Message::Data(Data::KeyDown(KeyDown {
            key_id,
            key_modifier_mask,
            key_button,
        })),
    ))
}

pub fn mouse_move(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("DMMV")(input)?;
    let (input, x) = be_u16(input)?;
    let (input, y) = be_u16(input)?;
    Ok((input, Message::Data(Data::MouseMove(MouseMove { x, y }))))
}

pub fn hello(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("Barrier")(input)?;
    let (input, major) = be_u16(input)?;
    let (input, minor) = be_u16(input)?;
    Ok((
        input,
        Message::Hello(Hello {
            server_version: ProtocolVersion { major, minor },
        }),
    ))
}

pub fn keep_alive(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("CALV")(input)?;
    Ok((input, Message::Command(Command::KeepAlive)))
}

pub fn query_info(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("QINF")(input)?;
    Ok((input, Message::Query(Query::Info)))
}

pub fn info_ack(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("CIAK")(input)?;
    Ok((input, Message::Command(Command::InfoAck)))
}

pub fn reset_options(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("CROP")(input)?;
    Ok((input, Message::Command(Command::ResetOptions)))
}

pub fn options(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("DSOP")(input)?;
    let (input, _) = length_data(be_u32)(input)?;
    Ok((input, Message::Data(Data::Options(Options {}))))
}

pub fn enter(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("CINN")(input)?;
    let (input, x) = be_u16(input)?;
    let (input, y) = be_u16(input)?;
    let (input, sequence_number) = be_u32(input)?;
    let (input, key_modifier_mask) = be_u16(input)?;
    Ok((
        input,
        Message::Command(Command::Enter(Enter {
            x,
            y,
            sequence_number,
            key_modifier_mask,
        })),
    ))
}

pub fn clipboard(input: &[u8]) -> IResult<&[u8], Message> {
    let (input, _) = tag("DCLP")(input)?;
    let (input, clipboard) = be_u8(input)?;
    let (input, sequence_number) = be_u32(input)?;
    let (input, mark) = be_u8(input)?;
    let (input, _) = length_data(be_u32)(input)?;
    Ok((
        input,
        Message::Data(Data::Clipboard(Clipboard {
            clipboard,
            sequence_number,
            mark,
        })),
    ))
}

#[derive(Debug, PartialEq)]
pub enum Message {
    Hello(Hello),
    Query(Query),
    Command(Command),
    Data(Data),
    Error(Error),
}

#[derive(Debug, PartialEq)]
pub enum Query {
    Info,
}

#[derive(Debug, PartialEq)]
pub enum Command {
    KeepAlive,
    InfoAck,
    ResetOptions,
    Enter(Enter),
}

#[derive(Debug, PartialEq)]
pub enum Data {
    MouseMove(MouseMove),
    KeyDown(KeyDown),
    Options(Options),
    Clipboard(Clipboard),
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Busy,
    Unkown,
    Bad,
}

#[derive(Debug, PartialEq)]
pub struct Enter {
    pub x: u16,
    pub y: u16,
    pub sequence_number: u32,
    pub key_modifier_mask: u16,
}

#[derive(Debug, PartialEq)]
pub struct MouseMove {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq)]
pub struct KeyDown {
    pub key_id: u16,
    pub key_modifier_mask: u16,
    pub key_button: u16,
}

#[derive(Debug, PartialEq)]
pub struct Hello {
    pub server_version: ProtocolVersion,
}

#[derive(Debug, PartialEq)]
pub struct ProtocolVersion {
    pub major: u16,
    pub minor: u16,
}

#[derive(Debug, PartialEq)]
pub struct Options {}

#[derive(Debug, PartialEq)]
pub struct Clipboard {
    clipboard: u8,
    sequence_number: u32,
    mark: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    #[test]
    fn hello() {
        // kMsgHello = "Barrier%2i%2i";
        const BYTE_ARRAY: [u8; 11] = hex!("42 61 72 72 69 65 72 00 01 00 06");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((
                &[][..],
                Message::Hello(Hello {
                    server_version: ProtocolVersion { major: 1, minor: 6 }
                })
            ))
        );
    }

    #[test]
    fn command_keep_alive() {
        // kMsgCKeepAlive = "CALV";
        const BYTE_ARRAY: [u8; 4] = hex!("43 41 4c 56");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Command(Command::KeepAlive)))
        );
    }

    #[test]
    fn query_info() {
        // kMsgQInfo = "QINF";
        const BYTE_ARRAY: [u8; 4] = hex!("51 49 4e 46");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Query(Query::Info)))
        );
    }

    #[test]
    fn command_info_ack() {
        // kMsgCInfoAck = "CIAK";
        const BYTE_ARRAY: [u8; 4] = hex!("43 49 41 4b");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Command(Command::InfoAck)))
        );
    }

    #[test]
    fn command_reset_options() {
        // kMsgCResetOptions = "CROP";
        const BYTE_ARRAY: [u8; 4] = hex!("43 52 4f 50");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Command(Command::ResetOptions)))
        );
    }

    #[test]
    fn command_enter() {
        // Enter Screen
        // Screen X: 0
        // Screen Y: 503
        // Sequence Number: 1
        // Modifier Key Mask: 0
        // kMsgCEnter = "CINN%2i%2i%4i%2i";
        const BYTE_ARRAY: [u8; 14] = hex!("43 49 4e 4e 00 00 01 f7 00 00 00 01 00 00");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((
                &[][..],
                Message::Command(Command::Enter(Enter {
                    x: 0,
                    y: 503,
                    sequence_number: 1,
                    key_modifier_mask: 0,
                }))
            ))
        );
    }

    #[test]
    fn data_options() {
        // kMsgDSetOptions = "DSOP%4I";
        const BYTE_ARRAY: [u8; 8] = hex!("44 53 4f 50 00 00 00 00");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Data(Data::Options(Options {}))))
        );
    }

    #[test]
    fn data_clipboard() {
        // kMsgDClipboard = "DCLP%1i%4i%1i%s";
        const BYTE_ARRAY: [u8; 15] = hex!("44 43 4c 50 01 00 00 00 00 01 00 00 00 01 34");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((&[][..], Message::Data(Data::Clipboard(Clipboard {}))))
        );
    }

    #[test]
    fn data_mouse_move() {
        // Mouse Move
        // X Axis: 315
        // Y Axis: 664
        // kMsgDMouseMove = "DMMV%2i%2i";
        const BYTE_ARRAY: [u8; 8] = hex!("44 4d 4d 56 01 3b 02 98");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((
                &[][..],
                Message::Data(Data::MouseMove(MouseMove { x: 315, y: 664 }))
            ))
        );
    }

    #[test]
    fn data_key_down() {
        // Key Pressed
        // Key Id: 99
        // Key Modifier Mask: 2
        // Key Button: 54
        // kMsgDKeyDown = "DKDN%2i%2i%2i";
        const BYTE_ARRAY: [u8; 10] = hex!("44 4b 44 4e 00 63 00 02 00 36");
        assert_eq!(
            message(&BYTE_ARRAY),
            Ok((
                &[][..],
                Message::Data(Data::KeyDown(KeyDown {
                    key_id: 99,
                    key_modifier_mask: 2,
                    key_button: 54
                }))
            ))
        );
    }
}
