#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;

mod lints;
mod render;
mod types;

use render::render;
use std::collections::BTreeMap;
use types::{Catalog, Categories, Entry};

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn valid(entry: &Entry) -> bool {
    let lints = [lints::filename, lints::description];
    lints.iter().all(|lint| lint(&entry))
}

fn get_files() -> Result<(String, String), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 2 {
        return Err("Expected a two input files, `data.yml` and `categories.yml`".into());
    }
    Ok((files[0].clone(), files[1].clone()))
}

fn read_categories(file: String) -> Result<Categories, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn read_entries(file: String) -> Result<Vec<Entry>, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn validate(categories: &Categories, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
    if !is_sorted(&entries) {
        return Err("All list entries must be sorted".into());
    };

    for entry in entries {
        for tag in &entry.tags {
            if !categories.contains(tag) {
                return Err(format!(
                "Unknown tag `{}` for entry `{}`. It might be missing from the categories file.",
                tag , entry.name
            )
                .into());
            }
        }
        if !valid(&entry) {
            return Err(format!("Error in entry: {}", entry.name).into());
        }
    }
    Ok(())
}

fn group(categories: &Categories, entries: Vec<Entry>) -> Catalog {
    let mut linters = BTreeMap::new();
    let mut others = BTreeMap::new();
    for entry in entries {
        for tag in &entry.tags {
            if categories.is_language(&tag) {
                let language = linters.entry(tag.into()).or_insert_with(|| vec![]);
                language.push(entry.clone());
            } else {
                let other = others.entry(tag.into()).or_insert_with(|| vec![]);
                other.push(entry.clone());
            }
        }
    }

    return Catalog { linters, others };
}

fn main() -> Result<(), Box<dyn Error>> {
    let (categories, data) = get_files()?;
    let categories = read_categories(categories)?;
    let entries = read_entries(data)?;
    validate(&categories, &entries)?;

    let catalog = group(&categories, entries);
    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, catalog)?;
    println!("{}", rendered);
    Ok(())
}
