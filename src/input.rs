extern crate uinput;

use std::thread;
use std::time::Duration;
use uinput::device::Device;
use uinput::event::absolute::Absolute::Position;
use uinput::event::absolute::Position::{X, Y};
use uinput::event::controller::Controller;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event;

pub struct Mouse {
    minimum: i32,
    maximum: i32,
    device: Device,
}

impl Mouse {
    pub fn new(maximum: i32) -> Self {
        const minimum: i32 = 0;
        let mut device = uinput::open("/dev/uinput")
            .unwrap()
            .name("barrier")
            .unwrap()
            .event(Event::Controller(Controller::Mouse(Left)))
            .unwrap()
            .event(Event::Absolute(Position(X)))
            .unwrap()
            .max(1920)
            .event(Event::Absolute(Position(Y)))
            .unwrap()
            .max(1080)
            .create()
            .unwrap();
        Mouse {
            minimum,
            maximum,
            device,
        }
    }

    pub fn moveAbs(&mut self, x: i32, y: i32) {
        self.device.send(X, x).unwrap();
        self.device.send(Y, y).unwrap();
        self.device.synchronize().unwrap();
    }
}
