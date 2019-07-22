extern crate uinput;

use std::thread;
use std::time::Duration;
// use uinput::event::absolute::Absolute::Position;
// use uinput::event::absolute::Position::{X, Y};
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::relative::Position::{X, Y};
use uinput::event::relative::Relative::Position;
use uinput::event::Event::{Absolute, Controller, Relative};

fn main() {
    let mut device = uinput::open("/dev/uinput")
        .unwrap()
        .name("test")
        .unwrap()
        .event(Controller(Mouse(Left)))
        .unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        // .event(Absolute(Position(X)))
        .event(Relative(Position(X)))
        .unwrap()
        // .event(Absolute(Position(Y)))
        .event(Relative(Position(Y)))
        .unwrap()
        .create()
        .unwrap();

    for _ in 0..50 {
        thread::sleep(Duration::from_millis(15));

        device.send(X, 5).unwrap();
        device.send(Y, 5).unwrap();
        device.synchronize().unwrap();
    }
}
