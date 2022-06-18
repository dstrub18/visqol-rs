use ndarray::Array2;

use crate::{audio_signal::AudioSignal, analysis_window::AnalysisWindow};

pub trait patch_creator
{
    fn create_ref_patch_indices(&self, spectrogram: &Array2<f64>, ref_signal: &AudioSignal, window: &AnalysisWindow) -> Vec<usize>;
}