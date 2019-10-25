use crate::types::{Entry, Groups, Type};
use std::collections::BTreeMap;
use std::error::Error;
use tera::{Context, Tera};

fn group(entries: Vec<Entry>) -> Groups {
    let collections = entries
        .iter()
        .cloned()
        .filter(|e| e.ttype == Type::Collection)
        .collect();

    let mut tools_map = BTreeMap::new();
    let tools: Vec<Entry> = entries
        .into_iter()
        .filter(|e| e.ttype == Type::Entry)
        .collect();

    for tool in tools {
        for category in &tool.categories {
            let cat_entries = tools_map.entry(category.clone()).or_insert_with(|| vec![]);
            cat_entries.push(tool.clone());
        }
    }
    return Groups {
        tools: tools_map,
        collections,
    };
}

pub fn render(template: &str, entries: Vec<Entry>) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    let groups = group(entries);
    context.insert("groups", &groups);
    Ok(Tera::one_off(template, &context, true)?)
}
