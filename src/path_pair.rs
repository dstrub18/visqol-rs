use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// Bundles 2 `String`s into a single struct.
/// Useful since ViSQOL is a full-reference metric, i.e. it requires a reference signal and a degraded signal
pub struct PathPair {
    pub reference: String,
    pub degraded: String,
}
impl PathPair {
    pub fn new(reference: &str, degraded: &str) -> Self {
        Self {
            reference: String::from(reference),
            degraded: String::from(degraded),
        }
    }
}
