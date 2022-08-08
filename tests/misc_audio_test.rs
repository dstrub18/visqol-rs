use num::complex::Complex64;
use visqol_rs::misc_audio;
use approx::assert_abs_diff_eq;
#[test]
fn load_as_mono()
{
    let expected_mono_test_sample_rate = 48000;
    let expected_mono_test_num_rows = 131444;
    let expected_mono_test_num_cols = 1;
    let expected_mono_duration = 2.74;
    
    let tolerance = 0.01;

    let signal = misc_audio::load_as_mono("test_data/CA01_01.wav");
    assert_eq!(signal.sample_rate, expected_mono_test_sample_rate);
    assert_eq!(signal.data_matrix.len(), expected_mono_test_num_rows);
    assert_eq!(signal.data_matrix.ndim(), expected_mono_test_num_cols);
    assert_abs_diff_eq!(signal.get_duration(), expected_mono_duration, epsilon=tolerance);
}

#[test]
fn load_stereo()
{
    let expected_stereo_test_sample_rate = 48000;
    let expected_stereo_test_num_rows = 597784;
    let expected_stereo_test_num_cols = 1;
    let expected_stereo_duration = 12.45;
    let tolerance = 0.01;

    let signal = misc_audio::load_as_mono("test_data/conformance_testdata_subset/guitar48_stereo.wav");
    assert_eq!(signal.sample_rate, expected_stereo_test_sample_rate);
    assert_eq!(signal.len() as u32, expected_stereo_test_num_rows);
    assert_eq!(signal.data_matrix.ndim() as u32, expected_stereo_test_num_cols);
    assert_abs_diff_eq!(signal.get_duration(), expected_stereo_duration, epsilon=tolerance);
    assert_abs_diff_eq!(signal[2], -0.000_015_258_789_062_5, epsilon=tolerance);
    assert_abs_diff_eq!(signal[597782], -0.000_259_399_414_062_5, epsilon=tolerance);
}


#[test]
fn test_mirror_spectrum()
{
    let mut some_vec = vec![Complex64{re: 1.0, im: 1.0},Complex64{re: 2.0, im: 2.0},Complex64{re: 3.0, im: 3.0}, Complex64{re: 4.0, im: 4.0}];
    let expected_result = vec![Complex64{re: 1.0, im: 0.0},Complex64{re: 2.0, im: 2.0},Complex64{re: 3.0, im: 3.0}, Complex64{re: 4.0, im: 0.0},Complex64{re: 3.0, im: -3.0},Complex64{re: 2.0, im: -2.0}];

    misc_audio::mirror_spectrum(&mut some_vec);
    assert_eq!(some_vec, expected_result);
}