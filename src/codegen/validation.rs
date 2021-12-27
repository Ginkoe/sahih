use indexmap::IndexMap;
use log::debug;
use openapiv3::Type;

use super::model::{Model, ModelProperty};

#[derive(Debug)]
pub enum ValidatorLib {
    Yup,
}

#[derive(Debug)]
pub enum NumberRules {
    Min(f64),
    Max(f64),
}

impl NumberRules {
    pub fn build(&self) -> String {
        match self {
            Self::Min(value) => format!(".min({})", value),
            Self::Max(value) => format!(".max({})", value),
        }
    }
}

#[derive(Debug)]
pub enum StringRules {
    Min(usize),
    Max(usize),
    Matches(String),
    OneOf(Vec<String>),
    Email,
    Uuid,
}

impl StringRules {
    pub fn build(&self) -> String {
        match self {
            Self::Min(value) => format!(".min({})", value),
            Self::Max(value) => format!(".max({})", value),
            Self::Email => ".email()".to_string(),
            Self::Matches(regex) => format!(".matches(/{}/)", regex),
            Self::Uuid => ".uuid()".to_string(),
            Self::OneOf(enumerate) => {
                let quoted: Vec<String> = enumerate.iter().map(|e| format!("`{}`", e)).collect();
                quoted.join(",")
            }
        }
    }
}

/*
    TODO: Replace with Rule struct when iter implemented
*/
#[derive(Debug)]
pub enum PropRules {
    String(Vec<StringRules>),
    Number(Vec<NumberRules>),
    Unsupported,
}

#[derive(Debug)]
pub struct ValidationGenerator {
    lib: ValidatorLib,
    pub name: String,
    pub properties: IndexMap<String, PropRules>,
}

impl ValidationGenerator {
    pub fn new(name: &str, lib: ValidatorLib) -> Self {
        Self {
            name: name.to_owned(),
            lib,
            properties: IndexMap::new(),
        }
    }

    pub fn register_property(&mut self, prop: &ModelProperty) {
        let prop_rule = match &prop.prop_type {
            Type::Number(number_type) => {
                let mut rules = Vec::<NumberRules>::new();

                if let Some(min) = number_type.minimum {
                    rules.push(NumberRules::Min(min));
                }

                if let Some(max) = number_type.maximum {
                    rules.push(NumberRules::Max(max));
                }

                debug!("Number rules : {:#?}", rules);
                PropRules::Number(rules)
            }

            Type::String(string_rules) => {
                let mut rules = Vec::<StringRules>::new();

                if let Some(min) = string_rules.min_length {
                    rules.push(StringRules::Min(min));
                }

                if let Some(max) = string_rules.max_length {
                    rules.push(StringRules::Max(max));
                }

                if let Some(pattern) = &string_rules.pattern {
                    rules.push(StringRules::Matches(pattern.clone()));
                }

                if !string_rules.enumeration.is_empty() {
                    rules.push(StringRules::OneOf(string_rules.enumeration.clone()));
                }

                PropRules::String(rules)
            }
            _ => PropRules::Unsupported,
        };

        self.properties.insert(prop.name.clone(), prop_rule);
    }

    pub fn from(model: &Model, lib: ValidatorLib) -> Self {
        let mut generator = ValidationGenerator::new(&model.name, lib);
        for (_, prop_type) in &model.properties {
            generator.register_property(prop_type);
        }

        generator
    }
}

#[cfg(test)]
mod tests {}
