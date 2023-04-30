#[macro_use] mod macros;
use std::collections::HashMap;

use anyhow::{Result, anyhow};
use serde_json::Value;

pub fn get_sum(v: &Value) -> Result<i64> {
    
    let value = match v {
        Value::Number(n) => n.as_i64()
            .ok_or(anyhow!("issue with number: {}", n))?,
        Value::Array(vv) => vv
            .iter()
            .filter_map(|vvv| get_sum(&vvv).ok())
            .sum(),
        Value::Object(o) => {
            let contains_red = o
            .values()
            .any(|ov| {
                if let Value::String(os) = ov {
                    if os == "red" {
                        return true;
                    }
                }
                return false;
            });
            match contains_red {
                true => 0,
                false => o
                    .values()
                    .filter_map(|ov| get_sum(ov).ok())
                    .sum()
            }
        },
        _ => 0
    };

    Ok(value)
}

impl_from_num_for_json!(u8 i8 u16 i16 u32 i32 u64 i64 u128 i128 usize isize f32 f64);

#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String,Json>>)
}

impl From<bool> for Json {
    fn from(b: bool) -> Json {
        Json::Boolean(b)
    }
}

impl From<String> for Json {
    fn from(s: String) -> Json {
        Json::String(s)
    }
}

impl<'a> From<&'a str> for Json {
    fn from(s: &'a str) -> Json {
        Json::String(s.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json_null() {
        // Act & Assert
        assert_eq!(json!(null), Json::Null);
    }

    #[test]
    fn json_array_with_json_element() {
        // Arrange
        let expected = Json::Array(vec![
            Json::Object(Box::new(
                vec![("answer".to_string(), Json::Number(42.0))]
                    .into_iter().collect()))
        ]);

        // Act
        let actual = json!(
            [
                {
                    "answer": 42
                }
            ]
        );

        // Assert
        assert_eq!(actual, expected);        
    }
}