use approx::assert_abs_diff_eq;
use more_asserts::assert_gt;
use visqol_rs::{visqol_manager::{VisqolManager}, file_path::FilePath};

#[test]
fn regression_test_mono()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/transcoded_CA01_01.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);

    assert_abs_diff_eq!(result.moslqo, 2.00039, epsilon=0.001);
}

#[test]
fn regression_test_stereo()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/guitar48_stereo.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/guitar48_stereo_64kbps_aac.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);

    assert_abs_diff_eq!(result.moslqo, 4.5123244380958774, epsilon=0.00001);
}
#[test]
fn test_identical_stddev_nsim()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/guitar48_stereo.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/guitar48_stereo.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);

    for nsim in result.fnsim {
        assert_eq!(nsim, 1.0);
    }
    for std in result.fstdnsim {
        assert_eq!(std, 0.0);
    }
    for each_fvdegenergy in result.fvdegenergy {
        assert_gt!(each_fvdegenergy, 0.0);
    }

}

#[test]
fn test_non48k_sample_rate()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/non_48k_sample_rate/guitar48_stereo_44100Hz.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/conformance_testdata_subset/non_48k_sample_rate/guitar48_stereo_44100Hz.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let _result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);
}

#[test]
fn test_unscaled_speech_mode()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = true;
    let use_unscaled_speech_mos_mapping = true;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);
    assert_abs_diff_eq!(result.moslqo, 4.1557613014690995, epsilon=0.000001);
}
#[test]
fn test_scaled_speech_mode()
{
    let ref_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav";
    let deg_path = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/test_data/clean_speech/CA01_01.wav";

    let ref_signal_path = FilePath::new(ref_path.to_string());
    let deg_signal_path = FilePath::new(deg_path.to_string());
    
    let use_speech_mode = true;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "/Users/danielstrubig/Documents/CodingProjects/rust/exercises/visqol/visqol-rs/model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(sim_to_qual_model, use_speech_mode, use_unscaled_speech_mos_mapping, search_window);

    let result = visqol.run_from_filepaths(&ref_signal_path, &deg_signal_path);
    assert_abs_diff_eq!(result.moslqo, 5.0, epsilon=0.001);
}