use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_SYN};
use evdev_rs::{AbsInfo, Device, InputEvent, TimeVal, UInputDevice};
use std::thread;
use std::time::Duration;

fn main() {
    let device = Device::new().unwrap();
    device.set_name("barrier-rust");
    device.enable(&EventCode::EV_KEY(EV_KEY::BTN_LEFT)).unwrap();
    device.enable(&EventType::EV_ABS).unwrap();
    device
        .enable_event_code(
            &EventCode::EV_ABS(EV_ABS::ABS_X),
            Some(&AbsInfo {
                value: 0,
                minimum: 0,
                maximum: 1920,
                fuzz: 0,
                flat: 0,
                resolution: 0,
            }),
        )
        .unwrap();
    device
        .enable_event_code(
            &EventCode::EV_ABS(EV_ABS::ABS_Y),
            Some(&AbsInfo {
                value: 0,
                minimum: 0,
                maximum: 1080,
                fuzz: 0,
                flat: 0,
                resolution: 0,
            }),
        )
        .unwrap();

    let uinput_device = UInputDevice::create_from_device(&device).unwrap();
    thread::sleep(Duration::from_secs(1));
    println!("{:?}", uinput_device.devnode());

    for i in 0..50 {
        thread::sleep(Duration::from_millis(50));

        uinput_device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_X),
                1000 + (i * 10),
            ))
            .unwrap();
        uinput_device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_Y),
                500 + (i * 10),
            ))
            .unwrap();
        uinput_device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
    }
}
