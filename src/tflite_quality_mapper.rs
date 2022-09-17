use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;
use tract_onnx::prelude::*;

pub struct TfLiteQualityMapper {
    model_path: String,
    _num_frequency_bands: usize,
}

impl TfLiteQualityMapper {

    const _TAU_IDX: usize  = 84;
    pub fn new(model_path: &str, _num_frequency_bands: usize) -> Self {
        Self {
            model_path: String::from(model_path),
            _num_frequency_bands,
        }
    }
}

impl SimilarityToQualityMapper for TfLiteQualityMapper
{
    fn predict_quality(&self, _features: &[f64]) -> f32 
    {
        // Model must have 85 inputs.
        // Tau might be index 84?
        let model = tract_onnx::onnx()
        // load the model
        .model_for_path(&self.model_path).unwrap()
        .into_runnable().unwrap();
        let input: Tensor  = tract_ndarray::Array4::from_shape_vec((85, 85, 85, 85), vec![1.0; 85 * 4]).unwrap().into();
        let t = tvec![input];
        let _result = model.run(t).unwrap();

        0.0
    }
}

#[cfg(test)]
mod tests
{
    use crate::similarity_to_quality_mapper::SimilarityToQualityMapper;

    use super::TfLiteQualityMapper;

    #[test]
    #[ignore = "Feature not complete"]
    fn test_lattice()
    {
        let tf = TfLiteQualityMapper::new("/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/model.onnx", 21);
        tf.predict_quality(&[0.0]);
    }
}