use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

pub fn to_wstring(text: &str) -> *const u16 {
    let v: Vec<u16> = OsStr::new(text)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    v.as_ptr()
}