use std::fmt::Display;

use crate::class::Class;
use crate::constant::Constant;
use crate::function::Function;
use crate::literal::Value;
use crate::Generator;
use crate::Indentation;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct File {
    pub namespace: Option<String>,
    pub declares: Vec<(String, Value)>,
    pub uses: Vec<String>,
    pub constant_uses: Vec<String>,
    pub function_uses: Vec<String>,
    pub functions: Vec<Function>,
    pub constants: Vec<Constant>,
    pub classes: Vec<Class>,
}

impl File {
    pub fn new() -> Self {
        Self {
            namespace: None,
            declares: vec![],
            uses: vec![],
            constant_uses: vec![],
            function_uses: vec![],
            constants: vec![],
            functions: vec![],
            classes: vec![],
        }
    }

    pub fn declare<T: ToString, Tv: Into<Value>>(mut self, name: T, value: Tv) -> Self {
        self.declares.push((name.to_string(), value.into()));

        self
    }

    pub fn namespaced<T: ToString>(mut self, namespace: T) -> Self {
        self.namespace = Some(namespace.to_string());

        self
    }

    pub fn uses<T: ToString>(mut self, symbol: T) -> Self {
        self.uses.push(symbol.to_string());

        self
    }

    pub fn uses_constant<T: ToString>(mut self, constant: T) -> Self {
        self.constant_uses.push(constant.to_string());

        self
    }

    pub fn uses_function<T: ToString>(mut self, function: T) -> Self {
        self.function_uses.push(function.to_string());

        self
    }

    pub fn constant<T: Into<Constant>>(mut self, constant: T) -> Self {
        self.constants.push(constant.into());

        self
    }

    pub fn function(mut self, function: Function) -> Self {
        self.functions.push(function);

        self
    }

    pub fn class(mut self, class: Class) -> Self {
        self.classes.push(class.into());

        self
    }
}

impl Generator for File {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        code.push_str("<?php\n\n");

        for (name, value) in &self.declares {
            code.push_str(&format!(
                "declare({}={});\n\n",
                name,
                value.generate(indentation, level)
            ));
        }

        if let Some(namespace) = &self.namespace {
            code.push_str(&format!("namespace {};\n\n", namespace));
        }

        let mut used = false;
        if !self.uses.is_empty() {
            used = true;
            for r#use in &self.uses {
                code.push_str(&format!("use {};\n", r#use));
            }

            code.push_str("\n");
        }

        if !self.function_uses.is_empty() {
            used = true;
            for function in &self.function_uses {
                code.push_str(&format!("use function {};\n", function));
            }

            code.push_str("\n");
        }

        if !self.constant_uses.is_empty() {
            used = true;
            for constant in &self.constant_uses {
                code.push_str(&format!("use const {};\n", constant));
            }

            code.push_str("\n");
        }

        if used {
            code.push_str("\n");
        }

        if !self.constants.is_empty() {
            for constant in &self.constants {
                code.push_str(&constant.generate(indentation, level));
            }

            code.push_str("\n");
        }

        for function in &self.functions {
            code.push_str(&function.generate(indentation, level));
            code.push_str("\n");
        }

        for class in &self.classes {
            code.push_str(&class.generate(indentation, level));
            code.push_str("\n");
        }

        code
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.generate(Indentation::Spaces(4), 0))
    }
}
