use crate::error::{Error, Result};
use serde::{
    Deserialize, Deserializer,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::{Map, Number, Value};
use std::{fmt, fs, io::Read, path::Path};

const MAX_BYTES: u64 = 4 * 1024 * 1024;

pub(crate) fn load(path: &Path) -> Result<Value> {
    if !fs::symlink_metadata(path)?.is_file() {
        return Err(Error::Json);
    }
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(MAX_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_BYTES {
        return Err(Error::Json);
    }
    serde_json::from_slice::<Strict>(&bytes)
        .map(|v| v.0)
        .map_err(|_| Error::Json)
}

struct Strict(Value);
impl<'de> Deserialize<'de> for Strict {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        struct Unique;
        impl<'de> Visitor<'de> for Unique {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON with unique object properties")
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_f64<E: de::Error>(self, v: f64) -> std::result::Result<Strict, E> {
                Number::from_f64(v)
                    .map(|n| Strict(Value::Number(n)))
                    .ok_or_else(|| E::custom("non-finite JSON number"))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_string<E: de::Error>(self, v: String) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut sequence: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut values = Vec::new();
                while let Some(value) = sequence.next_element::<Strict>()? {
                    values.push(value.0);
                }
                Ok(Strict(Value::Array(values)))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut source: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut values = Map::new();
                while let Some(key) = source.next_key::<String>()? {
                    if values.contains_key(&key) {
                        return Err(de::Error::custom("duplicate JSON property"));
                    }
                    values.insert(key, source.next_value::<Strict>()?.0);
                }
                Ok(Strict(Value::Object(values)))
            }
        }
        deserializer.deserialize_any(Unique)
    }
}
