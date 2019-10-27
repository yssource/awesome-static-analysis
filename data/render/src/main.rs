#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;

mod lints;
mod render;
mod types;

use render::render;
use types::{Categories, Entry};

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
fn check(categories: &Categories, entries: &Vec<Entry>) -> Result<(), Box<dyn Error>> {
    if !is_sorted(&entries) {
        return Err("All list entries must be sorted".into());
    };

    for entry in entries {
        for category in &entry.categories {
            if !categories.contains(category) {
                return Err(format!(
                "Unknown category `{}` for entry `{}`. It might be missing from the categories file.",
                category, entry.name
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

fn main() -> Result<(), Box<dyn Error>> {
    let (categories, data) = get_files()?;
    let categories = read_categories(categories)?;
    let entries = read_entries(data)?;
    check(&categories, &entries)?;

    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, entries)?;
    println!("{}", rendered);
    Ok(())
}
