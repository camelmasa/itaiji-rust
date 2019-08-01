#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate yaml_rust;

use std::ffi::{CStr, CString};

pub mod itaiji;
pub use self::itaiji::Converter;

lazy_static! {
    static ref CONVERTER: Converter = Converter::new();
}

#[no_mangle]
pub extern "C" fn seijitai(c_text: *const libc::c_char) -> *const libc::c_char {
    let text = unsafe { CStr::from_ptr(c_text) }.to_str().unwrap();
    let seijitai_text = CONVERTER.seijitai(text);

    CString::new(seijitai_text).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn itaiji(c_text: *const libc::c_char) -> *const libc::c_char {
    let text = unsafe { CStr::from_ptr(c_text) }.to_str().unwrap();
    let itaiji_text = CONVERTER.itaiji(text);

    CString::new(itaiji_text).unwrap().into_raw()
}
