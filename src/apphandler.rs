extern crate winapi;
extern crate user32;
extern crate lazy_static;
extern crate kiss_ui;
extern crate clipboard_win;
use winapi::windef::{HWND,RECT,POINT,HMENU};
use winapi::minwindef::{UINT,WPARAM,LPARAM,LRESULT};
use winapi::winuser::{WNDPROC};
use kiss_ui::prelude::*;
use kiss_ui::text::*;
use kiss_ui::button::Button;
use kiss_ui::container::{Vertical};
use self::clipboard_win::get_clipboard_string;
use std::sync::atomic::{AtomicBool,Ordering,AtomicPtr};

use ffi::*;
use helpers;
use appsettings::*;
use translator::Translator;

const EXIT_CMD_PARAM: WPARAM = 123;
const EXIT_MENU_ITEM_ID: u64 = 2000;

static mut INITIALIZED: bool = false;
static mut MENU_PTR: Option<AtomicPtr<HMENU>> = None;
static mut MENU_LOCK: Option<AtomicBool> = None;
static mut DIALOG_LOCK: Option<AtomicBool> = None;
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
        assert_eq!(INITIALIZED, false);
        APPSETTINGS = Some(settings);
        DIALOG_LOCK = Some(AtomicBool::new(false));
        MENU_LOCK = Some(AtomicBool::new(false));
        MENU_PTR = Some(AtomicPtr::new(0 as *mut HMENU));
        INITIALIZED = true;
    }
    Some(window_proc)
}

unsafe extern "system" fn window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    let dialog_locked: bool = DIALOG_LOCK.as_ref().unwrap().load(Ordering::Relaxed);
    match msg {
        winapi::winuser::WM_HOTKEY if !dialog_locked => {
            handle_translation();
        },
        winapi::winuser::WM_APP if l_param == (winapi::winuser::WM_LBUTTONUP as i64) => {
            let menu_locked = MENU_LOCK.as_ref().unwrap().load(Ordering::Relaxed);
            if menu_locked {
                let h_pop = MENU_PTR.as_ref().unwrap().load(Ordering::Relaxed);
                user32::DestroyMenu(*h_pop);
            }
        },
        winapi::winuser::WM_APP if l_param == (winapi::winuser::WM_RBUTTONUP as i64) => {
            handle_popup_menu(h_wnd);
        },
        winapi::winuser::WM_COMMAND if w_param == EXIT_CMD_PARAM => {
            user32::DestroyWindow(h_wnd);
        },
        winapi::winuser::WM_CLOSE => { user32::DestroyWindow(h_wnd); },
        winapi::winuser::WM_DESTROY => { user32::PostQuitMessage(0); },
        _ => return user32::DefWindowProcW(h_wnd, msg, w_param, l_param)
    };
    0
}

fn handle_translation() {
    unsafe { DIALOG_LOCK.as_ref().unwrap().store(true, Ordering::Relaxed); };
    helpers::simulate_ctrl_c();
    let text = get_clipboard_string().unwrap();
    if text != "" {
        let translated = TRANSLATOR
            .translate(text, SETTINGS.source_lang, SETTINGS.target_lang)
            .unwrap_or_else(|err| {err});
        kiss_ui::show_gui(|| {
            Dialog::new(
                Vertical::new(
                    children![
                        TextBox::new()
                            .set_multiline(true)
                            .set_visible_columns(49)
                            .set_visible_lines(8)
                            .set_name("to_translate"),
                        Button::new()
                            .set_label("Translate")
                            .set_name("translate_btn")
                            .set_onclick(translate_clicked),
                        TextBox::new()
                            .set_text(&translated)
                            .set_multiline(true)
                            .set_visible_columns(49)
                            .set_visible_lines(9)
                            .set_name("translated"),
                    ]
                )
                .set_elem_spacing_pixels(10)
            )
            .set_title("HandyTranslator")
            .set_size_pixels(580, 390)
        });
    }
    unsafe { DIALOG_LOCK.as_ref().unwrap().store(false, Ordering::Relaxed); };
}

fn translate_clicked(btn: Button) {
    let dialog = btn.get_dialog().unwrap();
    let to_translate_tb = dialog
        .get_child("to_translate").unwrap()
        .try_downcast::<TextBox>().ok().expect("'to_translate' is not a TextBox.");
    let translated_tb = dialog
        .get_child("translated").unwrap()
        .try_downcast::<TextBox>().ok().expect("'translated' is not a TextBox.");
    let text = to_translate_tb.get_text();
    let translated = TRANSLATOR
        .translate((*text).to_string(), SETTINGS.source_lang, SETTINGS.target_lang)
        .unwrap_or_else(|err| {err});

    translated_tb.set_text(&translated);
}

unsafe fn handle_popup_menu(h_wnd: HWND) {
    let mut h_pop = user32::CreatePopupMenu();
    MENU_PTR.as_ref().unwrap().store(&mut h_pop, Ordering::Relaxed);
    MENU_LOCK.as_ref().unwrap().store(true, Ordering::Relaxed);
    user32::InsertMenuW(
        h_pop,
        0,
        MF_BYPOSITION|MF_STRING, EXIT_MENU_ITEM_ID,
        helpers::to_wstring(&"Exit"));
    user32::SendMessageW(h_wnd, winapi::winuser::WM_INITMENUPOPUP, h_pop as WPARAM, 0);

    let mut p: POINT = POINT { x: 0, y: 0 };
    user32::GetCursorPos(&mut p);

    let cmd_performed = TrackPopupMenu(
        h_pop,
        TPM_LEFTALIGN|TPM_RIGHTBUTTON|TPM_RETURNCMD|TPM_NONOTIFY,
        p.x,
        p.y,
        0,
        h_wnd,
        0 as *const RECT);
    if cmd_performed > 0 {
        MENU_LOCK.as_ref().unwrap().store(false, Ordering::Relaxed);
        user32::SendMessageW(h_wnd, winapi::winuser::WM_COMMAND, EXIT_CMD_PARAM, 0);
    }
    user32::DestroyMenu(h_pop);
}
