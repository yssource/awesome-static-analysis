use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub name: String,
    pub tag: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Categories {
    pub languages: Vec<Category>,
    pub other: Vec<Category>,
}

impl Categories {
    pub fn contains(&self, tag: &str) -> bool {
        self.languages.iter().any(|lang| lang.tag == tag)
            || self.other.iter().any(|lang| lang.tag == tag)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    Linter,
    Service,
    Collection,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Entry {
    pub name: String,
    pub url: String,
    pub description: String,
    pub categories: HashSet<String>,
    pub proprietary: Option<bool>,
    pub deprecated: Option<bool>,
    pub wrapper: Option<bool>,
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
    pub linters: BTreeMap<String, Vec<Entry>>,
}
