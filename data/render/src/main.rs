#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;

mod lints;
mod render;
mod types;

use render::render;
use types::Tool;

fn is_sorted<T>(data: &[T]) -> bool
where
    T: Ord,
{
    data.windows(2).all(|w| w[0] <= w[1])
}

fn valid(tool: &Tool) -> bool {
    let lints = [lints::filename, lints::description];
    lints.iter().all(|lint| lint(&tool))
}

fn get_file() -> Result<String, Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 1 {
        return Err("Expected a single input file".into());
    }
    Ok(files[0].clone())
}

fn read(file: String) -> Result<Vec<Tool>, Box<dyn Error>> {
    let f = std::fs::File::open(file)?;
    Ok(serde_yaml::from_reader(f)?)
}

fn check(entries: &Vec<Tool>) -> Result<(), Box<dyn Error>> {
    if !is_sorted(&entries) {
        return Err("All list entries must be sorted".into());
    };

    for entry in entries {
        if !valid(&entry) {
            return Err(format!("Error with entry: {}", entry.name).into());
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = get_file()?;
    let entries = read(file)?;
    check(&entries)?;

    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, entries)?;
    println!("{}", rendered);
    Ok(())
}
