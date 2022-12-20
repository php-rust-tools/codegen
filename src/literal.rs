use crate::Generator;
use crate::Indentation;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Null,
    True,
    False,
    Float(f64),
    Integer(i64),
    String(String),
    Literal(String),
    List(Vec<Value>),
    HashMap(Vec<(Value, Value)>),
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        if value {
            Value::True
        } else {
            Value::False
        }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Integer(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl<T: Into<Value>> From<Vec<T>> for Value {
    fn from(value: Vec<T>) -> Self {
        Value::List(value.into_iter().map(|value| value.into()).collect())
    }
}

impl<T: Into<Value>> From<Vec<(T, T)>> for Value {
    fn from(value: Vec<(T, T)>) -> Self {
        Value::HashMap(
            value
                .into_iter()
                .map(|(key, value)| (key.into(), value.into()))
                .collect(),
        )
    }
}

impl Generator for Value {
    fn generate(&self, identation: Indentation, level: usize) -> String {
        match self {
            Value::Null => "null".to_string(),
            Value::True => "true".to_string(),
            Value::False => "false".to_string(),
            Value::Integer(value) => value.to_string(),
            Value::String(value) => format!("\"{}\"", value),
            Value::Float(value) => value.to_string(),
            Value::Literal(value) => value.to_string(),
            Value::List(values) => {
                let mut result = String::new();

                result.push('[');
                result.push_str(
                    &values
                        .iter()
                        .map(|value| value.generate(identation, level))
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                result.push(']');

                result
            }
            Value::HashMap(values) => {
                let mut result = String::new();

                result.push('[');
                result.push_str(
                    &values
                        .iter()
                        .map(|(key, value)| {
                            format!(
                                "{} => {}",
                                key.generate(identation, level),
                                value.generate(identation, level)
                            )
                        })
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                result.push(']');

                result
            }
        }
    }
}
