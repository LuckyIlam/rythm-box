use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::Instrument;

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
            Instrument { id: "kick".into(), name: "Kick".into(), sound: "kick".into(), volume: 0.8, sample: Some("kick".into()) },
            Instrument { id: "snare".into(), name: "Snare".into(), sound: "snare".into(), volume: 0.8, sample: Some("snare".into()) },
            Instrument { id: "hihat-closed".into(), name: "Hi-Hat".into(), sound: "hihat-closed".into(), volume: 0.8, sample: Some("hihat-closed".into()) },
            Instrument { id: "hihat-open".into(), name: "Open Hat".into(), sound: "hihat-open".into(), volume: 0.8, sample: Some("hihat-open".into()) },
            Instrument { id: "crash".into(), name: "Crash".into(), sound: "crash".into(), volume: 0.8, sample: Some("crash".into()) },
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
        self.grid.insert(instrument.id.clone(), vec![false; self.steps as usize]);
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
}

impl Default for Pattern {
    fn default() -> Self {
        Self::new("pattern-1", "Pattern 1")
    }
}
