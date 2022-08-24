use crate::misc_math;
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
    fn predict_quality(&self, similarity_vector: &[f64]) -> f32 {
        const FIT_PARAMETER_A: f32 = 1.155_945_5;
        const FIT_PARAMETER_B: f32 = 4.685_115_3;
        const FIT_PARAMETER_X0: f32 = 0.765_523_2;
        const FIT_SCALE: f32 = 1.2031409;

        let nsim_mean =
            (similarity_vector.iter().sum::<f64>() as f32) / (similarity_vector.len() as f32);
        let mos = misc_math::exponential_from_fit(
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

        ((mos * scale).max(1.0)).min(5.0)
    }
}
