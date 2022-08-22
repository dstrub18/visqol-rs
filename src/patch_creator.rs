use ndarray::Array2;

use crate::{
    analysis_window::AnalysisWindow, audio_signal::AudioSignal, visqol_error::VisqolError,
};

/// This trait enables the creation of patches from a spectrogram.
/// The term `patch` here refers to a segment of 2-dimensional data. How the data is segmented is determined by the individual implementation of this trait.
pub trait PatchCreator {

    /// Given a spectrogram, this function returns 0-indexed indices of each patch.
    fn create_ref_patch_indices(
        &self,
        spectrogram: &Array2<f64>,
        ref_signal: &AudioSignal,
        window: &AnalysisWindow,
    ) -> Result<Vec<usize>, VisqolError>;
    
    /// Given a spectrogram and the corresponding indices, this function performs the segmentation and returns each patch in a vector of 2-dimensional arrays.
    fn create_patches_from_indices(
        &self,
        spectrogram: &Array2<f64>,
        patch_indices: &[usize],
    ) -> Vec<Array2<f64>>;
}
