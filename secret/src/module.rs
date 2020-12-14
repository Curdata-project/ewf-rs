use serde::export::Vec;
use serde::{Serialize, Deserialize};
use alloc::boxed::Box;
use alloc::alloc::Global;
use crate::println;
use common_structure::get_rng_core;
use rand::RngCore;
use kv_object::sm2::{CertificateSm2, KeyPairSm2};
use asymmetric_crypto::prelude::Keypair;
use serde_json::Map;
use hex::{FromHex, ToHex};
use dislog_hal::Bytes;
use core::ops::Deref;

/// 密钥体
///     keypair:str：账户
///     cert:str：证书
///     last_tx_time:u32：最后交易时间戳
///     uid:str：uid
#[derive(Serialize, Deserialize, Clone)]
pub struct SecretBody{
    pub keypair:Box<str>,
    pub cert:Box<str>,
    pub secret_type:Box<str>,
    pub uid:Box<str>,
    pub seed:Box<str>,
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
    pub info: RegisterUserInfo,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SecretBodyList{
    pub list:Vec<SecretBody>
}

/// 服务器注册类
#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterParam{
    pub cert: Box<str>,
    pub info: RegisterUserInfo,
    pub hold:RegisterParamHold,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterParamHold{
    pub seed:Box<str>
}


#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterResp{
    pub cert:Box<str>,
    pub uid:Box<str>,
    pub hold:RegisterParamHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterUserInfo {
    pub account: Box<str>,
    pub password: Box<str>,
}

fn deserialize_secret(seed:&str,uid:&str,tp:&str,cert:&str) -> Option<SecretBody>{
    return match tp {
        "sm2"=>{
            let mut seed_ary = [0u8; 32];
            let u8_vec_seed = Vec::<u8>::from_hex(seed).expect("data incrrect");
            seed_ary.clone_from_slice(&u8_vec_seed);

            let new_cert = CertificateSm2::from_bytes(
                &Vec::<u8>::from_hex(cert).expect("data incrrect")
            ).expect("data incrrect");

            let keypair =  KeyPairSm2::generate_from_seed(seed_ary).expect("data incrrect");

            let cert_str = serde_json::to_string(&new_cert).unwrap();
            let keypair_str = serde_json::to_string(&keypair).unwrap();

            Some(SecretBody{
                keypair: Box::from(keypair_str.as_str()),
                cert: Box::from(cert_str.as_str()),
                secret_type: Box::from(tp),
                uid: Box::from(uid),
                seed:Box::from(""),
            })
        },
        _=>None,
    }
}

pub trait Exec {
    fn run(self);
}

impl Exec for SecretBodyList{
    fn run(self) {
        let mut sb_vec:Vec<SecretBody> = Vec::new();


        for sb in self.list {
            let new_sb = deserialize_secret(sb.seed.deref(),
                                        sb.uid.deref(),
                                        sb.secret_type.deref(),
                                        sb.cert.deref()).unwrap();
            sb_vec.push(new_sb);
        }

        let sb_list = SecretBodyList{
            list: sb_vec
        };

        let notify_body = super::common_module::NotifyBody{
            method: Box::from("notify_get_secret_list"),
            code: 0,
            msg: Box::from("success"),
            param: Box::from(serde_json::to_string(&sb_list).unwrap())
        };

        super::notify(&serde_json::to_string(&notify_body).unwrap());
    }
}

impl Exec for GenAndRegisterParam{
    fn run(self){

        let mut rng = get_rng_core();
        let mut seed_ary = [0u8; 32];
        rng.fill_bytes(&mut seed_ary);
        let keypair = KeyPairSm2::generate_from_seed(seed_ary).unwrap();
        let u8_vec_seed = keypair.0.get_code();

        let seed = hex::encode(u8_vec_seed);

        let hold = RegisterParamHold{
            seed: Box::from(seed.as_str())
        };

        let unregister_cert = keypair.get_certificate();
        let byte = unregister_cert.to_bytes();
        let cert = hex::encode(byte);
        let rp = RegisterParam{
            cert: Box::from(cert.as_str()),
            info: self.info,
            hold
        };

        let cb = super::common_module::CommonBody{
            method: Box::from("request_register"),
            param: Box::from(serde_json::to_string(&rp).unwrap())
        };
        //调用js注册方法
        super::request(serde_json::to_string(&cb).unwrap().as_str());

    }
}

impl Exec for RegisterResp{
    fn run(self) {
        //调用js的run方法
        let json = serde_json::to_value(&self.hold).unwrap();
        let seed = json.get("seed").unwrap().as_str().unwrap();
        //存库
        let value = alloc::format!("({},{},{},{},{})",self.uid,"sm2",seed,"",self.cert);

        let insert_sql = alloc::format!("{}{}",super::sql::INSERT_SECRET,value);
        //把需要执行的sql和返回的结果返回
        //让js端来执行sql和判断是否需要把数据返回给用户
        let secret_body = deserialize_secret(seed,self.uid.deref(),"sm2",self.cert.deref()).unwrap();
        let param = serde_json::json!({
            "result":secret_body,
            "sql":insert_sql,
        });

        let notify_body = super::common_module::NotifyBody{
            method: Box::from("notify_gen_and_register"),
            code: 0,
            msg: Box::from("success"),
            param: Box::from(serde_json::to_string(&param).unwrap())
        };
        //调用js的通知
        super::notify(&serde_json::to_string(&notify_body).unwrap())
    }
}

impl Exec for SecretBody {
    fn run(self) {
        let sb = deserialize_secret(self.seed.deref(),self.uid.deref(),self.secret_type.deref(),self.cert.deref()).unwrap();

        let notify_body = super::common_module::NotifyBody{
            method: Box::from("notify_get_secret"),
            code: 0,
            msg: Box::from("success"),
            param: Box::from(serde_json::to_string(&sb).unwrap())
        };

        super::notify(&serde_json::to_string(&notify_body).unwrap());

    }
}