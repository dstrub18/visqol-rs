pub trait SimilarityToQualityMapper {
    fn predict_quality(&self, similarity_vector: &[f64]) -> f32;
}
