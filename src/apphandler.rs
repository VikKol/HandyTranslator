extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate lazy_static;
extern crate kiss_ui;
use winapi::windef::{HWND};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WNDPROC,MB_ICONEXCLAMATION,MB_OK};

use kiss_ui::container::Horizontal;
use kiss_ui::dialog::Dialog;
use kiss_ui::text::Label;

use clipboard;
use appsettings::*;
use translator::Translator;

static mut STARTED: bool = false;
static mut APPSETTINGS: Option<AppSettings> = None;
lazy_static! {
	static ref SETTINGS: AppSettings = {
		unsafe { APPSETTINGS.as_ref().unwrap().clone() }
	};
    static ref TRANSLATOR: Translator = {
		 Translator::new(SETTINGS.sts_url, SETTINGS.client_id, SETTINGS.client_secret, SETTINGS.scope, SETTINGS.translator_url)
    };
}

pub fn init(settings: AppSettings) -> WNDPROC {
	unsafe {
		if !STARTED { 
			APPSETTINGS = Some(settings);
			STARTED = true;
		}
	}
	Some(window_proc)
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
	if msg == winapi::winuser::WM_HOTKEY {
		let text = "text".to_owned();//clipboard::get_selection();
		if text != "" {
			let translated = TRANSLATOR.translate(text, SETTINGS.source_lang, SETTINGS.target_lang);
			kiss_ui::show_gui(|| {
				Dialog::new(
					Horizontal::new(
						children![
							Label::new(translated),
						]
					)
				)
				.set_title("HandyTranslator")
				.set_size_pixels(640, 480)
			});
			/*
			user32::MessageBoxW(
				0 as HWND, 
				translated.as_ptr() as *mut _, 
				"Title".as_ptr() as *mut _, 
				MB_ICONEXCLAMATION | MB_OK);
			*/
		}
	}
	match msg {
		winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); 0 },  
		winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); 0 }, 
		_ => user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
	}
}