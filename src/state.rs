use std::collections::HashMap;

/// Holds persistent widget state across frames, keyed by widget label.
/// This is the core mechanism that makes the Streamlit-like API work:
/// widgets return their current value each frame, and state persists
/// between frames so users don't need to manage it manually.
#[derive(Default)]
pub struct WidgetState {
    strings: HashMap<String, String>,
    floats: HashMap<String, f64>,
    bools: HashMap<String, bool>,
    usizes: HashMap<String, usize>,
}

impl WidgetState {
    pub fn get_string(&self, key: &str) -> Option<&String> {
        self.strings.get(key)
    }

    pub fn set_string(&mut self, key: &str, value: String) {
        self.strings.insert(key.to_string(), value);
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.floats.get(key).copied()
    }

    pub fn set_f64(&mut self, key: &str, value: f64) {
        self.floats.insert(key.to_string(), value);
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.bools.get(key).copied()
    }

    pub fn set_bool(&mut self, key: &str, value: bool) {
        self.bools.insert(key.to_string(), value);
    }

    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.usizes.get(key).copied()
    }

    pub fn set_usize(&mut self, key: &str, value: usize) {
        self.usizes.insert(key.to_string(), value);
    }
}
