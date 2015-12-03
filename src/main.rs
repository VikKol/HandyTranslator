#![no_main]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate kiss_ui;

extern crate winapi;
extern crate user32;
use winapi::{c_int,HWND,HINSTANCE,LPSTR};

mod appsettings;
mod window;
mod helpers;
mod stsclient;
mod translator;
mod apphandler;
use appsettings::*;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn WinMain(_: HINSTANCE, _: HINSTANCE, _: LPSTR, _: c_int) -> c_int {
    let settings = AppSettings {
        sts_url: "https://datamarket.accesscontrol.windows.net/v2/OAuth2-13",
        scope: "http://api.microsofttranslator.com",
        client_id: "client_id",
        client_secret: "client_secret",
        translator_url: "http://api.microsofttranslator.com/v2/Http.svc/Translate",
        source_lang: "en",
        target_lang: "uk"
    };
    let hanlder = apphandler::init(settings);
    let wnd_handle: HWND = window::create_background("HandyTranslator", hanlder);

    helpers::register_apphotkey(wnd_handle);

    let mut msg = window::create_window_msg();
    unsafe {
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }

    helpers::unregister_apphotkey(wnd_handle);
    0
}
