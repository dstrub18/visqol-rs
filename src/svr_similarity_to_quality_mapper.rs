use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;
use crate::support_vector_regression_model::SupportVectorRegressionModel;

pub struct SvrSimilarityToQualityMapper {
    model: SupportVectorRegressionModel,
}

impl SvrSimilarityToQualityMapper {
    pub fn new(model_path: &str) -> Self {
        Self {
            model: SupportVectorRegressionModel::init(model_path),
        }
    }
}

impl SimilarityToQualityMapper for SvrSimilarityToQualityMapper {
    fn predict_quality(&self, similarity_vector: &[f64]) -> f32 {
        let solution = self.model.predict(similarity_vector);
        solution.min(5.0).max(1.0)
    }
}
