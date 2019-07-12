use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("brendan-nom:24800") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 24800");

            stream.write(b"test").unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 14]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("response: {}", text);
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
