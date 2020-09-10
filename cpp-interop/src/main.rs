#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/dog.rs"));

use std::ffi::CString;

fn main() {
    unsafe {
        let name = CString::new("ｲｯﾇ").expect("CString::new failed");
        let mut dog = Dog::new(name.as_ptr());
        dog.walk();
    }
}
