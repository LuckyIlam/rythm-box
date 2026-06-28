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

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, vol: f32) {
        self.volume = vol.clamp(0.0, 1.0);
    }

    pub fn with_volume(mut self, vol: f32) -> Self {
        self.set_volume(vol);
        self
    }
}

impl Default for Instrument {
    fn default() -> Self {
        Self::new("inst", "Instrument")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_clamps_to_range() {
        let mut inst = Instrument::new("kick", "Kick");
        inst.set_volume(1.5);
        assert_eq!(inst.volume(), 1.0);
        inst.set_volume(-0.5);
        assert_eq!(inst.volume(), 0.0);
        inst.set_volume(0.5);
        assert_eq!(inst.volume(), 0.5);
    }

    #[test]
    fn test_with_volume() {
        let inst = Instrument::new("kick", "Kick").with_volume(2.0);
        assert_eq!(inst.volume(), 1.0);
    }

    #[test]
    fn test_default_volume() {
        let inst = Instrument::default();
        assert_eq!(inst.volume(), 0.8);
    }

    #[test]
    fn test_new_sets_no_sample() {
        let inst = Instrument::new("hat", "Hat");
        assert_eq!(inst.sample, None);
        assert_eq!(inst.sound, "sine");
    }
}
