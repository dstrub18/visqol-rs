
use visqol_rs::analysis_window::AnalysisWindow;

#[test]
fn analysis_window_test()
{
    let sr_8k = 8000;
    let sr_16k = 16000;
    let sr_225k = 22050;
    let sr_441k = 44100;
    let sr_48k = 48000;
    let sr_96k = 96000;

    let temporal_window = 80.0; // unit: ms
    let overlap = 0.25;

    let window_dur = 0.08;

    let window_size = AnalysisWindow::new(sr_8k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_8k, window_size), temporal_window);

    let window_size = AnalysisWindow::new(sr_16k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_16k, window_size), temporal_window);

    let window_size = AnalysisWindow::new(sr_225k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_225k, window_size), temporal_window);

    let window_size = AnalysisWindow::new(sr_441k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_441k, window_size), temporal_window);

    let window_size = AnalysisWindow::new(sr_48k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_48k, window_size), temporal_window);

    let window_size = AnalysisWindow::new(sr_96k, overlap, window_dur).size;
    assert_eq!(calculate_temporal_window(sr_96k, window_size), temporal_window);
}

fn calculate_temporal_window(sr: u32, window_size: usize) -> f64
{
    ((1000.0 / sr as f64) * window_size as f64).round()
}