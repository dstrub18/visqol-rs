/// Temporal analysis window used for creating spectrograms
pub struct AnalysisWindow {
    /// Size of the window in samples
    pub size: usize,
    /// Overlap of the window in milliseconds
    pub overlap: f64,
}

impl AnalysisWindow {
    /// Creates a new analysis window based on sample rate, desired overlap and duration
    pub fn new(sample_rate: u32, overlap: f64, window_duration: f64) -> Self {
        Self {
            size: (sample_rate as f64 * window_duration).round() as usize,
            overlap,
        }
    }
}
