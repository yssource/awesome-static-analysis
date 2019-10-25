use crate::types::Entry;

pub fn filename(entry: &Entry) -> bool {
    entry.name.len() <= 50
}

pub fn description(entry: &Entry) -> bool {
    entry.description.len() <= 200
}
