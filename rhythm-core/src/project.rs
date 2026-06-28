use crate::models::Pattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub version: String,
    pub created: String,
    pub modified: String,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            name: String::from("Untitled"),
            version: String::from("1.0"),
            created: String::new(),
            modified: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectSettings {
    pub master_volume: f32,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self { master_volume: 1.0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub meta: ProjectMeta,
    pub settings: ProjectSettings,
    pub patterns: Vec<Pattern>,
}

impl Project {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            meta: ProjectMeta {
                name: name.into(),
                ..Default::default()
            },
            settings: ProjectSettings::default(),
            patterns: vec![Pattern::default()],
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
