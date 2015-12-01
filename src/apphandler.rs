extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate lazy_static;
use winapi::windef::{HWND};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WNDPROC,MB_ICONEXCLAMATION,MB_OK};

use clipboard;
use appsettings::*;
use translator::Translator;

static mut _started: bool = false;
static mut _appsettings: Option<AppSettings> = None;
lazy_static! {
	static ref _settings: AppSettings = {
		unsafe { _appsettings.as_ref().unwrap().clone() }
	};
    static ref _translator: Translator = {
		 Translator::new(_settings.sts_url, _settings.client_id, _settings.client_secret, _settings.translator_url)
    };
}

pub fn init(settings: AppSettings) -> WNDPROC {
	unsafe {
		if !_started { 
			_appsettings = Some(settings);
			_started = true;
		}
	}
	Some(window_proc)
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
	if msg == winapi::winuser::WM_HOTKEY {
		let text = clipboard::get_selection();
		if text != "" {
			let translated = _translator.translate(text, _settings.source_lang, _settings.target_lang);
			user32::MessageBoxA(
				0 as HWND, 
				translated.as_ptr() as *mut _, 
				"Title".as_ptr() as *mut _, 
				MB_ICONEXCLAMATION | MB_OK);
		}
	}
	match msg {
		winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
		winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 }, 
		_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
	}
}