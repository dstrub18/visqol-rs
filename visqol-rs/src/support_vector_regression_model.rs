use ffsvm::{self, DenseFeatures, DenseSVM, Label, Predict};
use std::convert::TryFrom;

/// The default content of the model, embedded in the binary at compile time.
/// The path is specified relative to the current file.
const DEFAULT_MODEL_CONTENT: &str = include_str!("../../model/libsvm_nu_svr_model.txt");

/// Thin wrapper around `ffsvm` to compute a prediction from a support vector machine.
pub struct SupportVectorRegressionModel {
    model: ffsvm::DenseSVM,
}

impl SupportVectorRegressionModel {
    /// Creates a model from a string containing a description in LibSVM format.
    /// That is the main constructor, which is independent of the file system.
    pub fn from_str(model_description: &str) -> Self {
        Self {
            model: DenseSVM::try_from(model_description)
                .expect("Failed to load SVM model from string"),
        }
    }

    /// Creates an instance of the model with standard, built-in weights.
    pub fn default() -> Self {
        Self::from_str(DEFAULT_MODEL_CONTENT)
    }

    /// Given a slice of features, this function produces a single score.
    pub fn predict(&self, observation: &[f64]) -> f64 {
        let mut problem = DenseFeatures::from(&self.model);
        let features = problem.features();

        for (i, element) in observation.iter().enumerate() {
            features[i] = *element as f32;
        }
        self.model
            .predict_value(&mut problem)
            .expect("Failed to compute prediction");
        let solution = problem.label();
        let mut score = 0.0;
        if let Label::Value(s) = solution {
            score = s;
        }
        score as f64
    }
}

#[cfg(test)]
mod tests {
    use super::SupportVectorRegressionModel;
    use approx::assert_abs_diff_eq;
    #[test]
    fn svn_predicts_known_mos() {
        // The test now uses the built-in model, and no longer needs to calculate the relative path.
        let svm = SupportVectorRegressionModel::default();

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