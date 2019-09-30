use barrier::input::{Keyboard, Mouse};
use barrier::parser::{parse_frame, Data, Message, Query};
use serde::{Deserialize, Serialize};
use snafu::{OptionExt, ResultExt, Snafu};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not get XDG base directory: {}", source))]
    ConfigDir { source: xdg::BaseDirectoriesError },
    #[snafu(display("Could not get config file"))]
    ConfigFile {},
    #[snafu(display("Could not merge config: {}", source))]
    MergeConfig { source: config::ConfigError },
    #[snafu(display("Could not parse config: {}", source))]
    DeserializeConfig { source: config::ConfigError },
    #[snafu(display("Create stream failed: {}", source))]
    CreateStreamFailed { source: std::io::Error },
    #[snafu(display("Clone stream failed: {}", source))]
    CloneStreamFailed { source: std::io::Error },
    #[snafu(display("Write to stream failed: {}", source))]
    WriteStreamFailed { source: std::io::Error },
    #[snafu(display("Read from stream failed: {}", source))]
    ReadStreamFailed { source: std::io::Error },
    #[snafu(display("Create device failed: {}", source))]
    CreateDeviceFailed { source: barrier::input::Error },
    #[snafu(display("Handling event failed: {}", source))]
    HandleEvent { source: barrier::input::Error },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    server: ConfigServer,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConfigServer {
    address: String,
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        std::process::exit(2);
    }
}

fn try_main() -> Result<()> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("barrier-rust").context(ConfigDir {})?;
    let config_path = xdg_dirs
        .find_config_file("config.toml")
        .context(ConfigFile {})?;
    let mut settings = config::Config::default();
    settings
        .merge(config::File::from(config_path))
        .context(MergeConfig {})?
        .merge(config::Environment::with_prefix("BARRIER_RUST"))
        .context(MergeConfig {})?;
    let config = settings
        .try_into::<Config>()
        .context(DeserializeConfig {})?;
    println!("{:?}", config);
    run(&config)
}

fn run(config: &Config) -> Result<()> {
    let mouse = Mouse::new(1920, 1080).context(CreateDeviceFailed {})?;
    let keyboard = Keyboard::new().context(CreateDeviceFailed {})?;
    let stream =
        TcpStream::connect(config.server.address.clone()).context(CreateStreamFailed {})?;
    event_loop(stream, mouse, keyboard)
}

fn write_message(mut stream: &TcpStream, message: Vec<u8>) -> Result<()> {
    println!("send raw message: {:x?}", message);
    let mut response_buffer = Vec::new();
    response_buffer.extend_from_slice(&(message.len() as u32).to_be_bytes());
    response_buffer.extend_from_slice(&message);
    stream
        .write(&response_buffer)
        .context(WriteStreamFailed {})?;
    Ok(())
}

fn event_loop(mut stream: TcpStream, mut mouse: Mouse, mut keyboard: Keyboard) -> Result<()> {
    let writer = stream.try_clone().unwrap();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        write_message(&writer, keep_alive()).unwrap();
    });

    loop {
        let mut frame_size_buffer = [0 as u8; 4];
        stream
            .read_exact(&mut frame_size_buffer)
            .context(ReadStreamFailed {})?;
        let frame_size = u32::from_be_bytes(frame_size_buffer) as usize;
        // println!("receive message with frame size: {:?}", frame_size);

        let mut buffer = vec![0; frame_size];
        stream
            .read_exact(&mut buffer[..frame_size])
            .context(ReadStreamFailed {})?;

        // println!("receive raw message: {:x?}", &buffer[..frame_size]);
        let frame = parse_frame(&buffer[..frame_size]);
        match frame {
            Ok(frame) => {
                let message = frame.1;
                let response = handler(message, &mut mouse, &mut keyboard)?;
                match response {
                    Option::Some(response) => {
                        write_message(&stream, response)?;
                    }
                    Option::None => {}
                }
            }
            Err(e) => println!("Failed to parse frame: {:x?}", e),
        }
    }
}

fn handler(
    message: Message,
    mouse: &mut Mouse,
    keyboard: &mut Keyboard,
) -> Result<Option<Vec<u8>>> {
    println!("receive: {:?}", message);
    match message {
        Message::Hello(_) => Ok(Some(hello_back())),
        Message::Query(Query::Info) => Ok(Some(info())),
        Message::Data(Data::MouseMove(mousemove)) => {
            mouse
                .move_abs(mousemove.x as i32, mousemove.y as i32)
                .context(HandleEvent {})?;
            Ok(None)
        }
        Message::Data(Data::MouseDown(mousedown)) => {
            mouse.button_down(mousedown.id).context(HandleEvent {})?;
            Ok(None)
        }
        Message::Data(Data::MouseUp(mouseup)) => {
            mouse.button_up(mouseup.id).context(HandleEvent {})?;
            Ok(None)
        }
        Message::Data(Data::KeyDown(key)) => {
            keyboard.key_down(key.button).context(HandleEvent {})?;
            Ok(None)
        }
        Message::Data(Data::KeyUp(key)) => {
            keyboard.key_up(key.button).context(HandleEvent {})?;
            Ok(None)
        }
        _ => Ok(None),
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

fn keep_alive() -> Vec<u8> {
    let mut v = Vec::new();
    v.extend_from_slice(b"CALV");
    v
}
