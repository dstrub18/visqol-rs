// This is great so far, Daniel :) You'll wing it.
// Continue with tests here

use ffsvm::Solution;
use visqol_rs::support_vector_regression_model::SupportVectorRegressionModel;
use approx::assert_abs_diff_eq;
#[test]
fn test_svn()
{
    
    let model_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";
    let svm = SupportVectorRegressionModel::init(model_path);
    
    // This is the FVNSIM results for a ViSQOL comparison between
    // contrabassoon48_stereo.wav and contrabassoon48_stereo_24kbps_aac.wav
    let observation = vec![0.853862, 0.680331, 0.535649,
    0.639760, 0.029999, 0.058591, 0.077462, 0.012432, 0.192035, 0.389230,
    0.479403, 0.419914, 0.521414, 0.858340, 0.884218, 0.864682, 0.868514,
    0.845271, 0.850559, 0.877882, 0.903985, 0.887572, 0.920558, 0.920375,
    0.954934, 0.945048, 0.952716, 0.986600, 0.987345, 0.936462, 0.856010,
    0.829761];

    let expected_score = 4.30533;

    let solution = svm.predict(&observation);
    let mut predicted_score = 0.0;
    if let Solution::Value(s) = solution
    {
        predicted_score = s;
    }
    assert_abs_diff_eq!(predicted_score, expected_score, epsilon=0.00001);
    
}