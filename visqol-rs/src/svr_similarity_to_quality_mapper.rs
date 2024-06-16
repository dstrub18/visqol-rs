use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;
use crate::support_vector_regression_model::SupportVectorRegressionModel;

/// Maps a a similarity score to a MOS using support vector regression.
pub struct SvrSimilarityToQualityMapper {
    model: SupportVectorRegressionModel,
}

impl SvrSimilarityToQualityMapper {
    /// Initializes the model's weights with a libSVM formatted file located in `model_path`
    pub fn new(model_path: &str) -> Self {
        Self {
            model: SupportVectorRegressionModel::init(model_path),
        }
    }
}

impl SimilarityToQualityMapper for SvrSimilarityToQualityMapper {
    fn predict_quality(&self, similarity_vector: &[f64]) -> f64 {
        let solution = self.model.predict(similarity_vector);
        solution.clamp(1.0, 5.0)
    }
}
