use crate::attribute::AttributeGroup;
use crate::comment::Document;
use crate::constant::ClassConstant;
use crate::method::Method;
use crate::modifiers::Modifier;
use crate::property::Property;
use crate::Generator;
use crate::Indentation;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Class {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub constants: Vec<ClassConstant>,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
}

impl Class {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            modifiers: vec![],
            name: name.to_string(),
            extends: None,
            implements: vec![],
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

    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers.push(modifier);

        self
    }

    pub fn extend<T: ToString>(mut self, extends: T) -> Self {
        self.extends = Some(extends.to_string());

        self
    }

    pub fn implement<T: ToString>(mut self, implements: T) -> Self {
        self.implements.push(implements.to_string());

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

impl Generator for Class {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(documentation) = &self.documentation {
            code.push_str(&documentation.generate(indentation, level));
        }

        for attribute in &self.attributes {
            code.push_str(&attribute.generate(indentation, level));
        }

        for modifier in &self.modifiers {
            code.push_str(&format!("{} ", modifier.generate(indentation, level)));
        }

        code.push_str(&format!("class {}", self.name));

        if let Some(extends) = &self.extends {
            code.push_str(&format!(" extends {}", extends));
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

        if !self.constants.is_empty() {
            code.push_str(
                &self
                    .constants
                    .iter()
                    .map(|constant| constant.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push_str("\n");
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

            code.push_str("\n");
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
