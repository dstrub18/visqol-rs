pub trait SimilarityToQualityMapper
{
    fn predict_quality(&self, similarity_vector: &Vec<f64>) -> f32;
}