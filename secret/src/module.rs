use serde::export::Vec;
use serde::{Serialize, Deserialize};
use alloc::boxed::Box;
use alloc::alloc::Global;
use crate::println;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config{
    pub db:Box<str>,
}

/// 请求体
/// 根据方法来确定调用不同的子函数
///     method:str
///         gen_and_register 生成并注册密钥
///         get_secret_list 分页获取密钥列表
///         get_secret  根据uid获取密钥
///     param:str
///         GenAndRegisterParam 生成并注册密钥参数
///         GetSecretListParam  分页获取密钥列表参数
///         GetSecretParam  根据uid获取密钥参数
#[derive(Serialize, Deserialize, Clone)]
pub struct RequestBody{
    pub method:Box<str>,
    pub param:Box<str>,
}

/// 返回体
///     code:u8
///         0成功、!0失败
///     msg:str
///         不管成功还是失败都会有信息
///     result:str
///         SecretBody密钥体的对象或者列表
#[derive(Serialize, Deserialize, Clone)]
pub struct ResponseBody{
    pub code:u8,
    pub msg:Box<str>,
    pub result:Box<str>,
}

/// 密钥体
///     keypair:str：账户
///     cert:str：证书
///     last_tx_time:u32：最后交易时间戳
///     uid:str：uid
#[derive(Serialize, Deserialize, Clone)]
pub struct SecretBody{
    pub keypair:Box<str>,
    pub cert:Box<str>,
    pub secret_type:u32,
    pub uid:Box<str>,
}

/// 生成并注册密钥参数
///     url:string：注册服务地址
///     timeout:u32：超时时间，ms
///     account:string：账户
///     password:string：密码
#[derive(Serialize, Deserialize, Clone)]
pub struct GenAndRegisterParam{
    pub url:Box<str>,
    pub timeout:u32,
    pub account:Box<str>,
    pub password:Box<str>,
}

/// 获取密钥列表
///     page_items:u32：条目
///     page_num:u32：页数
#[derive(Serialize, Deserialize, Clone)]
pub struct GetSecretListParam{
    pub page_items:u32,
    pub page_num:u32,
}

/// 获取密钥
///     uid:string：用户uid
#[derive(Serialize, Deserialize, Clone)]
pub struct GetSecretParam{
    pub uid:Box<str>
}


impl RequestBody {
    pub fn run(self) -> Box<str>{
        let str_method = crate::u8_pointer_and_size_to_str(self.method.as_ptr(), self.method.len());
        let str_param = crate::u8_pointer_and_size_to_str(self.param.as_ptr(), self.param.len());
        match str_method {
            "gen_and_register" => {
                let param:GenAndRegisterParam = serde_json::from_str(str_param).unwrap();
                param.run()
            },
            "get_secret_list" => {
                let param:GetSecretListParam = serde_json::from_str(str_param).expect("");
                param.run()
            }
            "get_secret" => {
                let param:GetSecretParam = serde_json::from_str(str_param).expect("");
                param.run()
            },
            _ => Box::from("unknown"),
        }
    }
}

pub trait Exec {
    fn run(self) -> Box<str>;
}

impl Exec for GenAndRegisterParam{
    fn run(self) -> Box<str>{
        Box::from("gen_and_register run")


    }
}

impl Exec for GetSecretListParam{
    fn run(self) -> Box<str, Global> {
        Box::from("get_secret_list run")
    }
}

impl Exec for GetSecretParam{
    fn run(self) -> Box<str, Global> {
        //先查库看看uid能不能查出对应的数据
        let find_by_uid = alloc::format!( "{}\"{}\"",super::sql::SELECT_SECRET_BY_UID,self.uid);
        println(find_by_uid.as_str());
        crate::exec_sql(find_by_uid.as_str());
        //等待结果？怎么取数据？
        //等待js发一个notify给我，我在notifyBody里边来继续处理
        //无需关注js返回的结果，如果查到就返回数据给调用者，没查到就返回错误
        Box::from("get_secret run")

    }
}