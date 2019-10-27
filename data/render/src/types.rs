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

    pub fn is_language(&self, tag: &str) -> bool {
        self.languages.iter().any(|lang| lang.tag == tag)
    }
}

pub type Tags = HashSet<String>;

#[derive(Clone, Debug, Serialize, Deserialize, Eq)]
pub struct Entry {
    pub name: String,
    pub url: String,
    pub description: String,
    // TODO #[validate(length(min = 1))]
    pub tags: Tags,
    pub proprietary: Option<bool>,
    pub deprecated: Option<bool>,
    pub wrapper: Option<bool>,
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
pub struct Catalog {
    pub linters: BTreeMap<String, Vec<Entry>>,
    pub others: BTreeMap<String, Vec<Entry>>,
}
