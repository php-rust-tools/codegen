use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct AttributeGroup {
    pub members: Vec<(String, Option<String>)>,
}

impl AttributeGroup {
    pub fn new() -> Self {
        Self { members: vec![] }
    }

    pub fn add<T: ToString>(mut self, name: T, arguments: Option<T>) -> Self {
        self.members.push((
            name.to_string(),
            arguments.map(|arguments| arguments.to_string()),
        ));

        self
    }
}

impl Generator for AttributeGroup {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut result = String::new();

        result.push_str(&indentation.indent("#[", level));
        result.push_str(
            &self
                .members
                .iter()
                .map(|(name, arguments)| {
                    if let Some(arguments) = arguments {
                        format!("{}({})", name, arguments)
                    } else {
                        name.to_string()
                    }
                })
                .collect::<Vec<String>>()
                .join(", "),
        );
        result.push_str("]\n");

        result
    }
}

impl Default for AttributeGroup {
    fn default() -> Self {
        Self::new()
    }
}
