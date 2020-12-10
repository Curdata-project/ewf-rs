#![no_std]
#![feature(fmt_as_str)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(default_alloc_error_handler)]
#![feature(associated_type_defaults)]


mod module;

extern crate alloc;


use core::{mem, slice};
use core::alloc::Layout;
use core::str;
use utils::*;
use serde::export::Vec;
use alloc::boxed::Box;
use serde_json::Value;
use core::ops::Deref;

extern crate utils;
extern crate secret;


#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// js提供的方法
extern "C" {
    fn _print(p: *const u8, size: usize);
    fn _notify(p: *const u8, size: usize);
    fn _run(p: *const u8, size: usize);
}


/// 在js环境中打印信息
pub fn println(s: &str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _print(pair.0, pair.1);
    }
}

pub fn notify_to_js(s:&str){
    let pair = str_to_pointer_and_size(s);
    unsafe {
        _notify(pair.0,pair.1);
    }
}

#[no_mangle]
pub extern "C" fn __main(p:*const u8, size: usize){
    let u8_slice = unsafe {slice::from_raw_parts(p,size)};
    let str = str::from_utf8(u8_slice).expect("vec<u8> to str err!");
    let rb: secret::module::RequestBody = serde_json::from_str(str).expect("");
    let result = rb.run();
    println(&result);
    notify_to_js(&result);

}

/// 生成一个C方法，并且要求编译器不修改这个方法
/// 首先给个对齐位
/// 然后根据对齐位来生成内存布局
/// 根据布局参数来分配内存，生成一个u8指针
#[no_mangle]
pub extern "C" fn __wbindgen_malloc(size: usize) -> *mut u8 {
    let align = mem::align_of::<usize>();
    //获取布局，不能为0
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            if layout.size() > 0 {
                //分配内存
                let ptr = alloc::alloc::alloc(layout);
                if !ptr.is_null() {
                    return ptr
                }
            } else {
                return align as *mut u8
            }
        }
    }
    loop{}
}

/// js通知方法
#[no_mangle]
pub extern "C" fn __notify(p:*const u8, size: usize){
    let u8_slice = unsafe {slice::from_raw_parts(p,size)};

}