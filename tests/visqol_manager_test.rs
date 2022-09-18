use approx::assert_abs_diff_eq;
use more_asserts::assert_gt;
use visqol_rs::visqol_manager::VisqolManager;

#[test]
fn regression_test_mono() {
    let (ref_path, deg_path) = get_paths_to_speech_files();
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    // We can leave this empty, since svm model is not used.
    let sim_to_qual_model = "model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    let result = visqol.run(ref_path, deg_path).unwrap();

    assert_abs_diff_eq!(result.moslqo, 2.00039, epsilon = 0.001);
}

#[test]
fn regression_test_stereo() {
    let (ref_path, deg_path) = get_paths_to_guitar_files();
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    let result = visqol.run(ref_path, deg_path).unwrap();

    assert_abs_diff_eq!(result.moslqo, 4.512_324_438_095_877, epsilon = 0.000001);
}
#[test]
fn test_identical_stddev_nsim() {
    let (ref_path, _) = get_paths_to_guitar_files();
    let deg_path = &(*ref_path);
    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    let result = visqol.run(ref_path, deg_path).unwrap();

    for nsim in result.fvnsim {
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
fn test_non48k_sample_rate() {
    let ref_path = "test_data/conformance_testdata_subset/non_48k_sample_rate/guitar48_stereo_44100Hz.wav";
    let deg_path = &(*ref_path);

    let use_speech_mode = false;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    visqol.run(ref_path, deg_path).unwrap();
}

#[test]
fn test_unscaled_speech_mode() {
    let (ref_path, _) = get_paths_to_speech_files();
    let deg_path = &(*ref_path);

    let use_speech_mode = true;
    let use_unscaled_speech_mos_mapping = true;
    let search_window = 60;
    let sim_to_qual_model = "model/libsvm_nu_svr_model.txt";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    let result = visqol.run(ref_path, deg_path).unwrap();
    assert_abs_diff_eq!(result.moslqo, 4.1557613014690995, epsilon = 0.000001);
}
#[test]
fn test_scaled_speech_mode() {
    let (ref_path, _) = get_paths_to_speech_files();
    let deg_path = &(*ref_path);

    let use_speech_mode = true;
    let use_unscaled_speech_mos_mapping = false;
    let search_window = 60;
    let sim_to_qual_model = "";

    let mut visqol = VisqolManager::new(
        sim_to_qual_model,
        use_speech_mode,
        use_unscaled_speech_mos_mapping,
        search_window,
    );

    let result = visqol.run(ref_path, deg_path).unwrap();
    assert_abs_diff_eq!(result.moslqo, 5.0, epsilon = 0.001);
}

fn get_paths_to_speech_files()
-> (&'static str, &'static str) {
    ("test_data/clean_speech/CA01_01.wav", "test_data/clean_speech/transcoded_CA01_01.wav")
}

fn get_paths_to_guitar_files()
-> (&'static str, &'static str) {
    ("test_data/conformance_testdata_subset/guitar48_stereo.wav", "test_data/conformance_testdata_subset/guitar48_stereo_64kbps_aac.wav")
}
