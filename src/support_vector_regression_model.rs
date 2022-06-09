use ffsvm::{self, DenseSVM, Predict, DenseProblem, Solution};
use std::fs::read_to_string;
pub struct SupportVectorRegressionModel
{
    model: ffsvm::DenseSVM
}

impl SupportVectorRegressionModel
{
    pub fn init(model_path: &str) -> Self
    {
        let model_description = read_to_string(model_path).unwrap();
        Self
        {
            model: DenseSVM::try_from(model_description.as_str()).unwrap()
        }
    }

    pub fn predict(&self, observation: &Vec<f64>) -> Solution
    {
        let mut problem = DenseProblem::from(&self.model);
        let features = problem.features();
        for (i, element) in observation.iter().enumerate() 
        {
            features[i] = *element as f32;  
        }
        self.model.predict_value(&mut problem).unwrap();
        problem.solution()
    }
}