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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Pattern;
    use crate::project::Project;

    fn valid_project() -> Project {
        Project::new("Test Project")
    }

    #[test]
    fn test_valid_project_returns_no_errors() {
        let project = valid_project();
        let errors = validate_project(&project);
        assert!(errors.is_empty());
    }

    #[test]
    fn test_empty_name_fails() {
        let mut project = valid_project();
        project.meta.name = "".into();
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "meta.name"));
    }

    #[test]
    fn test_whitespace_name_fails() {
        let mut project = valid_project();
        project.meta.name = "   ".into();
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "meta.name"));
    }

    #[test]
    fn test_no_patterns_fails() {
        let mut project = valid_project();
        project.patterns.clear();
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "patterns"));
    }

    #[test]
    fn test_empty_pattern_name_fails() {
        let mut project = valid_project();
        project.patterns[0].name = "".into();
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "patterns[0].name"));
    }

    #[test]
    fn test_bpm_zero_fails() {
        let mut project = valid_project();
        project.patterns[0].bpm = 0;
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "patterns[0].bpm"));
    }

    #[test]
    fn test_bpm_over_300_fails() {
        let mut project = valid_project();
        project.patterns[0].bpm = 301;
        let errors = validate_project(&project);
        assert!(errors.iter().any(|e| e.field == "patterns[0].bpm"));
    }

    #[test]
    fn test_bpm_ok_at_boundaries() {
        let mut project = valid_project();
        project.patterns[0].bpm = 1;
        assert!(validate_project(&project).is_empty());
        project.patterns[0].bpm = 300;
        assert!(validate_project(&project).is_empty());
    }

    #[test]
    fn test_grid_step_mismatch_fails() {
        let mut project = valid_project();
        project.patterns[0]
            .grid
            .get_mut("kick")
            .unwrap()
            .push(false);
        let errors = validate_project(&project);
        assert!(errors
            .iter()
            .any(|e| e.field.starts_with("patterns[0].grid[")));
    }

    #[test]
    fn test_multiple_errors_collected() {
        let mut project = valid_project();
        project.meta.name = "".into();
        project.patterns[0].name = "".into();
        project.patterns[0].bpm = 0;
        let errors = validate_project(&project);
        assert!(errors.len() >= 3);
    }

    #[test]
    fn test_pattern_with_no_steps() {
        let mut project = valid_project();
        let mut p = Pattern::new("empty", "Empty");
        p.steps = 0;
        p.grid = std::collections::HashMap::new();
        project.patterns.push(p);
        let errors = validate_project(&project);
        assert!(errors.is_empty());
    }
}
