extern crate uinput;

use uinput::device::Device;
use uinput::event::absolute::Absolute::Position;
use uinput::event::absolute::Position::{X, Y};
use uinput::event::controller;
use uinput::event::controller::Controller;
use uinput::event::keyboard;
use uinput::event::{Event, Kind, Code};

pub struct Mouse {
    device: Device,
}

impl Mouse {
    pub fn new(x_maximum: i32, y_maximum: i32) -> Self {
        let device = uinput::open("/dev/uinput")
            .unwrap()
            .name("barrier mouse")
            .unwrap()
            .event(Event::Controller(Controller::Mouse(
                controller::Mouse::Left,
            )))
            .unwrap()
            .event(Event::Absolute(Position(X)))
            .unwrap()
            .max(x_maximum)
            .event(Event::Absolute(Position(Y)))
            .unwrap()
            .max(y_maximum)
            .create()
            .unwrap();
        Self { device }
    }

    pub fn move_abs(&mut self, x: i32, y: i32) {
        // self.device.send(X, x).unwrap();
        // self.device.send(Y, y).unwrap();
        // self.device.synchronize().unwrap();
    }

    pub fn button_down(&mut self, button: impl Into<MouseButton>) {
        let button = button.into();
        self.device
            .press(&Controller::Mouse(button.into()))
            .unwrap();
        self.device.synchronize().unwrap();
    }

    pub fn button_up(&mut self, button: impl Into<MouseButton>) {
        let button = button.into();
        self.device
            .release(&Controller::Mouse(button.into()))
            .unwrap();
        self.device.synchronize().unwrap();
    }
}

impl Into<controller::Mouse> for MouseButton {
    fn into(self) -> controller::Mouse {
        controller::Mouse::Left
    }
}

impl From<u8> for MouseButton {
    fn from(id: u8) -> Self {
        match id {
            1 => Self::Left,
            2 => Self::Middle,
            3 => Self::Right,
            4 => Self::Extra,
            id => {
                println!("unkown button id: {:?}", id);
                Self::Left
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Side,
    Extra,
    Forward,
    Back,
    Task,
}

pub struct Keyboard {
    device: Device,
}

#[derive(Debug, PartialEq)]
pub enum Key {
    A,
}



// Key_A: 30
// message: Data(KeyDown(Key { id: 97, modifier_mask: 0, button: 38 }))
// message: Data(KeyUp(Key { id: 97, modifier_mask: 0, button: 38 }))
// >>> hex(38)
// '0x26'
// >>> hex(97)
// '0x61'

// Key_S: 31
// message: Data(KeyDown(Key { id: 115, modifier_mask: 0, button: 39 }))
// message: Data(KeyUp(Key { id: 115, modifier_mask: 0, button: 39 }))

// Key_Esc: 1
// message: Data(KeyDown(Key { id: 61211, modifier_mask: 0, button: 9 }))
// message: Data(KeyUp(Key { id: 61211, modifier_mask: 0, button: 9 }))
// KEY_LEFTCTRL: 29
// message: Data(KeyDown(Key { id: 61411, modifier_mask: 0, button: 37 }))
// message: Data(KeyUp(Key { id: 61411, modifier_mask: 2, button: 37 }))
// KEY_LEFTALT: 56
// message: Data(KeyDown(Key { id: 61417, modifier_mask: 0, button: 64 }))
// message: Data(KeyUp(Key { id: 61417, modifier_mask: 4, button: 64 }))

// looks like formula is button + 8
impl From<u16> for Key {
    fn from(id: u16) -> Self {
        match id {
            _ => Self::A,
        }
    }
}

impl Into<keyboard::Key> for Key {
    fn into(self) -> keyboard::Key {
        keyboard::Key::A
    }
}

impl Keyboard {
    pub fn new() -> Self {
        let device = uinput::open("/dev/uinput")
            .unwrap()
            .name("barrier keyboard")
            .unwrap()
            .event(uinput::event::Keyboard::All)
            .unwrap()
            .create()
            .unwrap();
        Self { device }
    }

    pub fn key_down(&mut self, button: impl Into<Key>) {
        let key = button.into();
        let key: keyboard::Key = key.into();
        self.device.press(&key).unwrap();
        self.device.synchronize().unwrap();
    }

    pub fn key_up(&mut self, button: impl Into<Key>) {
        // let key = button.into();
        // let key: keyboard::Key = key.into();
        // self.device.release(&key).unwrap();
        // self.device.synchronize().unwrap();
    }
}
