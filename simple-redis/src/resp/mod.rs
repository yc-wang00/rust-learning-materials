mod encode;
mod decode;

use std::{collections::{HashMap, HashSet}};



pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(Vec<u8>),
    Array(Vec<RespFrame>),
    NullBulkString(RespNullBulkString),
    NullArray(RespNullArray),
    Null(RespNull),

    Boolean(bool),
    Double(f64),
    Map(HashMap<String, RespFrame>),
    Set(HashSet<RespFrame>),
}

pub struct SimpleString(String);
pub struct SimpleError(String);

pub struct RespNull;

pub struct RespNullArray;

pub struct RespNullBulkString;

pub trait RespEncode {
    fn encode(buf: Self) -> Vec<u8>;
}


pub trait RespDecode {
    fn decode(buf: Self) -> Vec<u8>;
}

impl RespDecode for RespFrame {
    fn decode(buf: Self) -> Vec<u8> {
        todo!()
    }
    
}