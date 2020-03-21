use std::thread;
use std::time::Duration;
use uinput::event::absolute::Absolute::Position;
use uinput::event::absolute::Position::{X, Y};
use uinput::event::controller::Controller::Mouse;
use uinput::event::controller::Mouse::Left;
use uinput::event::Event::{Absolute, Controller};

fn main() {
    let mut device = uinput::open("/dev/uinput")
        .unwrap()
        .name("test")
        .unwrap()
        .event(Controller(Mouse(Left)))
        .unwrap() // It's necessary to enable any mouse button. Otherwise Relative events would not work.
        .event(Absolute(Position(X)))
        .unwrap()
        .max(1920)
        .event(Absolute(Position(Y)))
        .unwrap()
        .max(1080)
        .create()
        .unwrap();

    thread::sleep(Duration::from_secs(1));
    for i in 0..50 {
        thread::sleep(Duration::from_millis(50));

        device.send(X, 1000 + (i * 10)).unwrap();
        device.send(Y, 500 + (i * 10)).unwrap();
        device.synchronize().unwrap();
    }
}
