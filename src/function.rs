use crate::attribute::AttributeGroup;
use crate::body::Body;
use crate::comment::Document;
use crate::data_type::DataType;
use crate::parameter::Parameter;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct Function {
    pub documentation: Option<Document>,
    pub attributes: Vec<AttributeGroup>,
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<DataType>,
    pub body: Body,
}

impl Function {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            parameters: vec![],
            return_type: None,
            body: Body::new().with_semicolon_for_empty(false),
            attributes: vec![],
            documentation: None,
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

    pub fn parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);

        self
    }

    pub fn returns(mut self, return_type: DataType) -> Self {
        self.return_type = Some(return_type);

        self
    }

    pub fn body<T: Into<Body>>(mut self, body: T) -> Self {
        self.body = body.into();
        self.body = self.body.with_semicolon_for_empty(false);

        self
    }
}

impl Generator for Function {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

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

        code.push_str(format!("function {}", self.name).as_str());

        if self.parameters.is_empty() {
            code.push_str("()");
        } else {
            code.push_str("(\n");
            code.push_str(
                &self
                    .parameters
                    .iter()
                    .map(|parameter| parameter.generate(indentation, level + 1))
                    .collect::<Vec<String>>()
                    .join(",\n"),
            );

            code.push_str(",\n");
            code.push_str(")");
        }

        if let Some(return_type) = &self.return_type {
            code.push_str(format!(": {}", return_type.generate(indentation, level)).as_str());
        }

        code.push_str(&self.body.generate(indentation, level));

        code
    }
}
