use crate::attribute::AttributeGroup;
use crate::comment::Document;
use crate::literal::Value;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct EnumCase {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub value: Option<Value>,
}

impl EnumCase {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            value: None,
        }
    }

    pub fn document(mut self, documentation: Document) -> Self {
        self.documentation = Some(documentation);

        self
    }

    pub fn attributes(mut self, attributes: AttributeGroup) -> Self {
        self.attributes.push(attributes);

        self
    }

    pub fn valued<T: Into<Value>>(mut self, value: T) -> Self {
        self.value = Some(value.into());

        self
    }
}

impl Generator for EnumCase {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(document) = &self.documentation {
            code.push_str(&document.generate(indentation, level));
        }

        for attribute in &self.attributes {
            code.push_str(&attribute.generate(indentation, level));
        }

        code.push_str(&indentation.indent(format!("case {}", self.name), level));

        if let Some(value) = &self.value {
            code.push_str(&format!(" = {};\n", value.generate(indentation, level)));
        } else {
            code.push_str(";\n");
        }

        code
    }
}

impl Generator for Vec<EnumCase> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();
        if self.is_empty() {
            return code;
        }

        for case in self {
            code.push_str(case.generate(indentation, level).as_str());
            code.push('\n');
        }

        code
    }
}

impl<T: ToString, Tv: Into<Value>> From<(T, Tv)> for EnumCase {
    fn from((name, value): (T, Tv)) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            value: Some(value.into()),
        }
    }
}

impl From<String> for EnumCase {
    fn from(name: String) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name,
            value: None,
        }
    }
}

impl From<&str> for EnumCase {
    fn from(name: &str) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            value: None,
        }
    }
}
