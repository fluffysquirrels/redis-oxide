/// These types are used by engine and ops to actually perform useful work.
pub type Value = Vec<u8>;
/// Key is the standard type to index our structures
pub type Key = Vec<u8>;
/// Count is used for commands that count.
pub type Count = i64;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum RedisValue {
    SimpleString(Value),
    Error(Value),
    BulkString(Value),
    Int(i64), // is it always i64?
    Array(Vec<RedisValue>),
    NullArray,
    NullBulkString,
}

pub const NULL_BULK_STRING: &str = "$-1\r\n";
pub const NULL_ARRAY: &str = "*-1\r\n";
pub const EMPTY_ARRAY: &str = "*0\r\n";

#[derive(Debug)]
pub enum EngineRes {
    Ok,
    StringRes(Value),
    Error(&'static [u8]),
    MultiStringRes(Vec<Value>),
    UIntRes(usize),
    Nil,
}
