pub struct AnalysisWindow
{
    pub window_duration: f64,
    pub size: usize,
    pub overlap: f64
}

impl AnalysisWindow
{
    pub fn new(sample_rate: u32, overlap: f64, window_duration: f64) -> Self
    {
        Self
        {
            window_duration: window_duration,
            size: (sample_rate as f64 * window_duration).round() as usize,
            overlap: overlap,
        }
    }
}