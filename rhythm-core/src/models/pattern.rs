use crate::models::Instrument;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub id: String,
    pub name: String,
    pub bpm: u32,
    pub steps: u32,
    pub instruments: Vec<Instrument>,
    pub grid: HashMap<String, Vec<bool>>,
}

impl Pattern {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        let defaults = vec![
            Instrument {
                id: "kick".into(),
                name: "Kick".into(),
                sound: "kick".into(),
                volume: 0.8,
                sample: Some("kick".into()),
            },
            Instrument {
                id: "snare".into(),
                name: "Snare".into(),
                sound: "snare".into(),
                volume: 0.8,
                sample: Some("snare".into()),
            },
            Instrument {
                id: "hihat-closed".into(),
                name: "Hi-Hat".into(),
                sound: "hihat-closed".into(),
                volume: 0.8,
                sample: Some("hihat-closed".into()),
            },
            Instrument {
                id: "hihat-open".into(),
                name: "Open Hat".into(),
                sound: "hihat-open".into(),
                volume: 0.8,
                sample: Some("hihat-open".into()),
            },
            Instrument {
                id: "crash".into(),
                name: "Crash".into(),
                sound: "crash".into(),
                volume: 0.8,
                sample: Some("crash".into()),
            },
        ];
        let mut grid = HashMap::new();
        for inst in &defaults {
            grid.insert(inst.id.clone(), vec![false; 16]);
        }
        Self {
            id: id.into(),
            name: name.into(),
            bpm: 120,
            steps: 16,
            instruments: defaults,
            grid,
        }
    }

    pub fn toggle_step(&mut self, instrument_id: &str, step: usize) {
        if let Some(steps) = self.grid.get_mut(instrument_id) {
            if step < steps.len() {
                steps[step] = !steps[step];
            }
        }
    }

    pub fn set_steps(&mut self, count: u32) {
        self.steps = count;
        for steps in self.grid.values_mut() {
            steps.resize(count as usize, false);
        }
    }

    pub fn add_instrument(&mut self, instrument: Instrument) {
        self.grid
            .insert(instrument.id.clone(), vec![false; self.steps as usize]);
        self.instruments.push(instrument);
    }

    pub fn remove_instrument(&mut self, id: &str) {
        self.grid.remove(id);
        self.instruments.retain(|i| i.id != id);
    }

    pub fn rename_instrument(&mut self, id: &str, new_name: &str) {
        if let Some(inst) = self.instruments.iter_mut().find(|i| i.id == id) {
            inst.name = new_name.to_string();
        }
    }

    pub fn is_grid_consistent(&self) -> bool {
        for inst in &self.instruments {
            if !self.grid.contains_key(&inst.id) {
                return false;
            }
        }
        for id in self.grid.keys() {
            if !self.instruments.iter().any(|i| i.id == *id) {
                return false;
            }
        }
        true
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new("pattern-1", "Pattern 1")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pattern_has_defaults() {
        let p = Pattern::new("p1", "Test");
        assert_eq!(p.id, "p1");
        assert_eq!(p.name, "Test");
        assert_eq!(p.bpm, 120);
        assert_eq!(p.steps, 16);
        assert_eq!(p.instruments.len(), 5);
        assert!(p.is_grid_consistent());
    }

    #[test]
    fn test_toggle_step() {
        let mut p = Pattern::default();
        p.toggle_step("kick", 0);
        assert!(p.grid["kick"][0]);
        p.toggle_step("kick", 0);
        assert!(!p.grid["kick"][0]);
    }

    #[test]
    fn test_toggle_step_out_of_bounds_is_noop() {
        let mut p = Pattern::default();
        p.toggle_step("kick", 999);
    }

    #[test]
    fn test_toggle_step_unknown_instrument_is_noop() {
        let mut p = Pattern::default();
        p.toggle_step("nonexistent", 0);
    }

    #[test]
    fn test_set_steps_resizes_all_grids() {
        let mut p = Pattern::default();
        p.set_steps(8);
        assert_eq!(p.steps, 8);
        for steps in p.grid.values() {
            assert_eq!(steps.len(), 8);
        }
    }

    #[test]
    fn test_add_instrument() {
        let mut p = Pattern::default();
        let inst = Instrument::new("cowbell", "Cowbell").with_volume(0.5);
        p.add_instrument(inst);
        assert_eq!(p.instruments.len(), 6);
        assert!(p.grid.contains_key("cowbell"));
        assert_eq!(p.grid["cowbell"].len(), 16);
        assert!(p.is_grid_consistent());
    }

    #[test]
    fn test_remove_instrument() {
        let mut p = Pattern::default();
        p.remove_instrument("kick");
        assert_eq!(p.instruments.len(), 4);
        assert!(!p.grid.contains_key("kick"));
        assert!(p.is_grid_consistent());
    }

    #[test]
    fn test_remove_nonexistent_instrument_is_noop() {
        let mut p = Pattern::default();
        p.remove_instrument("nothing");
        assert_eq!(p.instruments.len(), 5);
    }

    #[test]
    fn test_rename_instrument() {
        let mut p = Pattern::default();
        p.rename_instrument("kick", "Big Kick");
        let kick = p.instruments.iter().find(|i| i.id == "kick").unwrap();
        assert_eq!(kick.name, "Big Kick");
    }

    #[test]
    fn test_rename_nonexistent_is_noop() {
        let mut p = Pattern::default();
        p.rename_instrument("nothing", "whatever");
    }

    #[test]
    fn test_default() {
        let p = Pattern::default();
        assert_eq!(p.id, "pattern-1");
        assert_eq!(p.name, "Pattern 1");
    }

    #[test]
    fn test_grid_starts_all_false() {
        let p = Pattern::default();
        for steps in p.grid.values() {
            assert!(steps.iter().all(|&s| !s));
        }
    }

    #[test]
    fn test_serde_roundtrip() {
        let mut p = Pattern::default();
        p.toggle_step("kick", 0);
        p.toggle_step("snare", 4);
        let json = serde_json::to_string_pretty(&p).unwrap();
        let back: Pattern = serde_json::from_str(&json).unwrap();
        assert_eq!(back.grid["kick"][0], true);
        assert_eq!(back.grid["snare"][4], true);
        assert!(back.is_grid_consistent());
    }

    #[test]
    fn test_grid_consistency_invariant() {
        let mut p = Pattern::default();
        assert!(p.is_grid_consistent());
        p.remove_instrument("kick");
        assert!(p.is_grid_consistent());
        p.add_instrument(Instrument::new("tom", "Tom"));
        assert!(p.is_grid_consistent());
    }
}
