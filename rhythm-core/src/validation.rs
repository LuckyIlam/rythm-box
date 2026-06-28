use crate::project::Project;

#[derive(Debug)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
}

pub fn validate_project(project: &Project) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if project.meta.name.trim().is_empty() {
        errors.push(ValidationError {
            field: "meta.name".into(),
            message: "Project name cannot be empty".into(),
        });
    }

    if project.patterns.is_empty() {
        errors.push(ValidationError {
            field: "patterns".into(),
            message: "Project must have at least one pattern".into(),
        });
    }

    for (i, pattern) in project.patterns.iter().enumerate() {
        if pattern.name.trim().is_empty() {
            errors.push(ValidationError {
                field: format!("patterns[{}].name", i),
                message: "Pattern name cannot be empty".into(),
            });
        }
        if pattern.bpm == 0 || pattern.bpm > 300 {
            errors.push(ValidationError {
                field: format!("patterns[{}].bpm", i),
                message: "BPM must be between 1 and 300".into(),
            });
        }
        for (inst_id, steps) in &pattern.grid {
            if steps.len() != pattern.steps as usize {
                errors.push(ValidationError {
                    field: format!("patterns[{}].grid[{}]", i, inst_id),
                    message: format!(
                        "Step count mismatch: expected {}, got {}",
                        pattern.steps,
                        steps.len()
                    ),
                });
            }
        }
    }

    errors
}
