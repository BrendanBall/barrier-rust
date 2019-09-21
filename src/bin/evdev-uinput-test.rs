use evdev_sys::*;
use libc::c_void;
use std::thread;
use std::time::Duration;

const EV_SYN: u32 = 0x00;
const SYN_REPORT: u32 = 0;
const EV_KEY: u32 = 0x01;
const BTN_LEFT: u32 = 0x110;
const EV_ABS: u32 = 0x03;
const ABS_X: u32 = 0x00;
const ABS_Y: u32 = 0x01;

fn main() {
    unsafe {
        let device = libevdev_new();
        check(libevdev_enable_event_type(device, EV_KEY));
        check(libevdev_enable_event_code(
            device,
            EV_KEY,
            BTN_LEFT,
            std::ptr::null(),
        ));

        check(libevdev_enable_event_type(device, EV_ABS));
        let mut abs_x_info = input_absinfo {
            value: 0,
            minimum: 0,
            maximum: 1920,
            fuzz: 0,
            flat: 0,
            resolution: 0,
        };
        let mut abs_y_info = input_absinfo {
            maximum: 1080,
            ..abs_x_info
        };
        check(libevdev_enable_event_code(
            device,
            EV_ABS,
            ABS_X,
            &mut abs_x_info as *mut _ as *mut c_void,
        ));
        check(libevdev_enable_event_code(
            device,
            EV_ABS,
            ABS_Y,
            &mut abs_y_info as *mut _ as *mut c_void,
        ));
        let mut uinput_device: *mut *mut libevdev_uinput = &mut (0 as *mut _);
        // libevdev_set_name(device, "evdev-test".into());
        check(libevdev_uinput_create_from_device(
            device,
            LIBEVDEV_UINPUT_OPEN_MANAGED,
            uinput_device,
        ));

        thread::sleep(Duration::from_secs(1));
        // for i in 0..50 {
        //     thread::sleep(Duration::from_millis(50));

        //     libevdev_uinput_write_event(uinput_device, EV_ABS, ABS_X, 1000 + (i * 10));
        //     libevdev_uinput_write_event(uinput_device, EV_SYN, SYN_REPORT, 0);
        // }
        // libevdev_uinput_destroy(uinput_device);
        libevdev_free(device);
    }
}

fn check(code: i32) {
    println!("error code: {}", code);
    if code != 0 {
        panic!()
    }
}
