use ndarray::Array2;
use ndarray::Array2 as ImagePatch;
#[allow(unused)]
pub struct PatchSimilarityResult
{
    freq_band_means: Array2<f64>,
    freq_band_stddevs: Array2<f64>,
    freq_band_deg_energy: Array2<f64>,
    similarity: f64,
    ref_patch_start_time: f64,
    ref_patch_end_time: f64,
    deg_patch_start_time: f64,
    deg_patch_end_time: f64,
}
#[allow(unused)]
pub struct BestPatchSimilarityMatch
{
    result: PatchSimilarityResult
}

pub trait PatchSimilarityComparator
{
    fn measure_patch_similarity(ref_patch: &ImagePatch<f64>, deg_patch: &ImagePatch<f64>);
}