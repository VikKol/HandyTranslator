extern crate winapi;
extern crate user32;

use winapi::winnt::LPCWSTR;
use winapi::windef::{HWND,HMENU,HBRUSH};
use winapi::minwindef::{HINSTANCE,UINT,DWORD,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{MB_ICONEXCLAMATION,MB_OK,CW_USEDEFAULT,WS_OVERLAPPEDWINDOW,WS_VISIBLE,WNDCLASSW};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null,null_mut};

mod ffi;
use ffi::backgroundapp;

fn main() {
	let handler: &'static Fn(&UINT) = &handle_msgs;
	backgroundapp::start("HandyTranslator", handler);
}

fn handle_msgs(msg: &UINT) {
    match msg {
		&winapi::winuser::WM_CLOSE => 0,  
		&winapi::winuser::WM_DESTROY => 0, 
		_ => unsafe { user32::MessageBoxA(
			0 as HWND, 
			"Hello from handler".as_ptr() as *mut _, 
			"Title".as_ptr() as *mut _, 
			MB_ICONEXCLAMATION | MB_OK)
		}
	};
}