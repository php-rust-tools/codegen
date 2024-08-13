use crate::attribute::AttributeGroup;
use crate::body::Body;
use crate::comment::Document;
use crate::data_type::DataType;
use crate::literal::Value;
use crate::modifiers::Modifier;
use crate::modifiers::VisibilityModifier;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct PropertySetHookParameter {
    pub data_type: Option<DataType>,
    pub name: String,
}

#[derive(Debug)]
pub enum PropertyHook {
    Get(bool, Body),
    Set(Option<PropertySetHookParameter>, Body),
}

#[derive(Debug)]
pub struct Property {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub visibility: Option<VisibilityModifier>,
    pub modifiers: Vec<Modifier>,
    pub data_type: Option<DataType>,
    pub name: String,
    pub default: Option<Value>,
    pub hooks: Vec<PropertyHook>,
}

impl PropertySetHookParameter {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            data_type: None,
        }
    }

    pub fn typed(mut self, data_type: DataType) -> Self {
        self.data_type = Some(data_type);

        self
    }
}

impl Property {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            data_type: None,
            default: None,
            modifiers: vec![],
            attributes: vec![],
            visibility: None,
            documentation: None,
            hooks: vec![],
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

    pub fn typed(mut self, data_type: DataType) -> Self {
        self.data_type = Some(data_type);

        self
    }

    pub fn default<T: Into<Value>>(mut self, default: T) -> Self {
        self.default = Some(default.into());

        self
    }

    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers.push(modifier);

        self
    }

    pub fn public(mut self) -> Self {
        self.visibility = Some(VisibilityModifier::Public);

        self
    }

    pub fn protected(mut self) -> Self {
        self.visibility = Some(VisibilityModifier::Protected);

        self
    }

    pub fn private(mut self) -> Self {
        self.visibility = Some(VisibilityModifier::Private);

        self
    }

    pub fn visibility(mut self, visibility: VisibilityModifier) -> Self {
        self.visibility = Some(visibility);

        self
    }

    pub fn hook(mut self, hook: PropertyHook) -> Self {
        self.hooks.push(hook);

        self
    }
}

impl Generator for PropertySetHookParameter {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if let Some(data_type) = &self.data_type {
            code.push_str(&format!("{} ", data_type.generate(indentation, level)));
        }

        code.push_str(&self.name);

        code
    }
}

impl Generator for PropertyHook {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        match self {
            PropertyHook::Get(by_reference, body) => {
                let mut code = String::new();

                code.push_str(&indentation.value(level + 1));
                if *by_reference {
                    code.push_str("&get");
                } else {
                    code.push_str("get");
                }

                code.push_str(&body.generate(indentation, level + 1));

                code
            }
            PropertyHook::Set(parameter, body) => {
                let mut code = String::new();

                code.push_str(&indentation.value(level + 1));
                code.push_str("set");

                if let Some(parameter) = parameter {
                    code.push_str(" (");
                    code.push_str(&parameter.generate(indentation, level + 1));
                    code.push(')');
                }

                code.push_str(&body.generate(indentation, level + 1));

                code
            }
        }
    }
}

impl Generator for Vec<PropertyHook> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        if self.is_empty() {
            return String::from(";");
        }

        let hooks = self
            .iter()
            .map(|hook| hook.generate(indentation, level))
            .collect::<Vec<String>>()
            .join("");

        format!(" {{\n{}{}}}", hooks, indentation.value(level))
    }
}

impl Generator for Property {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = indentation.value(level);

        if let Some(document) = &self.documentation {
            code.push_str(&document.generate(indentation, level));
        }

        if !self.attributes.is_empty() {
            code.push_str(
                &self
                    .attributes
                    .iter()
                    .map(|attributes| attributes.generate(indentation, level))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push('\n');
        }

        if let Some(visibility) = &self.visibility {
            code.push_str(&format!("{} ", visibility.generate(indentation, level)));
        }

        for modifier in &self.modifiers {
            code.push_str(&format!("{} ", modifier.generate(indentation, level)));
        }

        if let Some(data_type) = &self.data_type {
            code.push_str(&format!("{} ", data_type.generate(indentation, level)));
        }

        code.push_str(&format!("${}", &self.name));

        if let Some(default) = &self.default {
            code.push_str(&format!(" = {}", default.generate(indentation, level)));
        }

        code.push_str(&self.hooks.generate(indentation, level));

        code
    }
}

impl Generator for Vec<Property> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();
        if self.is_empty() {
            return code;
        }

        for property in self.iter() {
            code.push_str(property.generate(indentation, level).as_str());
            code.push('\n');
        }

        code
    }
}
