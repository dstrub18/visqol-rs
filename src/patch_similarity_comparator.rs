use std::ops::Deref;

use ndarray::Array1;
use ndarray::Array2 as ImagePatch;
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct PatchSimilarityResult
{
    pub freq_band_means: Array1<f64>,
    pub freq_band_stddevs: Array1<f64>,
    pub freq_band_deg_energy: Array1<f64>,
    pub similarity: f64,
    pub ref_patch_start_time: f64,
    pub ref_patch_end_time: f64,
    pub deg_patch_start_time: f64,
    pub deg_patch_end_time: f64,
}

impl PatchSimilarityResult
{
    pub fn new(freq_band_means: Array1<f64>,
        freq_band_stddevs: Array1<f64>,
        freq_band_deg_energy: Array1<f64>,similarity: f64,) -> Self
    {
        Self
        {
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

    pub fn empty() -> Self
    {
        Self
        {
            freq_band_means: Array1::<f64>::zeros(0),
            freq_band_stddevs: Array1::<f64>::zeros(0),
            freq_band_deg_energy: Array1::<f64>::zeros(0),
            similarity: 0.0,
            ref_patch_start_time: 0.0,
            ref_patch_end_time: 0.0,
            deg_patch_start_time: 0.0,
            deg_patch_end_time: 0.0,
        }
    }
}

#[allow(unused)]
pub struct BestPatchSimilarityMatch
{
    result: PatchSimilarityResult
}

pub trait PatchSimilarityComparator
{
    fn measure_patch_similarity(&self, ref_patch: &mut ImagePatch<f64>, deg_patch: &mut ImagePatch<f64>) -> PatchSimilarityResult;
}