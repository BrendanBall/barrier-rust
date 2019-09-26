extern crate evdev_rs;

use evdev_rs::enums::{int_to_ev_key, EventCode, EventType, EV_ABS, EV_KEY, EV_SYN};
use evdev_rs::{AbsInfo, Device, InputEvent, TimeVal, UInputDevice};
use nix::errno::Errno;
use snafu::{OptionExt, ResultExt, Snafu};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not create {} device", device_type))]
    CreateDevice { device_type: DeviceType },
    #[snafu(display("Could not create {} uinput device: {}", device_type, source))]
    CreateUInputDevice {
        device_type: DeviceType,
        source: Errno,
    },
    #[snafu(display("Could not enable property for {}: {}", device_type, source))]
    EnableDeviceProperty {
        device_type: DeviceType,
        source: Errno,
    },
    #[snafu(display("Could not create event for {}: {}", device_type, source))]
    CreateEvent {
        device_type: DeviceType,
        source: Errno,
    },
    #[snafu(display("Could not map key for {}", device_type))]
    MapKey { device_type: DeviceType },
}

#[derive(Debug)]
pub enum DeviceType {
    Mouse,
    Keyboard,
}

impl std::fmt::Display for DeviceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        match *self {
            DeviceType::Mouse => f.write_str("Mouse"),
            DeviceType::Keyboard => f.write_str("Keyboard"),
        }
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Mouse {
    device: UInputDevice,
}

impl Mouse {
    pub fn new(x_maximum: i32, y_maximum: i32) -> Result<Self> {
        let evdevice = Device::new().context(CreateDevice {
            device_type: DeviceType::Mouse,
        })?;
        evdevice.set_name("barrier-rust");
        evdevice
            .enable(&EventCode::EV_KEY(EV_KEY::BTN_LEFT))
            .context(EnableDeviceProperty {
                device_type: DeviceType::Mouse,
            })?;
        evdevice
            .enable(&EventType::EV_ABS)
            .context(EnableDeviceProperty {
                device_type: DeviceType::Mouse,
            })?;
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
            .context(EnableDeviceProperty {
                device_type: DeviceType::Mouse,
            })?;
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
            .context(EnableDeviceProperty {
                device_type: DeviceType::Mouse,
            })?;

        let device = UInputDevice::create_from_device(&evdevice).context(CreateUInputDevice {
            device_type: DeviceType::Mouse,
        })?;
        Ok(Self { device })
    }

    pub fn move_abs(&mut self, x: i32, y: i32) -> Result<()> {
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_X),
                x,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_ABS(EV_ABS::ABS_Y),
                y,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        Ok(())
    }

    pub fn button_down(&mut self, button: impl Into<MouseButton>) -> Result<()> {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button.into()),
                1,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        Ok(())
    }

    pub fn button_up(&mut self, button: impl Into<MouseButton>) -> Result<()> {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button.into()),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Mouse,
            })?;
        Ok(())
    }
}

impl Into<EV_KEY> for MouseButton {
    fn into(self) -> EV_KEY {
        match self {
            MouseButton::Left => EV_KEY::BTN_LEFT,
            MouseButton::Right => EV_KEY::BTN_RIGHT,
            MouseButton::Middle => EV_KEY::BTN_MIDDLE,
            MouseButton::Side => EV_KEY::BTN_SIDE,
            MouseButton::Extra => EV_KEY::BTN_EXTRA,
            MouseButton::Forward => EV_KEY::BTN_FORWARD,
            MouseButton::Back => EV_KEY::BTN_BACK,
            MouseButton::Task => EV_KEY::BTN_TASK,
        }
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

// looks like formula is button - 8

fn button_to_ev_key(button: u16) -> Result<EV_KEY> {
    let key = int_to_ev_key((button - 8).into()).context(MapKey {
        device_type: DeviceType::Keyboard,
    })?;
    Ok(key)
}

impl Keyboard {
    pub fn new() -> Result<Self> {
        let evdevice = Device::new().context(CreateDevice {
            device_type: DeviceType::Keyboard,
        })?;
        evdevice.set_name("barrier-rust");
        for key in &KEYBOARD_KEYS[..] {
            evdevice
                .enable(&EventCode::EV_KEY(key.clone()))
                .context(EnableDeviceProperty {
                    device_type: DeviceType::Keyboard,
                })?;
        }
        let device = UInputDevice::create_from_device(&evdevice).context(CreateUInputDevice {
            device_type: DeviceType::Keyboard,
        })?;
        Ok(Self { device })
    }

    pub fn key_down(&mut self, button: u16) -> Result<()> {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button_to_ev_key(button)?),
                1,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Keyboard,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Keyboard,
            })?;
        Ok(())
    }

    pub fn key_up(&mut self, button: u16) -> Result<()> {
        let button = button.into();
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_KEY(button_to_ev_key(button)?),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Keyboard,
            })?;
        self.device
            .write_event(&InputEvent::new(
                &TimeVal::new(0, 0),
                &EventCode::EV_SYN(EV_SYN::SYN_REPORT),
                0,
            ))
            .context(CreateEvent {
                device_type: DeviceType::Keyboard,
            })?;
        Ok(())
    }
}

const KEYBOARD_KEYS: [EV_KEY; 146] = [
    EV_KEY::KEY_ESC,
    EV_KEY::KEY_1,
    EV_KEY::KEY_2,
    EV_KEY::KEY_3,
    EV_KEY::KEY_4,
    EV_KEY::KEY_5,
    EV_KEY::KEY_6,
    EV_KEY::KEY_7,
    EV_KEY::KEY_8,
    EV_KEY::KEY_9,
    EV_KEY::KEY_0,
    EV_KEY::KEY_MINUS,
    EV_KEY::KEY_EQUAL,
    EV_KEY::KEY_BACKSPACE,
    EV_KEY::KEY_TAB,
    EV_KEY::KEY_Q,
    EV_KEY::KEY_W,
    EV_KEY::KEY_E,
    EV_KEY::KEY_R,
    EV_KEY::KEY_T,
    EV_KEY::KEY_Y,
    EV_KEY::KEY_U,
    EV_KEY::KEY_I,
    EV_KEY::KEY_O,
    EV_KEY::KEY_P,
    EV_KEY::KEY_LEFTBRACE,
    EV_KEY::KEY_RIGHTBRACE,
    EV_KEY::KEY_ENTER,
    EV_KEY::KEY_LEFTCTRL,
    EV_KEY::KEY_A,
    EV_KEY::KEY_S,
    EV_KEY::KEY_D,
    EV_KEY::KEY_F,
    EV_KEY::KEY_G,
    EV_KEY::KEY_H,
    EV_KEY::KEY_J,
    EV_KEY::KEY_K,
    EV_KEY::KEY_L,
    EV_KEY::KEY_SEMICOLON,
    EV_KEY::KEY_APOSTROPHE,
    EV_KEY::KEY_GRAVE,
    EV_KEY::KEY_LEFTSHIFT,
    EV_KEY::KEY_BACKSLASH,
    EV_KEY::KEY_Z,
    EV_KEY::KEY_X,
    EV_KEY::KEY_C,
    EV_KEY::KEY_V,
    EV_KEY::KEY_B,
    EV_KEY::KEY_N,
    EV_KEY::KEY_M,
    EV_KEY::KEY_COMMA,
    EV_KEY::KEY_DOT,
    EV_KEY::KEY_SLASH,
    EV_KEY::KEY_RIGHTSHIFT,
    EV_KEY::KEY_KPASTERISK,
    EV_KEY::KEY_LEFTALT,
    EV_KEY::KEY_SPACE,
    EV_KEY::KEY_CAPSLOCK,
    EV_KEY::KEY_F1,
    EV_KEY::KEY_F2,
    EV_KEY::KEY_F3,
    EV_KEY::KEY_F4,
    EV_KEY::KEY_F5,
    EV_KEY::KEY_F6,
    EV_KEY::KEY_F7,
    EV_KEY::KEY_F8,
    EV_KEY::KEY_F9,
    EV_KEY::KEY_F10,
    EV_KEY::KEY_NUMLOCK,
    EV_KEY::KEY_SCROLLLOCK,
    EV_KEY::KEY_KP7,
    EV_KEY::KEY_KP8,
    EV_KEY::KEY_KP9,
    EV_KEY::KEY_KPMINUS,
    EV_KEY::KEY_KP4,
    EV_KEY::KEY_KP5,
    EV_KEY::KEY_KP6,
    EV_KEY::KEY_KPPLUS,
    EV_KEY::KEY_KP1,
    EV_KEY::KEY_KP2,
    EV_KEY::KEY_KP3,
    EV_KEY::KEY_KP0,
    EV_KEY::KEY_KPDOT,
    EV_KEY::KEY_ZENKAKUHANKAKU,
    EV_KEY::KEY_102ND,
    EV_KEY::KEY_F11,
    EV_KEY::KEY_F12,
    EV_KEY::KEY_RO,
    EV_KEY::KEY_KATAKANA,
    EV_KEY::KEY_HIRAGANA,
    EV_KEY::KEY_HENKAN,
    EV_KEY::KEY_KATAKANAHIRAGANA,
    EV_KEY::KEY_MUHENKAN,
    EV_KEY::KEY_KPJPCOMMA,
    EV_KEY::KEY_KPENTER,
    EV_KEY::KEY_RIGHTCTRL,
    EV_KEY::KEY_KPSLASH,
    EV_KEY::KEY_SYSRQ,
    EV_KEY::KEY_RIGHTALT,
    EV_KEY::KEY_HOME,
    EV_KEY::KEY_UP,
    EV_KEY::KEY_PAGEUP,
    EV_KEY::KEY_LEFT,
    EV_KEY::KEY_RIGHT,
    EV_KEY::KEY_END,
    EV_KEY::KEY_DOWN,
    EV_KEY::KEY_PAGEDOWN,
    EV_KEY::KEY_INSERT,
    EV_KEY::KEY_DELETE,
    EV_KEY::KEY_MUTE,
    EV_KEY::KEY_VOLUMEDOWN,
    EV_KEY::KEY_VOLUMEUP,
    EV_KEY::KEY_POWER,
    EV_KEY::KEY_KPEQUAL,
    EV_KEY::KEY_PAUSE,
    EV_KEY::KEY_KPCOMMA,
    EV_KEY::KEY_HANGEUL,
    EV_KEY::KEY_HANJA,
    EV_KEY::KEY_YEN,
    EV_KEY::KEY_LEFTMETA,
    EV_KEY::KEY_RIGHTMETA,
    EV_KEY::KEY_COMPOSE,
    EV_KEY::KEY_STOP,
    EV_KEY::KEY_AGAIN,
    EV_KEY::KEY_PROPS,
    EV_KEY::KEY_UNDO,
    EV_KEY::KEY_FRONT,
    EV_KEY::KEY_COPY,
    EV_KEY::KEY_OPEN,
    EV_KEY::KEY_PASTE,
    EV_KEY::KEY_FIND,
    EV_KEY::KEY_CUT,
    EV_KEY::KEY_HELP,
    EV_KEY::KEY_F13,
    EV_KEY::KEY_F14,
    EV_KEY::KEY_F15,
    EV_KEY::KEY_F16,
    EV_KEY::KEY_F17,
    EV_KEY::KEY_F18,
    EV_KEY::KEY_F19,
    EV_KEY::KEY_F20,
    EV_KEY::KEY_F21,
    EV_KEY::KEY_F22,
    EV_KEY::KEY_F23,
    EV_KEY::KEY_F24,
    EV_KEY::KEY_UNKNOWN,
];
