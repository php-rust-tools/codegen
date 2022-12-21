use crate::attribute::AttributeGroup;
use crate::comment::Document;
use crate::constant::ClassConstant;
use crate::method::Method;
use crate::property::Property;
use crate::usage::Usage;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct Trait {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub usages: Vec<Usage>,
    pub constants: Vec<ClassConstant>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
}

impl Trait {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            name: name.to_string(),
            usages: vec![],
            constants: vec![],
            properties: vec![],
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

    pub fn using<T: Into<Usage>>(mut self, usage: T) -> Self {
        self.usages.push(usage.into());

        self
    }

    pub fn constant<T: Into<ClassConstant>>(mut self, constant: T) -> Self {
        self.constants.push(constant.into());

        self
    }

    pub fn property(mut self, property: Property) -> Self {
        self.properties.push(property);

        self
    }

    pub fn method(mut self, method: Method) -> Self {
        self.methods.push(method);

        self
    }
}

impl Generator for Trait {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(documentation) = &self.documentation {
            code.push_str(&documentation.generate(indentation, level));
        }

        for attribute in &self.attributes {
            code.push_str(&attribute.generate(indentation, level));
        }

        code.push_str(&format!("trait {}", self.name));

        code.push_str("\n{\n");

        if !self.usages.is_empty() {
            code.push_str(
                &self
                    .usages
                    .iter()
                    .map(|usage| usage.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push('\n');
        }

        if !self.constants.is_empty() {
            code.push_str(
                &self
                    .constants
                    .iter()
                    .map(|constant| constant.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push('\n');
        }

        if !self.properties.is_empty() {
            code.push_str(
                &self
                    .properties
                    .iter()
                    .map(|property| property.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push('\n');
        }

        if !self.methods.is_empty() {
            code.push_str(
                &self
                    .methods
                    .iter()
                    .map(|method| method.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );
        }

        code.push_str("}\n");

        code
    }
}
