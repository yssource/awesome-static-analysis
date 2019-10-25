#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::process;

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

fn main() -> Result<(), Box<dyn Error>> {
    let files: Vec<_> = env::args().skip(1).collect();
    if files.len() != 1 {
        println!("Expected a single input file");
        process::exit(1);
    }

    let f = std::fs::File::open(&files[0])?;
    let entries: Vec<Tool> = serde_yaml::from_reader(f)?;

    if !is_sorted(&entries) {
        println!("All list entries must be sorted");
        process::exit(2);
    };

    for entry in &entries {
        if !valid(&entry) {
            println!("Error with entry: {}", entry.name);
            process::exit(3);
        }
    }

    let template = std::fs::read_to_string("src/templates/README.md")?;
    let rendered = render(&template, entries)?;
    println!("{}", rendered);
    Ok(())
}
