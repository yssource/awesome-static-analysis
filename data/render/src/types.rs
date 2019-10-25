use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Entry,
    Service,
    Collection,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Entry {
    pub name: String,
    pub url: String,
    pub description: String,
    pub categories: Vec<String>,
    pub proprietary: Option<bool>,
    pub deprecated: Option<bool>,
    #[serde(rename = "type")]
    pub ttype: Type,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Entry) -> bool {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Groups {
    pub collections: Vec<Entry>,
    pub tools: BTreeMap<String, Vec<Entry>>,
}
