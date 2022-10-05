use crate::convolution_2d::perform_valid_2d_conv_with_boundary;
use crate::patch_similarity_comparator::{PatchSimilarityComparator, PatchSimilarityResult};
use ndarray::{arr2, Array1, Axis};

/// Provides a neurogram similarity index measure (NSIM) implementation for a
/// patch similarity comparator. NSIM is a distance metric, adapted from the
/// image processing technique called structural similarity (SSIM) and is here
/// used to compare two patches taken from the reference and degraded
/// spectrograms.
pub struct NeurogramSimiliarityIndexMeasure {
    intensity_range: f64,
}

#[allow(unused)]
impl NeurogramSimiliarityIndexMeasure {
    pub fn new(intensity_range: f64) -> Self { Self { intensity_range } }
}

impl Default for NeurogramSimiliarityIndexMeasure {
    fn default() -> Self {
        Self {
            intensity_range: 1.0,
        }
    }
}

impl PatchSimilarityComparator for NeurogramSimiliarityIndexMeasure {
    /// Computes the NSIM between `ref_patch` and `deg_patch` and returns the mean and standard deviation of each frequency band, the energy of the degraded patch and the similarity score.
    fn measure_patch_similarity(
        &self,
        ref_patch: &mut ndarray::Array2<f64>,
        deg_patch: &mut ndarray::Array2<f64>,
    ) -> PatchSimilarityResult {
        let window = arr2(&[
            [0.0113033910173052, 0.0838251475442633, 0.0113033910173052],
            [0.0838251475442633, 0.619485845753726, 0.0838251475442633],
            [0.0113033910173052, 0.0838251475442633, 0.0113033910173052],
        ]);

        let k = [0.01, 0.03];
        let c1 = (k[0] * self.intensity_range).powf(2.0);
        let c3 = (k[1] * self.intensity_range).powf(2.0) / 2.0;

        // Compute mu
        let mu_ref = perform_valid_2d_conv_with_boundary(&window, ref_patch);
        let mu_deg = perform_valid_2d_conv_with_boundary(&window, deg_patch);

        let ref_mu_squared = &mu_ref * &mu_ref;
        let deg_mu_squared = &mu_deg * &mu_deg;
        let mu_r_mu_d = &mu_ref * &mu_deg;

        let mut ref_neuro_sq = ref_patch.clone() * ref_patch.clone();
        let mut deg_neuro_sq = deg_patch.clone() * deg_patch.clone();

        // Compute sigmas
        let conv2_ref_neuro_squared =
            perform_valid_2d_conv_with_boundary(&window, &mut ref_neuro_sq);
        let sigma_ref_squared = &conv2_ref_neuro_squared - &ref_mu_squared;

        let conv2_deg_neuro_squared =
            perform_valid_2d_conv_with_boundary(&window, &mut deg_neuro_sq);
        let sigma_deg_squared = &conv2_deg_neuro_squared - &deg_mu_squared;

        let mut ref_neuro_deg = ref_patch.clone() * deg_patch.clone();
        let conv2_ref_neuro_deg = perform_valid_2d_conv_with_boundary(&window, &mut ref_neuro_deg);

        let sigma_r_d = &conv2_ref_neuro_deg - &mu_r_mu_d;

        // Compute intensity
        let intensity_numerator = &mu_r_mu_d * 2.0 + c1;
        let intensity_denominator = &ref_mu_squared + &deg_mu_squared + c1;

        let intensity = &intensity_numerator / &intensity_denominator;

        // Compute structure
        let structure_numerator = &sigma_r_d + c3;
        let mut structure_denominator = &sigma_ref_squared * &sigma_deg_squared;

        // Avoid nans
        structure_denominator.map_inplace(|element| {
            *element = if *element < 0.0 {
                c3
            } else {
                element.sqrt() + c3
            }
        });

        let structure = &structure_numerator / &structure_denominator;
        let sim_map = &intensity * &structure;

        let freq_band_deg_energy: Array1<f64> = deg_patch
            .mean_axis(Axis(1))
            .expect("Failed to compute mean for degraded signal!");
        let freq_band_means: Array1<f64> = sim_map
            .mean_axis(Axis(1))
            .expect("Failed to compute mean for similarity map!");
        let freq_band_std: Array1<f64> = sim_map.std_axis(Axis(1), 1.0);
        let mean_freq_band_means = freq_band_means
            .mean()
            .expect("Failed to compute mean of means for degraded signal!");

        PatchSimilarityResult::new(
            freq_band_means.to_vec(),
            freq_band_std.to_vec(),
            freq_band_deg_energy.to_vec(),
            mean_freq_band_means,
        )
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_abs_diff_eq;
    use ndarray::Array2;

    use super::*;

    #[test]
    fn test_neurogram_measure() {
        let ref_patch = vec![1.0, 0.0, 0.0];
        let mut ref_patch_mat = Array2::from_shape_vec((3, 1), ref_patch).unwrap();
        let deg_patch = vec![0.0, 0.0, 0.0];
        let mut deg_patch_mat = Array2::from_shape_vec((3, 1), deg_patch).unwrap();
        let expected_result = vec![0.000125225, 0.00875062, 1.0];

        let sim_comparator = NeurogramSimiliarityIndexMeasure::default();

        let result =
            sim_comparator.measure_patch_similarity(&mut ref_patch_mat, &mut deg_patch_mat);

        assert_abs_diff_eq!(
            result.freq_band_means[0],
            expected_result[0],
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            result.freq_band_means[1],
            expected_result[1],
            epsilon = 0.0001
        );
        assert_abs_diff_eq!(
            result.freq_band_means[2],
            expected_result[2],
            epsilon = 0.0001
        );
    }
}
