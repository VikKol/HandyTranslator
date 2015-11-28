extern crate winapi;
extern crate user32;
extern crate kernel32;

use winapi::windef::{HWND};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{MB_ICONEXCLAMATION,MB_OK};

mod window;
mod helpers;

fn main() {
	window::hide_console_window();
	let handle: HWND = window::create_window(
		"HandyTranslator", 
		true, 580, 400, 
		Some(window_proc));

	helpers::register_apphotkey(handle);

	let mut msg = window::create_window_msg();
    unsafe {
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {               
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }	
	
	helpers::unregister_apphotkey(handle);
}
	
unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
	if msg == winapi::winuser::WM_HOTKEY {
		user32::MessageBoxA(
			0 as HWND, 
			"Hotkey".as_ptr() as *mut _, 
			"Title".as_ptr() as *mut _, 
			MB_ICONEXCLAMATION | MB_OK);
	}
	match msg {
		winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
		winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 }, 
		_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
	}
}