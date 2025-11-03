use ndarray::Array2 as ImagePatch;
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
/// Bundles similarity information of a single patch.
/// The term `Patch` here refers to a single of spectrogram data produced by a PatchCreator)
pub struct PatchSimilarityResult {
    /// Means of the individual frequency bands
    pub freq_band_means: Vec<f32>,
    /// Standard deviation of the individual frequency bands
    pub freq_band_stddevs: Vec<f32>,
    /// Energy of the degraded file per frequency band
    pub freq_band_deg_energy: Vec<f32>,
    /// Calculated patch similarity score
    pub similarity: f32,
    /// Reference start of patch in seconds
    pub ref_patch_start_time: f32,
    /// Reference end of patch in seconds
    pub ref_patch_end_time: f32,
    /// Degraded start of patch in seconds
    pub deg_patch_start_time: f32,
    /// Degraded end of patch in seconds
    pub deg_patch_end_time: f32,
}

impl PatchSimilarityResult {
    /// Creates a new similarity result, stores mean, std and energy of degraded signal and sets the time information to 0
    pub fn new(
        freq_band_means: Vec<f32>,
        freq_band_stddevs: Vec<f32>,
        freq_band_deg_energy: Vec<f32>,
        similarity: f32,
    ) -> Self {
        Self {
            freq_band_means,
            freq_band_stddevs,
            freq_band_deg_energy,
            similarity,
            ref_patch_start_time: 0.0,
            ref_patch_end_time: 0.0,
            deg_patch_start_time: 0.0,
            deg_patch_end_time: 0.0,
        }
    }
}

impl Default for PatchSimilarityResult {
    fn default() -> Self {
        Self {
            freq_band_means: Vec::<f32>::new(),
            freq_band_stddevs: Vec::<f32>::new(),
            freq_band_deg_energy: Vec::<f32>::new(),
            similarity: 0.0,
            ref_patch_start_time: 0.0,
            ref_patch_end_time: 0.0,
            deg_patch_start_time: 0.0,
            deg_patch_end_time: 0.0,
        }
    }
}

/// If implemented, this trait allows for computing a similarity score of 2 patches
pub trait PatchSimilarityComparator {
    fn measure_patch_similarity(
        &self,
        ref_patch: &mut ImagePatch<f32>,
        deg_patch: &mut ImagePatch<f32>,
    ) -> PatchSimilarityResult;
}
