use core::fmt::Debug;

use crate::Generator;
use crate::Indentation;

pub struct Body {
    pub factory: Option<Box<dyn Fn(Indentation, usize) -> String>>,
    pub semicolon_for_empty: bool,
}

impl Body {
    pub fn new() -> Self {
        Self {
            factory: None,
            semicolon_for_empty: true,
        }
    }

    pub fn with_factory<T: Fn(Indentation, usize) -> String + 'static>(factory: T) -> Self {
        Self {
            factory: Some(Box::new(factory)),
            semicolon_for_empty: true,
        }
    }

    pub fn with_semicolon_for_empty(mut self, semicolon_for_empty: bool) -> Self {
        self.semicolon_for_empty = semicolon_for_empty;

        self
    }
}

impl Debug for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Body")
            .field("factory:is-some", &self.factory.is_some())
            .field("semicolon_for_empty", &self.semicolon_for_empty)
            .finish()
    }
}

impl Generator for Body {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        match self.factory {
            Some(ref factory) => {
                let mut code = String::new();

                code.push_str(" {");
                code.push_str("\n");
                code.push_str(&factory(indentation, level + 1));
                code.push_str("\n");
                code.push_str(&indentation.indent("}", level));
                code.push_str("\n");

                code
            }
            None => {
                let mut code = String::new();

                if self.semicolon_for_empty {
                    code.push_str(";");
                } else {
                    code.push_str(" {");
                    code.push_str("\n");
                    code.push_str(&indentation.indent("// empty body", level + 1));
                    code.push_str("\n");
                    code.push_str(&indentation.indent("}", level));
                }

                code.push_str("\n");

                code
            }
        }
    }
}

impl From<String> for Body {
    fn from(body: String) -> Self {
        Self {
            factory: Some(Box::new(move |indentation, level| {
                let body = body.clone();

                indentation.indent(body, level)
            })),
            semicolon_for_empty: true,
        }
    }
}

impl From<&str> for Body {
    fn from(body: &str) -> Self {
        body.to_string().into()
    }
}

impl<T: Fn(Indentation, usize) -> String + 'static> From<T> for Body {
    fn from(factory: T) -> Self {
        Self {
            factory: Some(Box::new(factory)),
            semicolon_for_empty: true,
        }
    }
}

impl From<Option<Box<dyn Fn(Indentation, usize) -> String>>> for Body {
    fn from(factory: Option<Box<dyn Fn(Indentation, usize) -> String>>) -> Self {
        Self {
            factory,
            semicolon_for_empty: true,
        }
    }
}

impl From<Option<String>> for Body {
    fn from(body: Option<String>) -> Self {
        match body {
            Some(body) => body.into(),
            None => Self {
                factory: None,
                semicolon_for_empty: true,
            },
        }
    }
}

impl Default for Body {
    fn default() -> Self {
        Self {
            factory: None,
            semicolon_for_empty: true,
        }
    }
}
