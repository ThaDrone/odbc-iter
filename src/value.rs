use chrono::naive::{NaiveDate, NaiveDateTime, NaiveTime};
use chrono::{Datelike, Timelike};
use odbc::{SqlDate, SqlSsTime2, SqlTime, SqlTimestamp};
use std::fmt;

pub type ValueRow = Vec<Option<Value>>;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bit(bool),
    Tinyint(i8),
    Smallint(i16),
    Integer(i32),
    Bigint(i64),
    Float(f32),
    Double(f64),
    String(String),
    Timestamp(SqlTimestamp),
    Date(SqlDate),
    Time(SqlSsTime2),
}

/// Note that `as_` methods return reference so values can be parameter-bound to a query
/// Use `to_` or `into_` methods to get values cheaply
impl Value {
    pub fn as_bool(&self) -> Option<&bool> {
        match self {
            Value::Bit(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        self.as_bool().cloned()
    }

    pub fn as_i8(&self) -> Option<&i8> {
        match self {
            Value::Tinyint(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_i8(&self) -> Option<i8> {
        self.as_i8().cloned()
    }

    pub fn as_i16(&self) -> Option<&i16> {
        match self {
            Value::Smallint(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_i16(&self) -> Option<i16> {
        self.as_i16().cloned()
    }

    pub fn as_i32(&self) -> Option<&i32> {
        match self {
            Value::Integer(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        self.as_i32().cloned()
    }

    pub fn as_i64(&self) -> Option<&i64> {
        match self {
            Value::Bigint(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        self.as_i64().cloned()
    }

    pub fn as_f32(&self) -> Option<&f32> {
        match self {
            Value::Float(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_f32(&self) -> Option<f32> {
        self.as_f32().cloned()
    }

    pub fn as_f64(&self) -> Option<&f64> {
        match self {
            Value::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        self.as_f64().cloned()
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Result<String, Value> {
        match self {
            Value::String(value) => Ok(value),
            _ => Err(self),
        }
    }

    pub fn as_timestamp(&self) -> Option<&SqlTimestamp> {
        match self {
            Value::Timestamp(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_naive_date_time(&self) -> Option<NaiveDateTime> {
        self.as_timestamp().map(|value| {
            NaiveDate::from_ymd(value.year as i32, value.month as u32, value.day as u32)
                .and_hms_nano(
                    value.hour as u32,
                    value.minute as u32,
                    value.second as u32,
                    value.fraction,
                )
        })
    }

    pub fn as_date(&self) -> Option<&SqlDate> {
        match self {
            Value::Date(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_naive_date(&self) -> Option<NaiveDate> {
        self.as_date().map(|value| {
            NaiveDate::from_ymd(value.year as i32, value.month as u32, value.day as u32)
        })
    }

    pub fn as_time(&self) -> Option<&SqlSsTime2> {
        match self {
            Value::Time(value) => Some(value),
            _ => None,
        }
    }

    pub fn to_naive_time(&self) -> Option<NaiveTime> {
        self.as_time().map(|value| {
            NaiveTime::from_hms_nano(
                value.hour as u32,
                value.minute as u32,
                value.second as u32,
                value.fraction,
            )
        })
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Value {
        Value::Bit(value)
    }
}

impl From<i8> for Value {
    fn from(value: i8) -> Value {
        Value::Tinyint(value)
    }
}

impl From<i16> for Value {
    fn from(value: i16) -> Value {
        Value::Smallint(value)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Value {
        Value::Integer(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Value {
        Value::Bigint(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Value {
        Value::Float(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Value {
        Value::Double(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Value {
        Value::String(value)
    }
}

impl From<NaiveDateTime> for Value {
    fn from(value: NaiveDateTime) -> Value {
        Value::Timestamp(SqlTimestamp {
            day: value.day() as u16,
            month: value.month() as u16,
            year: value.year() as i16,
            hour: value.hour() as u16,
            minute: value.minute() as u16,
            second: value.second() as u16,
            fraction: value.nanosecond(),
        })
    }
}

impl From<SqlTimestamp> for Value {
    fn from(value: SqlTimestamp) -> Value {
        Value::Timestamp(value)
    }
}

use crate::odbc_type::UnixTimestamp;
impl From<UnixTimestamp> for Value {
    fn from(value: UnixTimestamp) -> Value {
        Value::Timestamp(value.into_inner())
    }
}

impl From<NaiveDate> for Value {
    fn from(value: NaiveDate) -> Value {
        Value::Date(SqlDate {
            day: value.day() as u16,
            month: value.month() as u16,
            year: value.year() as i16,
        })
    }
}

impl From<SqlDate> for Value {
    fn from(value: SqlDate) -> Value {
        Value::Date(value)
    }
}

impl From<NaiveTime> for Value {
    fn from(value: NaiveTime) -> Value {
        Value::Time(SqlSsTime2 {
            hour: value.hour() as u16,
            minute: value.minute() as u16,
            second: value.second() as u16,
            fraction: value.nanosecond(),
        })
    }
}

impl From<SqlTime> for Value {
    fn from(value: SqlTime) -> Value {
        Value::Time(SqlSsTime2 { hour: value.hour,
            minute: value.minute,
            second: value.second,
            fraction: 0,
        })
    }
}

impl From<SqlSsTime2> for Value {
    fn from(value: SqlSsTime2) -> Value {
        Value::Time(value)
    }
}

impl fmt::Display for Value {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Bit(ref b) => fmt::Display::fmt(b, f),
            Value::Tinyint(ref n) => fmt::Display::fmt(n, f),
            Value::Smallint(ref n) => fmt::Display::fmt(n, f),
            Value::Integer(ref n) => fmt::Display::fmt(n, f),
            Value::Bigint(ref n) => fmt::Display::fmt(n, f),
            Value::Float(ref n) => fmt::Display::fmt(n, f),
            Value::Double(ref n) => fmt::Display::fmt(n, f),
            Value::String(ref s) => fmt::Display::fmt(s, f),
            Value::Timestamp(ref timestamp) => write!(f,
                "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:03}",
                timestamp.year,
                timestamp.month,
                timestamp.day,
                timestamp.hour,
                timestamp.minute,
                timestamp.second,
                timestamp.fraction / 1_000_000),
            Value::Date(ref date) => write!(f, 
                "{:04}-{:02}-{:02}",
                date.year, date.month, date.day),
            Value::Time(ref time) => write!(f, 
                "{:02}:{:02}:{:02}.{:03}",
                time.hour, time.minute, time.second, time.fraction / 1_000_000),
        }
     }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NullableValue(Option<Value>);

impl NullableValue {
    pub fn into_option(self) -> Option<Value> {
        self.0
    }
}

impl fmt::Display for NullableValue {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(ref v) => fmt::Display::fmt(v, f),
            None => write!(f, "NULL"),
        }
     }
}

pub trait IntoNullable {
    fn into_nullable(self) -> NullableValue;
}

impl IntoNullable for Option<Value> {
    /// Convert to NullableValue that implements Display for None variant
    fn into_nullable(self) -> NullableValue {
        NullableValue(self)
    }
}

#[cfg(feature = "serde")]
mod ser {
    use serde::{self, Serialize};
    use super::*;
    //TODO: ValueRow (as Vec) and SchemaAccess (Map)

    impl Serialize for Value {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            match *self {
                Value::Bit(b) => serializer.serialize_bool(b),
                Value::Tinyint(n) => serializer.serialize_i8(n),
                Value::Smallint(n) => serializer.serialize_i16(n),
                Value::Integer(n) => serializer.serialize_i32(n),
                Value::Bigint(n) => serializer.serialize_i64(n),
                Value::Float(n) => serializer.serialize_f32(n),
                Value::Double(n) => serializer.serialize_f64(n),
                Value::String(ref s) => serializer.serialize_str(s),
                value @ Value::Timestamp(_) | 
                Value::Date(_) |
                Value::Time(_) => serializer.serialize_str(&value.to_string())
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn serialize_value_primitive() {
            assert_eq!(&serde_json::to_string(&Value::Bit(true)).unwrap(), "true");
            assert_eq!(&serde_json::to_string(&Value::Bit(false)).unwrap(), "false");

            assert_eq!(&serde_json::to_string(&Value::Integer(-1)).unwrap(), "-1");
            assert_eq!(&serde_json::to_string(&Value::Integer(22)).unwrap(), "22");

            assert_eq!(&serde_json::to_string(&Value::Double(-1.1)).unwrap(), "-1.1");
            assert_eq!(&serde_json::to_string(&Value::Double(33.22)).unwrap(), "33.22");

            assert_eq!(&serde_json::to_string(&Value::String("foo".to_owned())).unwrap(), "\"foo\"");
            assert_eq!(&serde_json::to_string(&Value::String("bar baz".to_owned())).unwrap(), "\"bar baz\"");
        }

        #[test]
        fn serialize_value_timestamp() {
            assert_eq!(&serde_json::to_string(&Value::from(NaiveDate::from_ymd(2016, 7, 8).and_hms_milli(9, 10, 11, 23))).unwrap(), "\"2016-07-08 09:10:11.023\"");
            assert_eq!(&serde_json::to_string(&Value::from(NaiveDate::from_ymd(2016, 12, 8).and_hms_milli(19, 1, 1, 0))).unwrap(), "\"2016-12-08 19:01:01.000\"");
        }

        #[test]
        fn serialize_value_date() {
            assert_eq!(&serde_json::to_string(&Value::from(NaiveDate::from_ymd(2016, 7, 8))).unwrap(), "\"2016-07-08\"");
            assert_eq!(&serde_json::to_string(&Value::from(NaiveDate::from_ymd(2016, 12, 8))).unwrap(), "\"2016-12-08\"");
        }

        #[test]
        fn serialize_value_time() {
            assert_eq!(&serde_json::to_string(&Value::from(NaiveTime::from_hms_milli(9, 10, 11, 23))).unwrap(), "\"09:10:11.023\"");
            assert_eq!(&serde_json::to_string(&Value::from(NaiveTime::from_hms_milli(19, 1, 1, 0))).unwrap(), "\"19:01:01.000\"");
        }
    }
}