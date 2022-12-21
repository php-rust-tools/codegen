use crate::{attribute::AttributeGroup, comment::Document, method::Method, Generator, Indentation};

#[derive(Debug)]
pub struct Interface {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub extends: Option<String>,
    pub methods: Vec<Method>,
}

impl Interface {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            extends: None,
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

    pub fn extends<T: ToString>(mut self, extends: T) -> Self {
        self.extends = Some(extends.to_string());

        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method.public());

        self
    }
}

impl Generator for Interface {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(documentation) = &self.documentation {
            code.push_str(&documentation.generate(indentation, level));
        }

        for attribute in &self.attributes {
            code.push_str(&attribute.generate(indentation, level));
        }

        code.push_str(&format!("interface {}", self.name));

        if let Some(extends) = &self.extends {
            code.push_str(&format!(" extends {}", extends));
        }

        code.push_str("\n{\n");

        code.push_str(self.methods.generate(indentation, level + 1).as_str());

        code = code.trim_end().to_string();
        code.push_str("\n}\n");

        code
    }
}

impl Generator for Vec<Interface> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();
        if self.is_empty() {
            return code;
        }

        for interface in self {
            code.push_str(interface.generate(indentation, level).as_str());
            code.push('\n');
        }

        code
    }
}
