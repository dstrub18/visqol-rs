use ffsvm::{self, DenseProblem, DenseSVM, Predict, Solution};
use std::convert::TryFrom;
use std::fs::read_to_string;

/// Thin wrapper around `ffsvm` to compute a prediction from a support vector machine.
pub struct SupportVectorRegressionModel {
    model: ffsvm::DenseSVM,
}

impl SupportVectorRegressionModel {
    /// Given a path to a `LibSVM` formatted `.txt` file, the model is initialized with its corresponding weights.
    pub fn init(model_path: &str) -> Self {
        let model_description = read_to_string(model_path).expect("failed to read model path!");
        Self {
            model: DenseSVM::try_from(model_description.as_str())
                .expect("Failed to load SVM model"),
        }
    }
    /// Given a slice of features, this function produces a single score.
    pub fn predict(&self, observation: &[f64]) -> f32 {
        let mut problem = DenseProblem::from(&self.model);
        let features = problem.features();

        for (i, element) in observation.iter().enumerate() {
            features[i] = *element as f32;
        }
        self.model
            .predict_value(&mut problem)
            .expect("Failed to compute prediction");
        let solution = problem.solution();
        let mut score = 0.0;
        if let Solution::Value(s) = solution {
            score = s;
        }
        score
    }
}

#[cfg(test)]
mod tests {
    use super::SupportVectorRegressionModel;
    use approx::assert_abs_diff_eq;
    #[test]
    fn svn_predicts_known_mos() {
        let model_path = "model/libsvm_nu_svr_model.txt";
        let svm = SupportVectorRegressionModel::init(model_path);

        // This is the FVNSIM results for a ViSQOL comparison between
        // contrabassoon48_stereo.wav and contrabassoon48_stereo_24kbps_aac.wav
        let observation = vec![
            0.853862, 0.680331, 0.535649, 0.639760, 0.029999, 0.058591, 0.077462, 0.012432,
            0.192035, 0.389230, 0.479403, 0.419914, 0.521414, 0.858340, 0.884218, 0.864682,
            0.868514, 0.845271, 0.850559, 0.877882, 0.903985, 0.887572, 0.920558, 0.920375,
            0.954934, 0.945048, 0.952716, 0.986600, 0.987345, 0.936462, 0.856010, 0.829761,
        ];

        let expected_score = 4.30533;

        let predicted_score = svm.predict(&observation);
        assert_abs_diff_eq!(predicted_score, expected_score, epsilon = 0.00001);
    }
}
