extern crate evdev_rs;

use evdev_rs::enums::{EventCode, EventType, EV_ABS, EV_KEY, EV_SYN};
use evdev_rs::{AbsInfo, Device, InputEvent, TimeVal, UInputDevice};

pub struct Mouse {
    device: UInputDevice,
}

impl Mouse {
    pub fn new(x_maximum: i32, y_maximum: i32) -> Self {
        let evdevice = Device::new().unwrap();
        evdevice.set_name("barrier-rust");
        evdevice
            .enable(&EventCode::EV_KEY(EV_KEY::BTN_LEFT))
            .unwrap();
        evdevice.enable(&EventType::EV_ABS).unwrap();
        evdevice
            .enable_event_code(
                &EventCode::EV_ABS(EV_ABS::ABS_X),
                Some(&AbsInfo {
                    value: 0,
                    minimum: 0,
                    maximum: x_maximum,
                    fuzz: 0,
                    flat: 0,
                    resolution: 0,
                }),
            )
            .unwrap();
        evdevice
            .enable_event_code(
                &EventCode::EV_ABS(EV_ABS::ABS_Y),
                Some(&AbsInfo {
                    value: 0,
                    minimum: 0,
                    maximum: y_maximum,
                    fuzz: 0,
                    flat: 0,
                    resolution: 0,
                }),
            )
            .unwrap();

        let device = UInputDevice::create_from_device(&evdevice).unwrap();
        Self { device }
    }

    pub fn move_abs(&mut self, x: i32, y: i32) {
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_X),
                x,
            ))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_Y),
                y,
            ))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
    }

    pub fn button_down(&mut self, button: impl Into<MouseButton>) {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button.into()),
                1,
            ))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
    }

    pub fn button_up(&mut self, button: impl Into<MouseButton>) {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button.into()),
                0,
            ))
            .unwrap();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .unwrap();
    }
}

impl Into<EV_KEY> for MouseButton {
    fn into(self) -> EV_KEY {
        EV_KEY::BTN_LEFT
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
    device: UInputDevice,
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

// impl Into<keyboard::Key> for Key {
//     fn into(self) -> keyboard::Key {
//         keyboard::Key::A
//     }
// }

impl Keyboard {
    pub fn new() -> Self {
        let evdevice = Device::new().unwrap();
        evdevice.set_name("barrier-rust");
        evdevice
            .enable(&EventCode::EV_KEY(EV_KEY::BTN_LEFT))
            .unwrap();
        let device = UInputDevice::create_from_device(&evdevice).unwrap();
        Self { device }
    }

    pub fn key_down(&mut self, button: impl Into<Key>) {
        // let key = button.into();
        // let key: keyboard::Key = key.into();
        // self.device.press(&key).unwrap();
        // self.device.synchronize().unwrap();
    }

    pub fn key_up(&mut self, button: impl Into<Key>) {
        // let key = button.into();
        // let key: keyboard::Key = key.into();
        // self.device.release(&key).unwrap();
        // self.device.synchronize().unwrap();
    }
}
