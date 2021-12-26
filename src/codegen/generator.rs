use indexmap::IndexSet;
use log::warn;
use openapiv3::Type;

use super::model::{Model, ModelProperty};

fn serialize_type(prop_type: &Type) -> String {
    let prop_type = match prop_type {
        Type::Number(_) => "number",
        Type::String(_) => "string",
        Type::Boolean {} => "boolean",
        // TODO: Recursive Array implementation
        _ => {
            warn!(
                "Serialization of type {:?} is not supported yet, collapsing to unknown",
                prop_type
            );
            "unknown"
        }
    };

    prop_type.to_string()
}

fn serialize_property(prop: &ModelProperty, is_required: bool) -> String {
    let ModelProperty {
        name,
        data: _, // TODO: Advanced Validation
        prop_type,
    } = prop;
    let literal_type = serialize_type(prop_type);

    let name = name.clone();

    let name = if name.contains(' ') {
        format!("\"{}\"", name)
    } else {
        name
    };

    format!(
        "{}{optional_op}: {}",
        name,
        literal_type,
        optional_op = if is_required { "" } else { "?" }
    )
}

pub struct InterfaceGenerator {
    pub name: String,
    properties: IndexSet<String>,
}

impl InterfaceGenerator {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            properties: IndexSet::new(),
        }
    }

    pub fn from(model: &Model) -> InterfaceGenerator {
        let mut generator = InterfaceGenerator::new(&model.name);
        for (_, prop_type) in &model.properties {
            generator.register_property(prop_type);
        }

        generator
    }

    pub fn register_property(&mut self, prop: &ModelProperty) {
        self.properties.insert(serialize_property(prop, true));
    }

    pub fn build(&self) -> String {
        let props_literal = self
            .properties
            .iter()
            .fold(String::new(), |sum, prop| format!("{}\n\t{}", sum, prop));

        format!(
            "interface {name} {{{props}\n}}",
            name = self.name,
            props = props_literal
        )
    }
}

#[cfg(test)]
mod tests {
    use openapiv3::{NumberType, SchemaData, StringType, Type};

    use crate::codegen::{
        generator::{serialize_property, serialize_type},
        model::ModelProperty,
    };

    #[test]
    fn it_serializes_numbers() {
        let to_serialize = Type::Number(NumberType {
            ..Default::default()
        });

        assert_eq!(serialize_type(&to_serialize), "number");
    }

    #[test]
    fn it_serializes_strings() {
        let to_serialize = Type::String(StringType {
            ..Default::default()
        });

        assert_eq!(serialize_type(&to_serialize), "string");
    }

    #[test]
    fn it_serializes_booleans() {
        let to_serialize = Type::Boolean {};

        assert_eq!(serialize_type(&to_serialize), "boolean");
    }

    #[test]
    fn it_serializes_required_prop() {
        let schema_data = SchemaData {
            ..Default::default()
        };
        let model_prop = ModelProperty {
            name: "testprop".to_string(),
            data: schema_data,
            prop_type: Type::Boolean {},
        };

        assert_eq!(serialize_property(&model_prop, true), "testprop: boolean");
    }

    #[test]
    fn it_serializes_optional_prop() {
        let schema_data = SchemaData {
            ..Default::default()
        };
        let model_prop = ModelProperty {
            name: "testprop".to_string(),
            data: schema_data,
            prop_type: Type::Boolean {},
        };

        assert_eq!(serialize_property(&model_prop, false), "testprop?: boolean");
    }
}
