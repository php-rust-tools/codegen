use crate::Generator;
use crate::Indentation;

#[derive(Debug)]
pub enum Element {
    Tag(String, String),
    Text(String),
    EmptyLine,
}

#[derive(Debug)]
pub struct Document {
    pub elements: Vec<Element>,
}

impl Document {
    pub fn new() -> Self {
        Self { elements: vec![] }
    }

    pub fn add_element(mut self, element: Element) -> Self {
        self.elements.push(element);

        self
    }

    pub fn empty_line(mut self) -> Self {
        self.elements.push(Element::EmptyLine);

        self
    }

    pub fn tag<T: ToString>(mut self, tag: T, description: T) -> Self {
        self.elements
            .push(Element::Tag(tag.to_string(), description.to_string()));

        self
    }

    pub fn simple_tag<T: ToString>(mut self, tag: T) -> Self {
        self.elements
            .push(Element::Tag(tag.to_string(), String::new()));

        self
    }

    pub fn text<T: ToString>(mut self, text: T) -> Self {
        self.elements
            .push(Element::Text(format!("{}\n", text.to_string())));

        self
    }
}

impl Generator for Document {
    fn generate(&self, indentation: Indentation, level: usize) -> String {
        let mut code = String::new();

        code.push_str(&format!("{}/**\n", indentation.value(level)));

        for element in &self.elements {
            let element = element.generate(indentation, level);
            if element.is_empty() {
                code.push_str(&format!("{} *\n", indentation.value(level)));

                continue;
            }

            for line in element.lines() {
                code.push_str(&format!(
                    "{} * {}\n",
                    indentation.value(level),
                    line.trim_end()
                ));
            }
        }

        code.push_str(&format!("{} */\n", indentation.value(level)));

        code
    }
}

impl Generator for Element {
    fn generate(&self, _: Indentation, _: usize) -> String {
        match self {
            Element::Tag(tag, description) => format!("@{} {}", tag, description),
            Element::Text(text) => text.to_string(),
            Element::EmptyLine => String::new(),
        }
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}
