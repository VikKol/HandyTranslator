extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate libc;
extern crate keystroke;
use winapi::windef::{HWND};
use winapi::winuser::{INPUT,KEYBDINPUT,VK_CONTROL,KEYEVENTF_KEYUP};

use keystroke:*;

use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use std::thread;
use std::time::Duration;
use std::mem;

const HOT_KEY_ID: i32 = 1;
const VK_C: u16 = 0x43;
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
	press_key(Key::Physical(Physical::C));
	release_key(Key::Physical(Physical::C));
	release_key(Key::Physical(Physical::Control));
/*	
	unsafe {
		let mut input = INPUT { 
			type_: winapi::INPUT_KEYBOARD, 
			u: Default::default()
		}; 
		*input.ki_mut() = KEYBDINPUT { 
			wVk: 0,
			wScan: 0,
			dwFlags: 0,
			time: 0,
			dwExtraInfo: 0
		};
		
		// Press the "Ctrl" key
		input.ki_mut().wVk = VK_CONTROL as u16;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
		
		thread::sleep(Duration::from_millis(200));
		
 		// Press the "C" key
		input.ki_mut().wVk = VK_C;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);

		thread::sleep(Duration::from_millis(200));

        // Release the "C" key
		input.ki_mut().wVk = VK_C;
		input.ki_mut().dwFlags = KEYEVENTF_KEYUP;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
 
		thread::sleep(Duration::from_millis(200));
 
        // Release the "Ctrl" key
		input.ki_mut().wVk = VK_CONTROL as u16;
		input.ki_mut().dwFlags = KEYEVENTF_KEYUP;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
		
		thread::sleep(Duration::from_millis(200));
	};
*/
}