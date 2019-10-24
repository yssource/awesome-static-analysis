#[macro_use]
extern crate serde_derive;

use std::env;
use std::error::Error;
use std::process;

mod lints;
mod render;
mod tool;

use render::render;
use tool::Tool;

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
    let tools: Vec<Tool> = serde_yaml::from_reader(f)?;

    if !is_sorted(&tools) {
        println!("List of tools must be sorted");
        process::exit(2);
    };

    for tool in &tools {
        if !valid(&tool) {
            println!("Error with entry: {}", tool.name);
            process::exit(3);
        }
    }

    let template = std::fs::read_to_string("src/templates/README.md")?;
    render(&template, tools)
}
