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

    let signal = misc_audio::load_as_mono(String::from("test_data/CA01_01.wav"));
    assert_eq!(signal.sample_rate, expected_mono_test_sample_rate);
    assert_eq!(signal.data_matrix.len(), expected_mono_test_num_rows);
    assert_eq!(signal.data_matrix.nrows(), expected_mono_test_num_rows);
    assert_eq!(signal.data_matrix.ncols(), expected_mono_test_num_cols);
    assert_abs_diff_eq!(signal.get_duration(), expected_mono_duration, epsilon=tolerance);
}

#[test]
#[ignore = "Wav headers with variable lengths are not supported just yet."]
fn load_stereo()
{
    let _expected_stereo_test_sample_rate = 48000;
    let _expected_stereo_test_num_rows = 597784;
    let _expected_stereo_test_num_cols = 1;
    let _expected_stereo_duration = 12.45;
    let _tolerance = 0.01;

    let _signal = misc_audio::load_as_mono(String::from("test_data/conformance_testdata_subset/guitar48_stereo.wav"));
}


#[test]
fn test_mirror_spectrum()
{
    let mut some_vec = vec![Complex64{re: 1.0, im: 1.0},Complex64{re: 2.0, im: 2.0},Complex64{re: 3.0, im: 3.0}, Complex64{re: 4.0, im: 4.0}];
    let expected_result = vec![Complex64{re: 1.0, im: 0.0},Complex64{re: 2.0, im: 2.0},Complex64{re: 3.0, im: 3.0}, Complex64{re: 4.0, im: 0.0},Complex64{re: 3.0, im: -3.0},Complex64{re: 2.0, im: -2.0}];

    misc_audio::mirror_spectrum(&mut some_vec);
    assert_eq!(some_vec, expected_result);
}