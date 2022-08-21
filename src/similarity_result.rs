use serde::Serialize;

use crate::patch_similarity_comparator::PatchSimilarityResult;

#[derive(Debug, Serialize)]
/// Contains information for the similarity of 2 signals
pub struct SimilarityResult {
    /// Predicted MOS: 1 = bad, 5 = excellent
    pub moslqo: f64,
    /// Mean of fvnsim scores
    pub vnsim: f64,
    /// Similarity score of reference and degraded file per frequency band, ordered from lowest to highest frequency
    pub fvnsim: Vec<f64>,
    /// Standard deviation of similarity per frequency band
    pub fstdnsim: Vec<f64>,
    /// Degraded energy for each frequency
    pub fvdegenergy: Vec<f64>,
    /// Frequencies for which a similarity sore was computed
    pub center_freq_bands: Vec<f64>,
    /// Similarity data for each patch in the signal
    pub patch_sims: Vec<PatchSimilarityResult>,
}

impl SimilarityResult {
    pub fn new(
        moslqo: f64,
        vnsim: f64,
        fnsim: Vec<f64>,
        fstdnsim: Vec<f64>,
        fvdegenergy: Vec<f64>,
        center_freq_bands: Vec<f64>,
        patch_sims: Vec<PatchSimilarityResult>,
    ) -> Self {
        Self {
            moslqo,
            vnsim,
            fvnsim: fnsim,
            fstdnsim,
            fvdegenergy,
            center_freq_bands,
            patch_sims,
        }
    }
}
