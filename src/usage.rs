use crate::modifiers::VisibilityModifier;
use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub struct Usage {
    pub traits: Vec<String>,
    pub adaptations: Vec<UsageAdaptation>,
}

#[derive(Debug)]
pub enum UsageAdaptation {
    Alias {
        method: String,
        alias: String,
        visibility: Option<VisibilityModifier>,
    },
    Visibility {
        method: String,
        visibility: VisibilityModifier,
    },
    Precedence {
        method: String,
        insteadof: Vec<String>,
    },
}

impl Usage {
    pub fn new(traits: Vec<String>) -> Self {
        Self {
            traits,
            adaptations: vec![],
        }
    }

    pub fn with<T: ToString>(mut self, r#trait: T) -> Self {
        self.traits.push(r#trait.to_string());

        self
    }

    pub fn alias<T: ToString>(
        mut self,
        method: T,
        alias: T,
        visibility: VisibilityModifier,
    ) -> Self {
        self.adaptations.push(UsageAdaptation::Alias {
            method: method.to_string(),
            alias: alias.to_string(),
            visibility: Some(visibility),
        });

        self
    }

    pub fn rename<T: ToString>(mut self, method: T, alias: T) -> Self {
        self.adaptations.push(UsageAdaptation::Alias {
            method: method.to_string(),
            alias: alias.to_string(),
            visibility: None,
        });

        self
    }

    pub fn public<T: ToString>(mut self, method: T) -> Self {
        self.adaptations.push(UsageAdaptation::Visibility {
            method: method.to_string(),
            visibility: VisibilityModifier::Public,
        });

        self
    }

    pub fn protected<T: ToString>(mut self, method: T) -> Self {
        self.adaptations.push(UsageAdaptation::Visibility {
            method: method.to_string(),
            visibility: VisibilityModifier::Protected,
        });

        self
    }

    pub fn private<T: ToString>(mut self, method: T) -> Self {
        self.adaptations.push(UsageAdaptation::Visibility {
            method: method.to_string(),
            visibility: VisibilityModifier::Private,
        });

        self
    }

    pub fn visibility<T: ToString>(mut self, method: T, visibility: VisibilityModifier) -> Self {
        self.adaptations.push(UsageAdaptation::Visibility {
            method: method.to_string(),
            visibility,
        });

        self
    }

    pub fn precede<T: ToString>(mut self, method: T, insteadof: Vec<T>) -> Self {
        self.adaptations.push(UsageAdaptation::Precedence {
            method: method.to_string(),
            insteadof: insteadof
                .into_iter()
                .map(|insteadof| insteadof.to_string())
                .collect(),
        });

        self
    }
}

impl Generator for Usage {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = indentation.indent("use", level);

        code.push(' ');
        code.push_str(&self.traits.join(", "));

        if !self.adaptations.is_empty() {
            code.push_str(" {\n");
            for adaptation in &self.adaptations {
                let adaptation = match adaptation {
                    UsageAdaptation::Alias {
                        method,
                        alias,
                        visibility,
                    } => {
                        let alias = if let Some(visibility) = visibility {
                            format!("{} {}", visibility.generate(indentation, level), alias)
                        } else {
                            alias.to_string()
                        };

                        indentation.indent(format!("{} as {}", method, alias), level + 1)
                    }
                    UsageAdaptation::Visibility { method, visibility } => indentation.indent(
                        format!("{} as {}", method, visibility.generate(indentation, level)),
                        level + 1,
                    ),
                    UsageAdaptation::Precedence { method, insteadof } => indentation.indent(
                        format!("{} insteadof {}", method, insteadof.join(", ")),
                        level + 1,
                    ),
                };

                code.push_str(&format!("{};\n", adaptation));
            }

            code.push_str(&format!("{}}}\n", indentation.value(level)));
        } else {
            code.push_str(";\n");
        }

        code
    }
}

impl Generator for Vec<Usage> {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();
        if self.is_empty() {
            return code;
        }

        for usage in self {
            code.push_str(usage.generate(indentation, level).as_str());
        }

        code.push('\n');

        code
    }
}

impl From<String> for Usage {
    fn from(r#trait: String) -> Self {
        Self::new(vec![r#trait])
    }
}

impl From<&str> for Usage {
    fn from(r#trait: &str) -> Self {
        Self::new(vec![r#trait.to_string()])
    }
}

impl<T: ToString> From<Vec<T>> for Usage {
    fn from(traits: Vec<T>) -> Self {
        traits
            .into_iter()
            .fold(Self::new(vec![]), |usage, r#trait| usage.with(r#trait))
    }
}
