use crate::error::ProjectError;
use crate::models::Pattern;
use crate::validation;
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

    pub fn validate(&self) -> Result<(), ProjectError> {
        let errors = validation::validate_project(self);
        if errors.is_empty() {
            Ok(())
        } else {
            let msg = errors
                .iter()
                .map(|e| format!("{}: {}", e.field, e.message))
                .collect::<Vec<_>>()
                .join("; ");
            Err(ProjectError::Validation(msg))
        }
    }

    pub fn to_json(&self) -> Result<String, ProjectError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn from_json(json: &str) -> Result<Self, ProjectError> {
        let project: Self = serde_json::from_str(json)?;
        project.validate()?;
        Ok(project)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project() {
        let p = Project::new("Test");
        assert_eq!(p.meta.name, "Test");
        assert_eq!(p.patterns.len(), 1);
        assert!(p.validate().is_ok());
    }

    #[test]
    fn test_serde_roundtrip() {
        let p = Project::new("Roundtrip");
        let json = p.to_json().unwrap();
        let back = Project::from_json(&json).unwrap();
        assert_eq!(back.meta.name, "Roundtrip");
        assert_eq!(back.patterns.len(), 1);
    }

    #[test]
    fn test_from_json_invalid_project_fails() {
        let json = r#"{"meta":{"name":"","version":"1.0","created":"","modified":""},"settings":{"master_volume":1.0},"patterns":[]}"#;
        let result = Project::from_json(json);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Project name cannot be empty"));
    }

    #[test]
    fn test_from_json_malformed_json_fails() {
        let result = Project::from_json("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_json_roundtrip_is_valid() {
        let p = Project::new("Valid");
        let json = p.to_json().unwrap();
        let back = Project::from_json(&json).unwrap();
        assert_eq!(back.meta.name, "Valid");
    }

    #[test]
    fn test_project_with_empty_name_fails_validation() {
        let mut p = Project::new("");
        p.meta.name = "".into();
        assert!(p.validate().is_err());
    }
}
