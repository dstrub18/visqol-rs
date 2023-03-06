/// Temporal analysis window used for creating spectrograms
pub struct AnalysisWindow {
    /// Duration of the window in seconds
    pub window_duration: f64,
    /// Size of the window in samples
    pub size: usize,
    /// Overlap of the window in milliseconds
    pub overlap: f64,
}

impl AnalysisWindow {
    /// Creates a new analysis window based on sample rate, desired overlap and duration
    pub fn new(sample_rate: u32, overlap: f64, window_duration: f64) -> Self {
        Self {
            window_duration,
            size: (sample_rate as f64 * window_duration).round() as usize,
            overlap,
        }
    }
}
