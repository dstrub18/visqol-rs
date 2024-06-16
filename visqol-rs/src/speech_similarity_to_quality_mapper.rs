use crate::math_utils;
use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;

/// Maps a similarity score to a MOS using polynomial mapping.
pub struct SpeechSimilarityToQualityMapper {
    scale_max_to_mos: bool,
}

impl SpeechSimilarityToQualityMapper {
    /// Creates a new `SpeechSimilarityToQualityMapper`.
    /// If `scale_max_to_mos` is set to true, the a quality score of 1.0 will be mapped to 5.0. If not, will be mapped to 4.x.
    pub fn new(scale_to_max_mos: bool) -> Self {
        Self {
            scale_max_to_mos: scale_to_max_mos,
        }
    }
}

impl SimilarityToQualityMapper for SpeechSimilarityToQualityMapper {
    fn predict_quality(&self, similarity_vector: &[f64]) -> f64 {
        const FIT_PARAMETER_A: f64 = 1.155_945_5;
        const FIT_PARAMETER_B: f64 = 4.685_115_3;
        const FIT_PARAMETER_X0: f64 = 0.765_523_2;
        const FIT_SCALE: f64 = 1.2031409;

        let nsim_mean = similarity_vector.iter().sum::<f64>() / (similarity_vector.len() as f64);
        let mos = math_utils::exponential_from_fit(
            nsim_mean,
            FIT_PARAMETER_A,
            FIT_PARAMETER_B,
            FIT_PARAMETER_X0,
        );

        let scale = if self.scale_max_to_mos {
            FIT_SCALE
        } else {
            1.0
        };

        (mos * scale).clamp(1.0, 5.0)
    }
}
