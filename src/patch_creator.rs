use ndarray::Array2;

use crate::{audio_signal::AudioSignal, analysis_window::AnalysisWindow};

pub trait PatchCreator
{
    fn create_ref_patch_indices(&self, spectrogram: &Array2<f64>, ref_signal: &AudioSignal, window: &AnalysisWindow) -> Vec<usize>;
    fn create_patches_from_indices(&self, spectrogram: &Array2<f64>, patch_indices: &Vec<usize>) -> Vec<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>>;
}