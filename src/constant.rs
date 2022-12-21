use crate::attribute::AttributeGroup;
use crate::comment::Document;
use crate::literal::Value;
use crate::modifiers::Modifier;
use crate::modifiers::VisibilityModifier;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct Constant {
    pub documentation: Option<Document>,
    pub name: String,
    pub value: Value,
}

impl Constant {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            name: name.to_string(),
            value: Value::Null,
        }
    }

    pub fn documented<T: ToString>(mut self, documentation: Document) -> Self {
        self.documentation = Some(documentation);

        self
    }

    pub fn valued<T: Into<Value>>(mut self, value: T) -> Self {
        self.value = value.into();

        self
    }
}

impl Generator for Constant {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut output = String::new();

        if let Some(documentation) = &self.documentation {
            output.push_str(&documentation.generate(indentation, level));
        }

        output.push_str(&format!(
            "const {} = {};\n",
            self.name,
            self.value.generate(indentation, level)
        ));

        output
    }
}

impl<T: ToString, Tv: Into<Value>> From<(T, Tv)> for Constant {
    fn from((name, value): (T, Tv)) -> Self {
        Self {
            documentation: None,
            name: name.to_string(),
            value: value.into(),
        }
    }
}

#[derive(Debug)]
pub struct ClassConstant {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub visibility: Option<VisibilityModifier>,
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub value: Value,
}

impl ClassConstant {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            visibility: None,
            modifiers: vec![],
            name: name.to_string(),
            value: Value::Null,
        }
    }

    pub fn documented<T: ToString>(mut self, documentation: Document) -> Self {
        self.documentation = Some(documentation);

        self
    }

    pub fn attributes(mut self, attributes: AttributeGroup) -> Self {
        self.attributes.push(attributes);

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

    pub fn modifier(mut self, modifier: Modifier) -> Self {
        self.modifiers.push(modifier);

        self
    }

    pub fn valued<T: Into<Value>>(mut self, value: T) -> Self {
        self.value = value.into();

        self
    }
}

impl Generator for ClassConstant {
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

            code.push_str("\n");
        }

        if let Some(visibility) = &self.visibility {
            code.push_str(&format!("{} ", visibility.generate(indentation, level)));
        }

        for modifier in &self.modifiers {
            code.push_str(&format!("{} ", modifier.generate(indentation, level)));
        }

        code.push_str(&format!(
            "const {} = {};\n",
            self.name,
            self.value.generate(indentation, level)
        ));

        code
    }
}

impl<T: ToString, Tv: Into<Value>> From<(T, Tv)> for ClassConstant {
    fn from((name, value): (T, Tv)) -> Self {
        Self {
            documentation: None,
            attributes: vec![],
            visibility: None,
            modifiers: vec![],
            name: name.to_string(),
            value: value.into(),
        }
    }
}
