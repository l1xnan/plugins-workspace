use rust_decimal::prelude::ToPrimitive;
use serde_json::Value as JsonValue;
use sqlx::{postgres::PgValueRef, types::Decimal, TypeInfo, Value, ValueRef};
use time::{Date, OffsetDateTime, PrimitiveDateTime, Time};

use crate::Error;

pub(crate) fn to_json(v: PgValueRef) -> Result<JsonValue, Error> {
    if v.is_null() {
        return Ok(JsonValue::Null);
    }

    let res = match v.type_info().name() {
        "CHAR" | "VARCHAR" | "TEXT" | "NAME" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode() {
                JsonValue::String(v)
            } else {
                JsonValue::Null
            }
        }
        "NUMERIC" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Decimal>() {
                JsonValue::from(v.to_f64())
            } else {
                JsonValue::Null
            }
        }
        "FLOAT4" | "FLOAT8" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<f64>() {
                JsonValue::from(v)
            } else {
                JsonValue::Null
            }
        }
        "INT2" | "INT4" | "INT8" | "INTEGER" | "SMALLSERIAL" | "SERIAL" | "BIGSERIAL" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<i64>() {
                JsonValue::Number(v.into())
            } else {
                JsonValue::Null
            }
        }
        "BOOL" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode() {
                JsonValue::Bool(v)
            } else {
                JsonValue::Null
            }
        }
        "DATE" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Date>() {
                JsonValue::String(v.to_string())
            } else {
                JsonValue::Null
            }
        }
        "TIME" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Time>() {
                JsonValue::String(v.to_string())
            } else {
                JsonValue::Null
            }
        }
        "TIMESTAMP" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<PrimitiveDateTime>() {
                JsonValue::String(v.to_string())
            } else {
                JsonValue::Null
            }
        }
        "TIMESTAMPTZ" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<OffsetDateTime>() {
                JsonValue::String(v.to_string())
            } else {
                JsonValue::Null
            }
        }
        "JSON" | "JSONB" => ValueRef::to_owned(&v).try_decode().unwrap_or_default(),
        "BYTEA" => {
            if let Ok(v) = ValueRef::to_owned(&v).try_decode::<Vec<u8>>() {
                JsonValue::Array(v.into_iter().map(|n| JsonValue::Number(n.into())).collect())
            } else {
                JsonValue::Null
            }
        }
        "VOID" => JsonValue::Null,
        _ => return Err(Error::UnsupportedDatatype(v.type_info().name().to_string())),
    };

    Ok(res)
}
