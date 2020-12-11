#![no_std]

pub mod module;

extern crate alloc;
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;