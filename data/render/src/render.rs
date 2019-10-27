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

    let mut linter_map = BTreeMap::new();
    let linters: Vec<Entry> = entries
        .into_iter()
        .filter(|e| e.ttype == Type::Linter)
        .collect();

    for linter in linters {
        for category in &linter.categories {
            let cat_entries = linter_map.entry(category.clone()).or_insert_with(|| vec![]);
            cat_entries.push(linter.clone());
        }
    }
    return Groups {
        linters: linter_map,
        collections,
    };
}

pub fn render(template: &str, entries: Vec<Entry>) -> Result<String, Box<dyn Error>> {
    let mut context = Context::new();
    let groups = group(entries);
    context.insert("groups", &groups);
    Ok(Tera::one_off(template, &context, true)?)
}
