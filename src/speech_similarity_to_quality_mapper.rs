use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;
use crate::misc_math;
pub struct SpeechSimilarityToQualityMapper
{
    scale_max_to_mos: bool
}

impl SpeechSimilarityToQualityMapper 
{
    pub fn new(scale_max_to_mos: bool) -> Self 
    {
        Self
        {
            scale_max_to_mos
        }
    }
}

impl SimilarityToQualityMapper for SpeechSimilarityToQualityMapper
{
    fn predict_quality(&self, similarity_vector: &[f64]) -> f32 
    {
        const FIT_PARAMETER_A: f32 = 1.155_945_5;
        const FIT_PARAMETERB : f32= 4.685_115_3;
        const FIT_PARAMETERX0: f32 = 0.765_523_2;
        const FIT_SCALE: f32 = 1.2031409;
        let nsim_mean = (similarity_vector.iter().sum::<f64>() as f32) / (similarity_vector.len() as f32);
        let mos = misc_math::exponential_from_fit(nsim_mean, FIT_PARAMETER_A, FIT_PARAMETERB, FIT_PARAMETERX0);
    
        let scale = if self.scale_max_to_mos {FIT_SCALE} else {1.0};

        ((mos * scale).max(1.0)).min(5.0)
    }
}

