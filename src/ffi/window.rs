#![allow(unused_imports)]
#![allow(non_snake_case)]
 
extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate libc;

use winapi::winnt::LPCWSTR;
use winapi::windef::{HWND,HMENU,HBRUSH};
use winapi::minwindef::{HINSTANCE,UINT,DWORD,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WS_OVERLAPPEDWINDOW,WS_VISIBLE,WNDCLASSW};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null,null_mut};

use ffi::helpers::*;

pub struct Window {
	width: i32,
	height: i32,
	visible: bool,
	title: &'static str,
	
	//_handler: 'static Fn
}

impl Window {
	pub fn new(width: i32, height: i32, visible: bool, title: &'static str) -> Window {
		let wnd = Window { width: width, height: height, visible: visible, title: title };
		Window::create_window(title, visible, width, height);
		wnd
	}
	
	pub fn new_bckgrnd_window(name: &'static str) {
		Window::create_window(name, false, 0, 0);
	}
	
	pub fn hide_console_window() {
		let window = unsafe { kernel32::GetConsoleWindow() };
		if window != null_mut() {
			unsafe { user32::ShowWindow (window, winapi::SW_HIDE) };
		}
	}
	
	pub unsafe fn new_default_msg() -> winapi::winuser::MSG {
		winapi::winuser::MSG {
			hwnd : 0 as HWND,
			message : 0 as UINT,
			wParam : 0 as WPARAM,
			lParam : 0 as LPARAM,
			time : 0 as DWORD,
			pt : winapi::windef::POINT { x: 0, y: 0, }
		}
	}
	
	unsafe extern "system" fn window_proc(h_wnd :HWND, msg :UINT, w_param :WPARAM, l_param :LPARAM) -> LRESULT {		
		match msg {
			winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
			winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 } 
			_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param),
		}
	}
	
	fn create_window(title: &'static str, visible: bool, width: i32, height: i32) {
		unsafe {
			// Register class
			let class_name = "window_".to_string() + title;
			let w_class_name = to_wstring(&class_name);
			let wnd = WNDCLASSW {
				style: 0,
				lpfnWndProc: Some(Window::window_proc), 
				cbClsExtra: 0,
				cbWndExtra: 0,
				hInstance: 0 as HINSTANCE,
				hIcon: user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION),
				hCursor: user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION),
				hbrBackground: 16 as HBRUSH,
				lpszMenuName: 0 as LPCWSTR,
				lpszClassName: w_class_name,
			};
			user32::RegisterClassW(&wnd);

			// Create window			
			let h_wnd_desktop = user32::GetDesktopWindow();
			let windowFlags = if visible { WS_OVERLAPPEDWINDOW | WS_VISIBLE } else { WS_OVERLAPPEDWINDOW };
			user32::CreateWindowExA(
				0, 
				class_name.as_ptr() as *mut _,
				title.as_ptr() as *mut _, 
				windowFlags, 
				0, 0, 
				width, height, 
				h_wnd_desktop, 
				0 as HMENU, 
				0 as HINSTANCE, 
				null_mut());
		}
	}
}