extern crate winapi;
extern crate user32;
use winapi::windef::{HWND};

mod window;
mod helpers;
mod clipboard;
mod translator;
mod apphandler;

static FROM: &'static str = "en";
static TO: &'static str = "uk";

fn main() {
	window::hide_console_window();

	let hanlder = apphandler::init(FROM, TO, "http://api.microsofttranslator.com/v2/Http.svc/Translate");
	let wnd_handle: HWND = window::create_window("HandyTranslator", false, 580, 400, hanlder);

	helpers::register_apphotkey(wnd_handle);

	let mut msg = window::create_window_msg();
    unsafe {
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {               
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }	

	helpers::unregister_apphotkey(wnd_handle);
}