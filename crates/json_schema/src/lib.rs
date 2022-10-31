use askama::Template;
use heck::{AsPascalCase, AsSnakeCase};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Template, Debug)]
#[template(path = "generator.j2")]
pub struct StructsTemplate {
    pub structs: Vec<Struct>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Schema {
    pub title: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
    pub properties: Option<HashMap<String, Schema>>,
}

impl Schema {
    pub fn into_structs(&self) -> Vec<Struct> {
        let mut structs = vec![];
        match self.r#type.as_str() {
            "object" => {
                let fields: Vec<_> = self
                    .properties
                    .as_ref()
                    .expect("object expect receive a value, but receive None")
                    .iter()
                    .map(|(name, schema)| process_type(&mut structs, name, schema))
                    .collect();
                structs.push(Struct::new(
                    as_pascal_case(
                        self.title
                            .as_ref()
                            .expect("'object' expect receive a `title`, but receive None"),
                    ),
                    self.description.clone(),
                    fields,
                ));
            }
            _ => panic!("Not implement"),
        }
        structs
    }
}

fn process_type(structs: &mut Vec<Struct>, name: &str, schema: &Schema) -> Field {
    let create_field = |r#type: &str| {
        Field::new(
            as_snake_case(name).as_str(),
            r#type,
            schema.description.clone(),
        )
    };

    match schema.r#type.as_str() {
        "object" => {
            let sts = schema.into_structs();
            structs.extend(sts);
            create_field(&as_pascal_case(schema.title.as_deref().unwrap_or(name)))
        }
        "integer" => create_field("i64"),
        "float" => create_field("f64"),
        "string" => create_field("String"),
        _ => panic!("Unsupported schema type"),
    }
}

fn as_pascal_case(s: &str) -> String {
    AsPascalCase(s).to_string()
}

fn as_snake_case(s: &str) -> String {
    AsSnakeCase(s).to_string()
}

#[derive(Debug, PartialEq, Eq)]
/// output struct
pub struct Struct {
    name: String,
    fields: Vec<Field>,
    description: Option<String>,
}

impl Struct {
    // TODO: Why here use impl Into<String>
    pub fn new(name: impl Into<String>, description: Option<String>, fields: Vec<Field>) -> Self {
        Self {
            name: name.into(),
            description,
            fields,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    name: String,
    r#type: String,
    description: Option<String>,
}

impl Field {
    pub fn new(
        name: impl Into<String>,
        r#type: impl Into<String>,
        description: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            r#type: r#type.into(),
            description,
        }
    }
}

mod tests {
    use super::*;
    #[allow(dead_code)]
    fn create_simple_schema() -> Schema {
        Schema {
            title: Some("person".to_owned()),
            r#type: "object".to_owned(),
            description: None,
            properties: Some(HashMap::from([
                (
                    "firstName".to_string(),
                    Schema {
                        title: None,
                        r#type: "string".to_owned(),
                        description: Some("first name".to_owned()),
                        properties: None,
                    },
                ),
                (
                    "lastName".to_string(),
                    Schema {
                        title: None,
                        r#type: "string".to_owned(),
                        description: Some("last name".to_owned()),
                        properties: None,
                    },
                ),
            ])),
        }
    }

    #[test]
    fn test_as_pascal_case() {
        let s = "hello_world";
        assert_eq!(as_pascal_case(s), "HelloWorld".to_string())
    }

    #[test]
    fn test_as_snake_case() {
        let s = "HelloWorld";
        assert_eq!(as_snake_case(s), "hello_world".to_string())
    }

    #[test]
    fn test_schema_transform_structs() {
        let expect_struct = Struct::new(
            "Person",
            None,
            vec![
                Field::new("last_name", "String", Some("last name".to_owned())),
                Field::new("first_name", "String", Some("first name".to_owned())),
            ],
        );
        let schema = create_simple_schema();
        assert_eq!(schema.into_structs(), vec![expect_struct]);
    }
}
