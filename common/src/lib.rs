#![no_std]

pub mod module;
use utils::*;

extern crate alloc;


extern "C" {
    pub fn _print(p: *const u8, size: usize);
    pub fn _run(p: *const u8, size: usize);
    pub fn _notify(p: *const u8, size: usize);
    pub fn _request(p: *const u8, size: usize);
}

pub fn notify(s:&str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _notify(pair.0, pair.1);
    }
}

pub fn println(s: &str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _print(pair.0, pair.1);
    }
}

pub fn request(s: &str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _request(pair.0, pair.1);
    }
}

pub fn exec_sql(s: &str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _run(pair.0, pair.1);
    }
}