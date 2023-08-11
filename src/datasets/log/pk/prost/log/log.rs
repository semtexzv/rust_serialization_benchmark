#![allow(unused_imports)]
#![allow(nonstandard_style)]
#![allow(unreachable_patterns)]
#![allow(clippy::module_inception)]
use ::protokit::*;
pub fn register_types(_registry: &mut ::protokit::textformat::reflect::Registry) {}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Address {
    #[field(1u32, "x0", varint, singular)]
    pub x0: u32,
    #[field(2u32, "x1", varint, singular)]
    pub x1: u32,
    #[field(3u32, "x2", varint, singular)]
    pub x2: u32,
    #[field(4u32, "x3", varint, singular)]
    pub x3: u32,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Log {
    #[field(1u32, "address", nested, optional)]
    pub address: Option<Box<Address>>,
    #[field(2u32, "identity", string, singular)]
    pub identity: String,
    #[field(3u32, "userid", string, singular)]
    pub userid: String,
    #[field(4u32, "date", string, singular)]
    pub date: String,
    #[field(5u32, "request", string, singular)]
    pub request: String,
    #[field(6u32, "code", varint, singular)]
    pub code: u32,
    #[field(7u32, "size", varint, singular)]
    pub size: u64,
}
#[derive(Debug, Default, Clone, PartialEq, Proto)]
pub struct Logs {
    #[field(1u32, "logs", nested, repeated)]
    pub logs: Vec<Log>,
}
