use approx::assert_abs_diff_eq;
use ndarray::Array2;
use visqol_rs::{neurogram_similiarity_index_measure::NeurogramSimiliarityIndexMeasure, patch_similarity_comparator::PatchSimilarityComparator};

#[test]
fn test_neurogram_measure()
{
    let ref_patch = vec![1.0, 0.0, 0.0];
    let mut ref_patch_mat = Array2::from_shape_vec((3,1), ref_patch).unwrap();
    let deg_patch = vec![0.0, 0.0, 0.0];
    let mut deg_patch_mat = Array2::from_shape_vec((3,1), deg_patch).unwrap();
    let expected_result = vec![0.000125225, 0.00875062, 1.0];

    let sim_comparator = NeurogramSimiliarityIndexMeasure::default();

    let result = sim_comparator.measure_patch_similarity(&mut ref_patch_mat, &mut deg_patch_mat);

    assert_abs_diff_eq!(result.freq_band_means[0], expected_result[0], epsilon = 0.0001);
    assert_abs_diff_eq!(result.freq_band_means[1], expected_result[1], epsilon = 0.0001);
    assert_abs_diff_eq!(result.freq_band_means[2], expected_result[2], epsilon = 0.0001);
}