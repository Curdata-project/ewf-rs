#![no_std]
#![feature(fmt_as_str)]
#![feature(alloc_error_handler)] // at the top of the file
#![feature(default_alloc_error_handler)]
#![feature(associated_type_defaults)]



extern crate alloc;


use core::{mem, slice};
use core::alloc::Layout;
use core::str;
use utils::*;
use common::*;
use serde::export::Vec;
use alloc::boxed::Box;
use serde_json::Value;
use core::ops::Deref;
use secret::module::Exec;

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
    let str = u8_pointer_and_size_to_str(p, size);
    let cb:common::module::CommonBody = serde_json::from_str(str).unwrap();

    match cb.method.deref() {
        "gen_and_register"=>{
            let garp:secret::module::GenAndRegisterParam = serde_json::from_str(cb.param.deref()).unwrap();
            garp.run();
        },
        "get_secret_list"=>{
            let sb_list:secret::module::SecretBodyList = serde_json::from_str(cb.param.deref()).unwrap();
            sb_list.run();
        },
        "get_secret"=>{
            let sb:secret::module::SecretBody = serde_json::from_str(cb.param.deref()).unwrap();
            sb.run();
        }
        _=>(),
    }
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
    let str = u8_pointer_and_size_to_str(p, size);

    let nb:common::module::NotifyBody = serde_json::from_str(str).unwrap();

    //错误处理，其实我这不用错误处理，js那出错就直接返回
    if nb.code != 0 { }

    match nb.method.deref() {
        "notify_gen_and_register" => {
            let rr:secret::module::RegisterResp = serde_json::from_str(nb.param.deref()).unwrap();
            rr.run();
        },
        "notify_get_secret" => {
            let sb:secret::module::SecretBody = serde_json::from_str(nb.param.deref()).unwrap();
            sb.run();
        },
        _ => {},
    }

}