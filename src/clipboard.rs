extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate libc;

use winapi::windef::{HWND};
use winapi::winnt::{HANDLE};
use winapi::minwindef::{UINT};
use winapi::winuser::{INPUT,KEYBDINPUT,VK_CONTROL,KEYEVENTF_KEYUP,MB_ICONEXCLAMATION,MB_OK};
//use libc;

use std::mem;
use std::thread;
use std::time::Duration;
use std::ffi::CString;

const VK_C: u16 = 0x43;
const CF_TEXT: UINT = 1;

pub fn get_selection() -> String {
	simulate_ctrl_c();
	get_data_from_clipboard()
}

fn get_data_from_clipboard() -> String {
	let text: String;
	unsafe { 
		user32::OpenClipboard(0 as HWND);
		let clip: HANDLE = user32::GetClipboardData(CF_TEXT);
		
		//kernel32::GlobalLock(clip); 
		
		let c_str = CString::from_raw(clip as *mut libc::c_char);
		text = c_str.to_str().unwrap().to_string();
		
		//kernel32::GlobalUnlock(clip);
	
		user32::MessageBoxA(
			0 as HWND, 
			text.as_ptr() as *mut _, 
			"Title".as_ptr() as *mut _, 
			MB_ICONEXCLAMATION | MB_OK);
	};		
	text
}

fn simulate_ctrl_c() {
	let mut input = INPUT { 
		type_: winapi::INPUT_KEYBOARD, 
		u: Default::default()
	}; 
	unsafe {
		*input.ki_mut() = KEYBDINPUT { 
			wVk: 0,
			wScan: 0,
			dwFlags: 0,
			time: 0,
			dwExtraInfo: 0
		};
		
		// Delay
		//thread::sleep(Duration::new(1, 0));

		// Press the "Ctrl" key
		input.ki_mut().wVk = VK_CONTROL as u16;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);

 		// Press the "C" key
		input.ki_mut().wVk = VK_C;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);

		//thread::sleep(Duration::from_millis(500));

        // Release the "C" key
		input.ki_mut().wVk = VK_C;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
 
        // Release the "Ctrl" key
		input.ki_mut().wVk = VK_CONTROL as u16;
		input.ki_mut().dwFlags = KEYEVENTF_KEYUP;
		user32::SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
		
		//thread::sleep(Duration::from_millis(500));
	};
}