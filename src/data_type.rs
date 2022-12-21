use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub enum DataType {
    Named(String),
    Nullable(Box<DataType>),
    Union(Vec<DataType>),
    Intersection(Vec<DataType>),
    Void,
    Null,
    True,
    False,
    Never,
    Float,
    Boolean,
    Integer,
    String,
    Array,
    Object,
    Mixed,
    Callable,
    Iterable,
    StaticReference,
    SelfReference,
    ParentReference,
}

impl Generator for DataType {
    fn generate(&self, _indentation: Indentation, _level: usize) -> String {
        match self {
            DataType::Named(name) => name.to_string(),
            DataType::Nullable(inner) => {
                format!("?{}", inner.generate(_indentation, _level))
            }
            DataType::Union(inner) => inner
                .iter()
                .map(|t| t.generate(_indentation, _level))
                .collect::<Vec<String>>()
                .join("|"),
            DataType::Intersection(inner) => inner
                .iter()
                .map(|t| t.generate(_indentation, _level))
                .collect::<Vec<String>>()
                .join("&"),
            DataType::Null => "null".to_string(),
            DataType::True => "true".to_string(),
            DataType::False => "false".to_string(),
            DataType::Float => "float".to_string(),
            DataType::Boolean => "bool".to_string(),
            DataType::Integer => "int".to_string(),
            DataType::String => "string".to_string(),
            DataType::Array => "array".to_string(),
            DataType::Object => "object".to_string(),
            DataType::Mixed => "mixed".to_string(),
            DataType::Callable => "callable".to_string(),
            DataType::Iterable => "iterable".to_string(),
            DataType::StaticReference => "static".to_string(),
            DataType::SelfReference => "self".to_string(),
            DataType::ParentReference => "parent".to_string(),
            DataType::Void => "void".to_string(),
            DataType::Never => "never".to_string(),
        }
    }
}
