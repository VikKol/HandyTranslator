extern crate winapi;
extern crate user32;
extern crate kernel32;
use winapi::windef::{HWND};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WNDPROC,MB_ICONEXCLAMATION,MB_OK};

use clipboard;
use translator::Translator;

static mut _source_lang: &'static str = "";
static mut _target_lang: &'static str = "";
static mut _translator_url: &'static str = "";
static mut _translator: Option<Translator> = None;
static mut _started: bool = false;

pub fn init(source_lang: &'static str, target_lang: &'static str, translator_url: &'static str) -> WNDPROC {
	unsafe {
		if !_started { 
			_source_lang = source_lang;
			_target_lang = target_lang;
			_translator_url = translator_url;
			_translator = Some(Translator::new(_translator_url));
			_started = true;
		}
		Some(window_proc)
	}
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
	if msg == winapi::winuser::WM_HOTKEY {
		let text = clipboard::get_selection();
		if text != "" {
			if let Some(translator_ref) = _translator.as_ref() {
				let translated = translator_ref.translate(text, _source_lang, _target_lang);
				
				user32::MessageBoxA(
					0 as HWND, 
					translated.as_ptr() as *mut _, 
					"Title".as_ptr() as *mut _, 
					MB_ICONEXCLAMATION | MB_OK);
			}
		}
	}
	match msg {
		winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
		winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 }, 
		_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
	}
}