use serde::{Serialize, Deserialize};
use alloc::boxed::Box;

/// js和wasm互调的结构体
///     js-to-wasm
///         method:gen_and_register:生成并注册
///         method:get_secret_list:获取密钥列表
///         method:get_secret:获取密钥
///     wasm-to-js
///         method:request_register
#[derive(Serialize, Deserialize, Clone)]
pub struct CommonBody{
    pub method:Box<str>,
    pub param:Box<str>,
}

/// js和wasm互调的结构体
///     js-to-wasm
///         method:notify_gen_and_register:生成并注册，服务注册通知
///         method:notify_get_secret_list:返回查到的list
///     wasm-to-js
///         method:notify_gen_and_register:生成并注册，返回结果和sql
///         method:notify_get_secret:根据uid获取secret，返回结果
///         method:notify_get_secret_list:返回加工过后的list
#[derive(Serialize, Deserialize, Clone)]
pub struct NotifyBody{
    pub method:Box<str>,
    pub code:i32,
    pub msg:Box<str>,
    pub param:Box<str>
}

