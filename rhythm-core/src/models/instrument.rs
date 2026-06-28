use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    pub id: String,
    pub name: String,
    pub sound: String,
    pub volume: f32,
    pub sample: Option<String>,
}

impl Instrument {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            sound: String::from("sine"),
            volume: 0.8,
            sample: None,
        }
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Self::new("inst", "Instrument")
    }
}
