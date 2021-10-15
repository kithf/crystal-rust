mod string;

use string::*;
use std::os::raw::c_char;

use json5;
use serde_derive::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Conf {
  name: String,
  age: i32
}

#[repr(C)]
pub struct ConfC {
  name: *const c_char,
  age: i32
}

#[no_mangle]
pub extern "C" fn test(_msg: *const c_char) -> ConfC {
  let js_on = json5::from_str::<Conf>(&r_str(_msg)).expect("json5 fail");
  return ConfC {
    name: c_str(&js_on.name),
    age: js_on.age
  }
}
