use crate::types::Tool;

pub fn filename(tool: &Tool) -> bool {
    tool.name.len() <= 50
}

pub fn description(tool: &Tool) -> bool {
    tool.description.len() <= 200
}
