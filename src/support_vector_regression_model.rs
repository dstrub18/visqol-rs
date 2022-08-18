use ffsvm::{self, DenseSVM, Predict, DenseProblem, Solution};
use std::fs::read_to_string;
use std::convert::TryFrom;
pub struct SupportVectorRegressionModel
{
    model: ffsvm::DenseSVM
}

impl SupportVectorRegressionModel
{
    pub fn init(model_path: &str) -> Self
    {
        let model_description = read_to_string(model_path).expect("failed to read model path!");
        Self
        {
            model: DenseSVM::try_from(model_description.as_str()).expect("Failed to load SVM model")
        }
    }

    pub fn predict(&self, observation: &[f64]) -> f32
    {
        let mut problem = DenseProblem::from(&self.model);
        let features = problem.features();
        
        for (i, element) in observation.iter().enumerate() 
        {
            features[i] = *element as f32;
        }
        self.model.predict_value(&mut problem).expect("Failed to compute prediction");
        let solution = problem.solution();
        let mut score = 0.0;
        if let Solution::Value(s) = solution
        {
            score = s;
        }
        score
    }
}