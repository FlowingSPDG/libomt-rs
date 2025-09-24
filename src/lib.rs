mod ffi {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    #![allow(improper_ctypes)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use ffi::*;

use std::ffi::CString;
use std::ptr::NonNull;

pub struct Receiver(NonNull<ffi::omt_receive_t>);
impl Receiver {
    pub fn create(address: &str, frame_types: ffi::OMTFrameType, preferred: ffi::OMTPreferredVideoFormat, flags: ffi::OMTReceiveFlags) -> Option<Self> {
        let caddr = CString::new(address).ok()?;
        let ptr = unsafe { ffi::omt_receive_create(caddr.as_ptr(), frame_types, preferred, flags) };
        NonNull::new(ptr).map(Receiver)
    }
}
impl Drop for Receiver {
    fn drop(&mut self) {
        unsafe { ffi::omt_receive_destroy(self.0.as_ptr()) };
    }
}

pub struct Sender(NonNull<ffi::omt_send_t>);
impl Sender {
    pub fn create(name: &str, quality: ffi::OMTQuality) -> Option<Self> {
        let cname = CString::new(name).ok()?;
        let ptr = unsafe { ffi::omt_send_create(cname.as_ptr(), quality) };
        NonNull::new(ptr).map(Sender)
    }
}
impl Drop for Sender {
    fn drop(&mut self) {
        unsafe { ffi::omt_send_destroy(self.0.as_ptr()) };
    }
}

pub fn discovery_getaddresses() -> Vec<String> {
    let mut count = 0i32;
    let arr = unsafe { ffi::omt_discovery_getaddresses(&mut count as *mut i32) };
    if arr.is_null() || count <= 0 { return Vec::new(); }
    let slice = unsafe { std::slice::from_raw_parts(arr, count as usize) };
    slice.iter().filter_map(|&p| {
        if p.is_null() { return None; }
        unsafe { std::ffi::CStr::from_ptr(p) }.to_str().ok().map(|s| s.to_string())
    }).collect()
}
