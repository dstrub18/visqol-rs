use crate::patch_similarity_comparator::{PatchSimilarityComparator, PatchSimilarityResult};
use crate::convolution_2d::perform_valid_2d_conv_with_boundary;
use ndarray::{Array2, ShapeBuilder, Axis, Array1};
pub struct NeurogramSimiliarityIndexMeasure
{
    intensity_range: f64
}

impl Default for NeurogramSimiliarityIndexMeasure 
{
    fn default() -> Self
    {
        Self
        {
            intensity_range: 1.0
        }
    }
}

impl PatchSimilarityComparator for NeurogramSimiliarityIndexMeasure
{
    fn measure_patch_similarity(&self, ref_patch: &mut ndarray::Array2<f64>, deg_patch: &mut ndarray::Array2<f64>) -> PatchSimilarityResult
    {
        let window_vec = vec![0.0113033910173052, 0.0838251475442633, 0.0113033910173052,
        0.0838251475442633, 0.619485845753726,  0.0838251475442633,
        0.0113033910173052, 0.0838251475442633, 0.0113033910173052];
        let window = Array2::from_shape_vec((3,3).f(), window_vec).unwrap();

        // what is k?
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
        let conv2_ref_neuro_squared = perform_valid_2d_conv_with_boundary(&window, &mut ref_neuro_sq);
        let sigma_ref_squared = &conv2_ref_neuro_squared - &ref_mu_squared;
        
        let conv2_deg_neuro_squared = perform_valid_2d_conv_with_boundary(&window, &mut deg_neuro_sq);
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
        structure_denominator.iter_mut().for_each(|element|{* element = if *element < 0.0 {c3} else {element.sqrt() + c3}});
        
        let structure = &structure_numerator / &structure_denominator;
        let sim_map = &intensity * &structure;
        
        let freq_band_deg_energy: Array1<f64> = deg_patch.mean_axis(Axis(1)).unwrap();
        let freq_band_means: Array1<f64> = sim_map.mean_axis(Axis(1)).unwrap();
        let freq_band_std: Array1<f64> = sim_map.std_axis(Axis(1), 1.0);
        let mean_freq_band_means = freq_band_means.mean().unwrap();

        PatchSimilarityResult::new(freq_band_means, freq_band_std, freq_band_deg_energy, mean_freq_band_means)

    }
}