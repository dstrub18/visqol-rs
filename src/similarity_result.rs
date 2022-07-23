use serde::Serialize;

use crate::patch_similarity_comparator::PatchSimilarityResult;

#[derive(Debug, Serialize)]
pub struct SimilarityResult 
{
    pub moslqo: f64,
    pub vnsim: f64,
    pub fnsim: Vec<f64>,
    pub fstdnsim: Vec<f64>,
    pub fvdegenergy: Vec<f64>,
    pub center_freq_bands: Vec<f64>,
    pub patch_sims: Vec<PatchSimilarityResult>
}

impl SimilarityResult 
{
    pub fn new(
        moslqo: f64,
        vnsim: f64,
        fnsim: Vec<f64>,
        fstdnsim: Vec<f64>,
        fvdegenergy: Vec<f64>,
        center_freq_bands: Vec<f64>,
        patch_sims: Vec<PatchSimilarityResult>
    ) -> Self
    {
        Self {
            moslqo,
            vnsim,
            fnsim,
            fstdnsim,
            fvdegenergy,
            center_freq_bands,
            patch_sims
        }
    }
}
