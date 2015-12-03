extern crate winapi;
extern crate user32;
extern crate keystroke;
use winapi::windef::{HWND};
use self::keystroke::*;

use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;

const HOT_KEY_ID: i32 = 1;
const VK_Q: u32 = 81;
const MOD_ALT: u32 = 0x0001;

pub fn register_apphotkey(h: HWND){
    unsafe { user32::RegisterHotKey(h, HOT_KEY_ID, MOD_ALT, VK_Q); };
}

pub fn unregister_apphotkey(h: HWND){
    unsafe { user32::UnregisterHotKey(h, HOT_KEY_ID); };
}

pub fn to_wstring(text: &str) -> *const u16 {
    let v: Vec<u16> = OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    v.as_ptr()
}

pub fn simulate_ctrl_c() {
    press_key(Key::Physical(Physical::Control));
    thread::sleep(Duration::from_millis(150)); // Why do I need this?
    press_key(Key::Physical(Physical::C));
    thread::sleep(Duration::from_millis(150));
    release_key(Key::Physical(Physical::C));
    thread::sleep(Duration::from_millis(150));
    release_key(Key::Physical(Physical::Control));
    thread::sleep(Duration::from_millis(150));
}
