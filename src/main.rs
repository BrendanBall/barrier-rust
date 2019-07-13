extern crate parser;

use parser::{parse_frame, Message};
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("127.0.0.1:24800") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 24800");

            let mut buff = [0 as u8; 128];
            match stream.read(&mut buff) {
                Ok(n) => {
                    let frame = parse_frame(&buff[..n]);
                    if let Ok(frame) = frame {
                        let message = frame.1;
                        match message {
                            Message::Hello(_) => {
                                let resp = b"Barrier";
                                stream.write(resp);
                            }
                            other => println!("response: {:?}", other),
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}
