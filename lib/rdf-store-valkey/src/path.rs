// This is free and unencumbered software released into the public domain.

use alloc::string::{String, ToString};
use core::str::FromStr;
use redis::{ToRedisArgs, ToSingleRedisArg};
use serde_json::Value;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValkeyPath(pub(crate) String);

impl From<String> for ValkeyPath {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<ValkeyPath> for String {
    fn from(input: ValkeyPath) -> Self {
        input.0
    }
}

impl From<&ValkeyPath> for String {
    fn from(input: &ValkeyPath) -> Self {
        input.0.clone()
    }
}

impl From<ValkeyPath> for Value {
    fn from(input: ValkeyPath) -> Self {
        Value::String(input.into())
    }
}

impl From<&ValkeyPath> for Value {
    fn from(input: &ValkeyPath) -> Self {
        Value::String(input.into())
    }
}

impl FromStr for ValkeyPath {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self(input.to_string()))
    }
}

impl ToSingleRedisArg for ValkeyPath {}

impl ToRedisArgs for ValkeyPath {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        // Escape '~' and '/' as per RFC 6901, without allocations:
        if self.0.contains('~') || self.0.contains('/') {
            out.write_arg_fmt(format_args!(
                "['{}']",
                self.0.replace('~', "~0").replace('/', "~1") // TODO: optimize
            ));
        } else {
            out.write_arg_fmt(format_args!("['{}']", self.0));
        }
    }
}
