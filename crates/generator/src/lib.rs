use std::fs;

use anyhow::{anyhow, Ok, Result};
use json_schema::{Schema, StructsTemplate};
use litrs::Literal;
use proc_macro::TokenStream;

#[proc_macro]
pub fn generator(input: TokenStream) -> TokenStream {
    let filepath = get_string_literal(input).unwrap();
    render_template_to_string(&filepath)
        .unwrap()
        .parse()
        .unwrap()
}

fn render_template_to_string(filepath: &str) -> Result<String> {
    let content = fs::read_to_string(filepath)?;
    let schema: Schema = serde_json::from_str(&content)?;
    let structs = schema.into_structs();
    let template = StructsTemplate { structs };
    Ok(template.to_string())
}

fn get_string_literal(input: TokenStream) -> Result<String> {
    input
        .into_iter()
        .next()
        .and_then(|v| Literal::try_from(v).ok())
        .and_then(|v| match v {
            Literal::String(s) => Some(s.value().to_string()),
            _ => None,
        })
        .ok_or_else(|| anyhow!("Only string literal can be received"))
}
