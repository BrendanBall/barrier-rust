use barrier::parser::{parse_frame, Message, Query};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:24800") {
        Ok(stream) => {
            println!("Successfully connected to server in port 24800");
            event_loop(stream)
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn event_loop(mut stream: TcpStream) {
    loop {
        let mut frame_size_buffer = [0 as u8; 4];
        match stream.read_exact(&mut frame_size_buffer) {
            Ok(()) => {
                let frame_size = u32::from_be_bytes(frame_size_buffer) as usize;
                println!("receive message with frame size: {:?}", frame_size);

                let mut buffer = vec![0; frame_size];

                match stream.read_exact(&mut buffer[..frame_size]) {
                    Ok(()) => {
                        // println!("receive raw message: {:x?}", &buffer[..frame_size]);
                        let frame = parse_frame(&buffer[..frame_size]);
                        if let Ok(frame) = frame {
                            let message = frame.1;
                            let response = handler(message);
                            match response {
                                Option::Some(response) => {
                                    println!("send raw message: {:x?}", response);
                                    let mut response_buffer = Vec::new();
                                    response_buffer
                                        .extend_from_slice(&(response.len() as u32).to_be_bytes());
                                    response_buffer.extend_from_slice(&response);
                                    stream.write(&response_buffer).unwrap();
                                }
                                Option::None => {}
                            }
                        }
                    }
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Failed to read frame size: {}", e);
                println!("current frame size buffer: {:x?}", frame_size_buffer);
                panic!()
            }
        }
    }
}

fn handler(message: Message) -> Option<Vec<u8>> {
    println!("message: {:?}", message);
    match message {
        Message::Hello(_) => Some(hello_back()),
        Message::Query(Query::Info) => Some(info()),
        _ => None,
    }
}

fn hello_back() -> Vec<u8> {
    const MAJOR: u16 = 1;
    const MINOR: u16 = 6;
    const CLIENT_NAME: &'static [u8] = b"brendan-nom";
    let mut v = Vec::new();
    v.extend_from_slice(b"Barrier");
    v.extend_from_slice(&MAJOR.to_be_bytes()[..]);
    v.extend_from_slice(&MINOR.to_be_bytes()[..]);
    v.extend_from_slice(&(CLIENT_NAME.len() as u32).to_be_bytes()[..]);
    v.extend_from_slice(CLIENT_NAME);
    v
}

fn info() -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"DINF");
    v.extend_from_slice(&(0 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(0 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(2560 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(1440 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(0 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(1280 as u16).to_be_bytes()[..]);
    v.extend_from_slice(&(720 as u16).to_be_bytes()[..]);
    v
}
