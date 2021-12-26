use indexmap::IndexMap;
use openapiv3::{SchemaData, Type};

pub enum REQUIRED<'a> {
    REQUIRED(&'a ModelProperty),
}

#[derive(Debug)]
pub struct Model {
    pub name: String,
    pub data: SchemaData,
    pub properties: IndexMap<String, ModelProperty>,
}

#[derive(Debug)]
pub struct ModelProperty {
    pub name: String,
    pub data: SchemaData,
    pub prop_type: Type,
}
