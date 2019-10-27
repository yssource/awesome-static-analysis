use crate::types::Catalog;
use std::error::Error;
use tera::{Context, Tera};

pub fn render(template: &str, catalog: Catalog) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("catalog", &catalog);
    Ok(Tera::one_off(template, &context, true)?)
}
