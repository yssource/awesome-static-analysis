use crate::tool::Tool;
use std::error::Error;

use tera::{Context, Tera};

pub fn render(template: &str, tools: Vec<Tool>) -> Result<(), Box<dyn Error>> {
    let mut context = Context::new();
    context.insert("tools", &tools);
    let rendered = Tera::one_off(template, &context, true)?;
    println!("{}", rendered);

    Ok(())
}
