use std::str;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

pub fn r_str(s: *const c_char) -> String {
  let c_str = unsafe { CStr::from_ptr(s) };
  return c_str.to_string_lossy().into_owned()
}

pub fn c_str(c: &str) -> *const c_char {
  let cstr = CString::new(c).unwrap();
  return cstr.into_raw()
}