use indexmap::IndexMap;
use log::{debug, warn};
use openapiv3::{ObjectType, OpenAPI, ReferenceOr, Schema, SchemaData, SchemaKind, Type};

use crate::codegen::model::{Model, ModelProperty};

// fn parse_schema_object() {
//     let mut fields = IndexMap::new();
// }

fn unwrap_type(schema: SchemaKind) -> Option<Type> {
    if let SchemaKind::Type(base_schema) = schema {
        Some(base_schema)
    } else {
        None
    }
}

fn extract_object_type(schema: SchemaKind) -> Option<ObjectType> {
    let base_type = unwrap_type(schema).expect("Type isn't an Object");

    match base_type {
        Type::Object(object) => Some(object),
        _ => {
            warn!("Primitive Types are not supported yet");
            None
        }
    }
}

fn extract_stack_type(schema: ReferenceOr<Schema>) -> Option<(SchemaData, SchemaKind)> {
    debug!("{:#?}", schema);
    match schema {
        ReferenceOr::Item(item) => {
            let Schema {
                schema_data,
                schema_kind,
            } = item;
            Some((schema_data, schema_kind))
        }
        _ => {
            warn!("Schema references are not suppoted yet");
            None
        }
    }
}

fn extract_heap_type(schema: ReferenceOr<Box<Schema>>) -> Option<(SchemaData, SchemaKind)> {
    match schema {
        ReferenceOr::Item(heap_item) => {
            let item = *heap_item;
            let Schema {
                schema_data,
                schema_kind,
            } = item;
            Some((schema_data, schema_kind))
        }
        _ => {
            warn!("Schema references are not suppoted yet");
            None
        }
    }
}

pub fn consume_schemas(path: &str) -> Vec<Model> {
    let schema_buffer = std::fs::read_to_string(path).expect("Could not find user");
    let openapi: OpenAPI = serde_json::from_str(schema_buffer.as_str()).unwrap();

    let components = openapi.components.unwrap();
    let schemas = components.schemas;

    let mut unwrapped_schemas: Vec<Model> = vec![];

    for schematype in schemas {
        let (model_name, model_data) = schematype;

        let (schema_data, schema_kind) = if let Some(item) = extract_stack_type(model_data) {
            item
        } else {
            continue;
        };

        let schema_object = if let Some(object) = extract_object_type(schema_kind) {
            object
        } else {
            continue;
        };

        let mut model = Model {
            name: model_name,
            data: schema_data,
            properties: IndexMap::new(),
        };

        for (prop_name, prop_schema) in schema_object.properties {
            let (schema_data, schema_kind) = if let Some(item) = extract_heap_type(prop_schema) {
                item
            } else {
                continue;
            };

            let prop_type = match unwrap_type(schema_kind) {
                Some(prop_type) => prop_type,
                None => {
                    continue;
                }
            };

            let model_prop = ModelProperty {
                name: prop_name,
                data: schema_data,
                prop_type,
            };

            model
                .properties
                .insert(model_prop.name.to_owned(), model_prop);
        }

        unwrapped_schemas.push(model);
    }

    unwrapped_schemas
}
