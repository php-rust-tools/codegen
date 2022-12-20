use crate::Generator;
use crate::Indentation;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Modifier {
    Abstract,
    Final,
    Readonly,
    Static,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum VisibilityModifier {
    Public,
    Protected,
    Private,
}

impl Generator for Modifier {
    fn generate(&self, _: Indentation, _: usize) -> String {
        match self {
            Modifier::Abstract => "abstract".to_string(),
            Modifier::Final => "final".to_string(),
            Modifier::Readonly => "readonly".to_string(),
            Modifier::Static => "static".to_string(),
        }
    }
}

impl Generator for VisibilityModifier {
    fn generate(&self, _: Indentation, _: usize) -> String {
        match self {
            VisibilityModifier::Public => "public".to_string(),
            VisibilityModifier::Protected => "protected".to_string(),
            VisibilityModifier::Private => "private".to_string(),
        }
    }
}
