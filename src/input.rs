extern crate uinput;

use uinput::device::Device;
use uinput::event::absolute::Absolute::Position;
use uinput::event::absolute::Position::{X, Y};
use uinput::event::controller;
use uinput::event::controller::Controller;
use uinput::event::Event;

pub struct Mouse {
    device: Device,
}

impl Mouse {
    pub fn new(x_maximum: i32, y_maximum: i32) -> Self {
        let device = uinput::open("/dev/uinput")
            .unwrap()
            .name("barrier")
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
        Mouse { device }
    }

    pub fn move_abs(&mut self, x: i32, y: i32) {
        self.device.send(X, x).unwrap();
        self.device.send(Y, y).unwrap();
        self.device.synchronize().unwrap();
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
            .press(&Controller::Mouse(button.into()))
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
            _ => Self::Left,
        }
    }
}

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
