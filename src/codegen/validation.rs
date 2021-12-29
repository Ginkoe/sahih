use indexmap::IndexMap;
use log::debug;
use openapiv3::Type;

use super::model::{Model, ModelProperty};

trait BuildableRule {
    fn build(&self) -> String;
}

fn collect_rules<B: BuildableRule>(prefix: &str, rules: &[B]) -> String {
    let collected_rules: String = rules.iter().map(|rule| rule.build()).collect();
    format!("{}{}", prefix, collected_rules)
}

#[derive(Debug)]
pub enum ValidatorLib {
    Yup,
}

#[derive(Debug)]
pub enum NumberRules {
    Min(f64),
    Max(f64),
}

impl BuildableRule for NumberRules {
    fn build(&self) -> String {
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

impl BuildableRule for StringRules {
    fn build(&self) -> String {
        match self {
            Self::Min(value) => format!(".min({})", value),
            Self::Max(value) => format!(".max({})", value),
            Self::Email => ".email()".to_string(),
            /*
                TODO: Handle special chars escaping
            */
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

impl PropRules {
    /*
        TODO: Refactor ProprRules with build trait
    */
    pub fn build(&self) -> String {
        let ser_rules = match self {
            PropRules::String(rules) => collect_rules(".string()", rules),
            PropRules::Number(rules) => collect_rules(".number()", rules),
            PropRules::Unsupported => String::from(".mixed()"),
        };

        format!("{}.required()", ser_rules)
    }
}

#[derive(Debug)]
pub struct ValidationGenerator {
    pub name: String,
    pub properties: IndexMap<String, PropRules>,
}

impl ValidationGenerator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
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

    pub fn from(model: &Model) -> Self {
        let mut generator = ValidationGenerator::new(&model.name);
        for (_, prop_type) in &model.properties {
            generator.register_property(prop_type);
        }

        generator
    }

    pub fn build(&self) -> String {
        let prop_shape: String = self
            .properties
            .iter()
            .map(|(prop_name, prop_rules)| {
                format!(
                    "{prop_name}: yup{prop_rules},\n",
                    prop_name = prop_name,
                    prop_rules = prop_rules.build()
                )
            })
            .collect();

        format!(
            "export const {model_name}Validator = yup.object().shape({{ {serialized_shape} }});\n",
            model_name = self.name,
            serialized_shape = prop_shape
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::StringRules;

    use super::{NumberRules, PropRules};

    #[test]
    fn it_builds_number_rules() {
        let rules = vec![NumberRules::Min(10.4), NumberRules::Max(40.0)];
        let prop_rules = PropRules::Number(rules);
        let built_rules = prop_rules.build();
        assert_eq!(built_rules, ".number().min(10.4).max(40).required()");
    }

    #[test]
    fn it_builds_string_match() {
        let rules = vec![StringRules::Matches("^[A-Z]{3}$".to_string())];
        let prop_rules = PropRules::String(rules);
        let built_rules = prop_rules.build();
        assert_eq!(built_rules, r#".string().matches(/^[A-Z]{3}$/).required()"#);
    }

    #[test]
    fn it_builds_string_len() {
        let rules = vec![StringRules::Min(8), StringRules::Max(128)];
        let prop_rules = PropRules::String(rules);
        let built_rules = prop_rules.build();
        assert_eq!(built_rules, ".string().min(8).max(128).required()");
    }

    #[test]
    fn it_builds_string_email() {
        let rules = vec![StringRules::Email];
        let prop_rules = PropRules::String(rules);
        let built_rules = prop_rules.build();
        assert_eq!(built_rules, ".string().email().required()");
    }

    #[test]
    fn it_builds_string_uuid() {
        let rules = vec![StringRules::Uuid];
        let prop_rules = PropRules::String(rules);
        let built_rules = prop_rules.build();
        assert_eq!(built_rules, ".string().uuid().required()");
    }
}
