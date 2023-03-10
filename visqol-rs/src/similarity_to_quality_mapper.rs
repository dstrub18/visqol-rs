/// Trait to provide a method for predicting a MOS based on features.
/// Given a feature, the implementations of this trait compute a single score.
pub trait SimilarityToQualityMapper {
    fn predict_quality(&self, features: &[f64]) -> f64;
}
