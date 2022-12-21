use crate::attribute::AttributeGroup;
use crate::data_type::DataType;
use crate::literal::Value;
use crate::modifiers::Modifier;
use crate::modifiers::VisibilityModifier;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct Parameter {
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub data_type: Option<DataType>,
    pub default: Option<Value>,
    pub modifiers: Vec<Modifier>,
    pub visibility: Option<VisibilityModifier>,
    pub variadic: bool,
}

impl Parameter {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            data_type: None,
            default: None,
            modifiers: vec![],
            attributes: vec![],
            visibility: None,
            variadic: false,
        }
    }

    pub fn attributes(mut self, attributes: AttributeGroup) -> Self {
        self.attributes.push(attributes);

        self
    }

    pub fn typed(mut self, data_type: DataType) -> Self {
        self.data_type = Some(data_type);

        self
    }

    pub fn variadic(mut self) -> Self {
        self.variadic = true;

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
}

impl Generator for Parameter {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        if !self.attributes.is_empty() {
            code.push_str(
                &self
                    .attributes
                    .iter()
                    .map(|attributes| attributes.generate(indentation, level))
                    .collect::<Vec<String>>()
                    .join("\n"),
            );

            code.push_str("\n");
            code.push_str(&indentation.value(level));
        } else {
            code.push_str(&indentation.value(level));
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

        if self.variadic {
            code.push_str("...");
        }

        code.push_str(&format!("${}", &self.name));

        if let Some(default) = &self.default {
            code.push_str(&format!(" = {}", default.generate(indentation, level)));
        }

        code
    }
}
