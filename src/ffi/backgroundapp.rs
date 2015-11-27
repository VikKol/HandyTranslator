extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate libc;
 
use winapi::winnt::LPCWSTR;
use winapi::windef::{HWND,HMENU,HBRUSH};
use winapi::minwindef::{HINSTANCE,UINT,DWORD,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{CW_USEDEFAULT,WS_OVERLAPPEDWINDOW,WS_VISIBLE,WNDCLASSW};
use std::os::windows::ffi::OsStrExt;
use std::ptr::{null,null_mut};

use ffi::helpers;

fn default_handler(msg: &UINT) {}
static mut _handler: &'static Fn(&UINT) = &default_handler;
static mut _started: bool = false;

pub fn start<F>(name: &'static str, handler: &'static F) -> bool where F: Fn(&UINT) {
	if unsafe { _started } { return false; }
	
    hide_console_window();
	unsafe { _handler = handler };
	create_window(name, true, 100, 100);

    unsafe {
        let mut msg = new_default_msg();        
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {               
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }
	true
}

fn hide_console_window() {
	let window = unsafe { kernel32::GetConsoleWindow() };
	if window != null_mut() {
		unsafe { user32::ShowWindow (window, winapi::SW_HIDE) };
	}
}

unsafe fn new_default_msg() -> winapi::winuser::MSG {
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
	_handler(&msg);
					
	match msg {
		winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
		winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 }, 
		_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
	}
}

fn create_window(title: &'static str, visible: bool, width: i32, height: i32) {
	// Register class
	let class_name = "window_".to_string() + title;
	let w_class_name = helpers::to_wstring(&class_name);
	let wnd = WNDCLASSW {
		style: 0,
		lpfnWndProc: Some(window_proc), 
		cbClsExtra: 0,
		cbWndExtra: 0,
		hInstance: 0 as HINSTANCE,
		hIcon: unsafe { user32::LoadIconW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
		hCursor: unsafe { user32::LoadCursorW(0 as HINSTANCE, winapi::winuser::IDI_APPLICATION) },
		hbrBackground: 16 as HBRUSH,
		lpszMenuName: 0 as LPCWSTR,
		lpszClassName: w_class_name,
	};
	unsafe { user32::RegisterClassW(&wnd) };

	// Create window			
	let h_wnd_desktop = unsafe { user32::GetDesktopWindow() };
	let windowFlags = if visible { WS_OVERLAPPEDWINDOW | WS_VISIBLE } else { WS_OVERLAPPEDWINDOW };
	unsafe { user32::CreateWindowExA(
		0, 
		class_name.as_ptr() as *mut _,
		title.as_ptr() as *mut _, 
		windowFlags, 
		CW_USEDEFAULT, 
		CW_USEDEFAULT, 
		width, height, 
		h_wnd_desktop, 
		0 as HMENU, 
		0 as HINSTANCE, 
		null_mut())
	};
}