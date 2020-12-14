#![no_std]
#![feature(fmt_as_str)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(default_alloc_error_handler)]
#![feature(associated_type_defaults)]

extern crate alloc;
use core::{slice};
use alloc::vec::Vec;
use core::str;

/// 根据u8指针和数据长度返回vec<u8>
pub fn u8_pointer_and_size_to_vec(p:*const u8, size: usize) -> Vec<u8>{
    let u8_slice = unsafe {slice::from_raw_parts(p,size)};
    u8_slice.to_vec()
}

/// 根据u8指针和数据长度返回&str
/// err则panic
pub fn u8_pointer_and_size_to_str<'a>(p:*const u8, size: usize) -> &'a str{
    let u8_slice = unsafe {slice::from_raw_parts(p,size)};
    let result = str::from_utf8(u8_slice).expect("vec<u8> to str err!");
    result
}

/// 字面量转bytes返回bytes指针和长度
pub fn str_to_pointer_and_size(s:&str) -> (*const u8,usize){
    let bytes = s.as_bytes();
    (bytes.as_ptr(), bytes.len())
}



