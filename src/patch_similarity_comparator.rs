use ndarray::Array2 as ImagePatch;
use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
pub struct PatchSimilarityResult {
    pub freq_band_means: Vec<f64>,
    pub freq_band_stddevs: Vec<f64>,
    pub freq_band_deg_energy: Vec<f64>,
    pub similarity: f64,
    pub ref_patch_start_time: f64,
    pub ref_patch_end_time: f64,
    pub deg_patch_start_time: f64,
    pub deg_patch_end_time: f64,
}

impl PatchSimilarityResult {
    pub fn new(
        freq_band_means: Vec<f64>,
        freq_band_stddevs: Vec<f64>,
        freq_band_deg_energy: Vec<f64>,
        similarity: f64,
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

    pub fn empty() -> Self {
        Self {
            freq_band_means: Vec::<f64>::new(),
            freq_band_stddevs: Vec::<f64>::new(),
            freq_band_deg_energy: Vec::<f64>::new(),
            similarity: 0.0,
            ref_patch_start_time: 0.0,
            ref_patch_end_time: 0.0,
            deg_patch_start_time: 0.0,
            deg_patch_end_time: 0.0,
        }
    }
}

pub trait PatchSimilarityComparator {
    fn measure_patch_similarity(
        &self,
        ref_patch: &mut ImagePatch<f64>,
        deg_patch: &mut ImagePatch<f64>,
    ) -> PatchSimilarityResult;
}
