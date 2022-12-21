use crate::attribute::AttributeGroup;
use crate::comment::Document;
use crate::constant::ClassConstant;
use crate::enum_case::EnumCase;
use crate::method::Method;
use crate::usage::Usage;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub enum EnumBackingType {
    Int,
    String,
}

#[derive(Debug)]
pub struct Enum {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub backing_type: Option<EnumBackingType>,
    pub implements: Vec<String>,
    pub usages: Vec<Usage>,
    pub constants: Vec<ClassConstant>,
    pub cases: Vec<EnumCase>,
    pub methods: Vec<Method>,
}

impl Enum {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            backing_type: None,
            implements: vec![],
            usages: vec![],
            constants: vec![],
            cases: vec![],
            methods: vec![],
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

    pub fn implements<T: ToString>(mut self, implements: T) -> Self {
        self.implements.push(implements.to_string());

        self
    }

    pub fn int_backed(mut self) -> Self {
        self.backing_type = Some(EnumBackingType::Int);

        self
    }

    pub fn string_backed(mut self) -> Self {
        self.backing_type = Some(EnumBackingType::String);

        self
    }

    pub fn backed(mut self, backing_type: EnumBackingType) -> Self {
        self.backing_type = Some(backing_type);

        self
    }

    pub fn using<T: Into<Usage>>(mut self, usage: T) -> Self {
        self.usages.push(usage.into());

        self
    }

    pub fn constant<T: Into<ClassConstant>>(mut self, constant: T) -> Self {
        self.constants.push(constant.into());

        self
    }

    pub fn case<T: Into<EnumCase>>(mut self, case: T) -> Self {
        self.cases.push(case.into());

        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method);

        self
    }
}

impl Generator for Enum {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(documentation) = &self.documentation {
            code.push_str(&documentation.generate(indentation, level));
        }

        for attribute in &self.attributes {
            code.push_str(&attribute.generate(indentation, level));
        }

        code.push_str(&format!("enum {}", self.name));

        if let Some(backing_type) = &self.backing_type {
            code.push_str(&format!(
                ": {}",
                match backing_type {
                    EnumBackingType::Int => "int",
                    EnumBackingType::String => "string",
                }
            ));
        }

        if !self.implements.is_empty() {
            code.push_str(&format!(
                " implements {}",
                self.implements
                    .iter()
                    .map(|implement| implement.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        code.push_str("\n{\n");

        code.push_str(self.usages.generate(indentation, level + 1).as_str());
        code.push_str(self.constants.generate(indentation, level + 1).as_str());
        code.push_str(self.cases.generate(indentation, level + 1).as_str());
        code.push_str(self.methods.generate(indentation, level + 1).as_str());

        code = code.trim_end().to_string();
        code.push_str("\n}\n");

        code
    }
}

impl Generator for Vec<Enum> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();
        if self.is_empty() {
            return code;
        }

        for r#enum in self {
            code.push_str(r#enum.generate(indentation, level).as_str());
            code.push('\n');
        }

        code
    }
}
