extern crate winapi;
extern crate user32;

use winapi::windef::{HWND};

mod ffi;
use ffi::window::*;

fn main()
{
    Window::hide_console_window();
    
    Window::new(400, 200, true, "HandyTranslator");

    unsafe 
    {
        let mut msg = Window::new_default_msg();        
        while user32::GetMessageW(&mut msg, 0 as HWND, 0, 0) > 0 {               
            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);
        }
    }
}