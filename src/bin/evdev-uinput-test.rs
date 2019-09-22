extern crate evdev_sys as raw;

use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_SYN};
use evdev_rs::util::event_code_to_int;
use evdev_rs::{AbsInfo, Device};
use libc::c_int;
use nix::errno::Errno;
use std::thread;
use std::time::Duration;

// const EV_SYN: u32 = 0x00;
// const SYN_REPORT: u32 = 0;
// const EV_KEY: u32 = 0x01;
// const BTN_LEFT: u32 = 0x110;
// const EV_ABS: u32 = 0x03;
// const ABS_X: u32 = 0x00;
// const ABS_Y: u32 = 0x01;

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
    for i in 0..50 {
        thread::sleep(Duration::from_millis(50));

        uinput_device
            .write_event(&EventCode::EV_ABS(EV_ABS::ABS_X),
                1000 + (i * 10),
            )
            .unwrap();
        uinput_device
            .write_event(&EventCode::EV_ABS(EV_ABS::ABS_Y),
                500 + (i * 10),
            )
            .unwrap();
        uinput_device
            .write_event(&EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            )
            .unwrap();
    }
}

/// The event structure itself
#[derive(Clone, Debug, PartialEq)]
pub struct InputEvent {
    pub event_type: EventType,
    pub event_code: EventCode,
    pub value: i32,
}

struct UInputDevice {
    raw: *mut raw::libevdev_uinput,
}

impl UInputDevice {
    pub fn create_from_device(device: &Device) -> Result<UInputDevice, Errno> {
        let mut libevdev_uinput: *mut raw::libevdev_uinput = &mut raw::libevdev_uinput {};
        let result = unsafe {
            raw::libevdev_uinput_create_from_device(
                device.raw,
                raw::LIBEVDEV_UINPUT_OPEN_MANAGED,
                &mut libevdev_uinput,
            )
        };

        match result {
            0 => Ok(UInputDevice {
                raw: libevdev_uinput,
            }),
            error => Err(Errno::from_i32(-error)),
        }
    }

    pub fn write_event(&self, code: &EventCode, ev_value: i32) -> Result<(), Errno> {
        let (ev_type, ev_code) = event_code_to_int(code);
        let result =
            unsafe { raw::libevdev_uinput_write_event(self.raw, ev_type, ev_code, ev_value) };

        match result {
            0 => Ok(()),
            error => Err(Errno::from_i32(-error)),
        }
    }
}

impl Drop for UInputDevice {
    fn drop(&mut self) {
        unsafe {
            raw::libevdev_uinput_destroy(self.raw);
        }
    }
}

// struct Device {
//     raw: *mut raw::libevdev,
// }

// fn check(code: i32) {
//     match code {
//         0 => {}
//         error => {
//             println!("error {:?}", Errno::from_i32(-error));
//             panic!();
//         }
//     }
// }

// impl Device {
//     fn new() -> Self {
//         let device = unsafe { raw::libevdev_new() };

//         if device.is_null() {
//             panic!()
//         }
//         unsafe {
//             check(raw::libevdev_enable_event_type(device, EV_KEY));
//             check(raw::libevdev_enable_event_code(
//                 device,
//                 EV_KEY,
//                 BTN_LEFT,
//                 std::ptr::null(),
//             ));

//             check(raw::libevdev_enable_event_type(device, EV_ABS));
//             let mut abs_x_info = raw::input_absinfo {
//                 value: 0,
//                 minimum: 0,
//                 maximum: 1920,
//                 fuzz: 0,
//                 flat: 0,
//                 resolution: 0,
//             };
//             let mut abs_y_info = raw::input_absinfo {
//                 maximum: 1080,
//                 ..abs_x_info
//             };
//             check(raw::libevdev_enable_event_code(
//                 device,
//                 EV_ABS,
//                 ABS_X,
//                 &mut abs_x_info as *mut _ as *mut c_void,
//             ));
//             check(raw::libevdev_enable_event_code(
//                 device,
//                 EV_ABS,
//                 ABS_Y,
//                 &mut abs_y_info as *mut _ as *mut c_void,
//             ));
//         }

//         Device { raw: device }
//     }
// }

// impl Drop for Device {
//     fn drop(&mut self) {
//         unsafe {
//             raw::libevdev_free(self.raw);
//         }
//     }
// }
