extern crate winapi;
extern crate user32;
extern crate keystroke;
use winapi::windef::{HWND,HICON};
use winapi::minwindef::{HINSTANCE};
use self::keystroke::*;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;
use std::mem::{size_of};
use ffi::*;

const TRAYICON_ID: u32 = 1;
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
    thread::sleep(Duration::from_millis(200)); // Why do I need this?
    press_key(Key::Physical(Physical::C));
    thread::sleep(Duration::from_millis(200));
    release_key(Key::Physical(Physical::C));
    thread::sleep(Duration::from_millis(200));
    release_key(Key::Physical(Physical::Control));
    thread::sleep(Duration::from_millis(200));
}

pub fn add_tray_icon(hwnd: HWND, tip: &'static str) {
    let tip_v: Vec<char> = tip.chars()
        .chain(Some(0 as char).into_iter())
        .collect();
    let mut nid = NOTIFYICONDATA {
        cbSize: size_of::<NOTIFYICONDATA>() as i32,
        hWnd: hwnd,
        uID: TRAYICON_ID,
        uFlags: NIF_ICON | NIF_MESSAGE | NIF_TIP,
        uCallbackMessage: winapi::winuser::WM_APP,
        hIcon: unsafe { user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
        szTip: tip_v.as_ptr() as * const _,
        dwState: 0,
        dwStateMask: 0,
        szInfo: 0 as *const _,
        uVersion: 0,
        szInfoTitle: 0 as *const _,
        dwInfoFlags: 0
    };
    unsafe { Shell_NotifyIcon(NIM_ADD, &mut nid); }
}

pub fn remove_tray_icon(hwnd: HWND) {
    let mut nid = NOTIFYICONDATA {
        cbSize: size_of::<NOTIFYICONDATA>() as i32,
        hWnd: hwnd,
        uID: TRAYICON_ID,
        uFlags: 0,
        uCallbackMessage: winapi::winuser::WM_APP,
        hIcon: 0 as HICON,
        szTip: 0 as *const _,
        dwState: 0,
        dwStateMask: 0,
        szInfo: 0 as *const _,
        uVersion: 0,
        szInfoTitle: 0 as *const _,
        dwInfoFlags: 0
    };
    unsafe { Shell_NotifyIcon(NIM_DELETE, &mut nid); }
}
