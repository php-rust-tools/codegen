pub mod attribute;
pub mod class;
pub mod comment;
pub mod constant;
pub mod data_type;
pub mod file;
pub mod function;
pub mod literal;
pub mod method;
pub mod modifiers;
pub mod parameter;
pub mod property;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub enum Indentation {
    Spaces(usize),
    Tabs(usize),
}

impl Indentation {
    pub fn value(&self, level: usize) -> String {
        self.to_string().repeat(level)
    }

    pub fn indent<T: ToString>(&self, code: T, level: usize) -> String {
        let indentation = self.value(level);

        code.to_string()
            .lines()
            .map(|line| format!("{}{}", indentation, line.trim_end()))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl std::fmt::Display for Indentation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Indentation::Spaces(count) => write!(f, "{:1$}", " ", count),
            Indentation::Tabs(count) => write!(f, "{:1$}", "\t", count),
        }
    }
}

pub trait Generator {
    fn generate(&self, indentation: Indentation, level: usize) -> String;
}
