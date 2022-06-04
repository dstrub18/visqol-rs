use ndarray::Array2;
use visqol_rs::alignment;
use visqol_rs::audio_signal::AudioSignal;
use visqol_rs::xcorr;

#[test]
fn align_signal_with_positive_lag()
{
    let ref_signal_vec = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag_2_vec = vec![1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1];

    let ref_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec).unwrap();
    let deg_signal_mat = Array2::from_shape_vec((deg_signal_lag_2_vec.len(), 1), deg_signal_lag_2_vec).unwrap();
    
    let best_lag_positive2 = 2;
    let zero_lag = 0;
    let ref_signal = AudioSignal::new(ref_signal_mat, 1);
    let deg_signal = AudioSignal::new(deg_signal_mat, 1);

    let initial_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &deg_signal.data_matrix);
    assert_eq!(initial_lag, best_lag_positive2);

    let (new_deg_signal, _) = alignment::globally_align(&ref_signal, &deg_signal);
    let final_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &new_deg_signal.data_matrix);

    assert_eq!(zero_lag, final_lag);

    assert_eq!(ref_signal.data_matrix.nrows() + best_lag_positive2 as usize, new_deg_signal.data_matrix.nrows());
}

#[test]
fn align_signal_with_negative_lag()
{
    let ref_signal_vec = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag_negative_2_vec = vec![2.0, 2.0, 2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0];

    let ref_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec).unwrap();
    let deg_signal_mat = Array2::from_shape_vec((deg_signal_lag_negative_2_vec.len(), 1), deg_signal_lag_negative_2_vec).unwrap();
    
    let best_lag_negative2 = -2;
    let zero_lag = 0;
    let ref_signal = AudioSignal::new(ref_signal_mat, 1);
    let deg_signal = AudioSignal::new(deg_signal_mat, 1);

    let initial_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &deg_signal.data_matrix);
    assert_eq!(initial_lag, best_lag_negative2);

    let (new_deg_signal, _) = alignment::globally_align(&ref_signal, &deg_signal);
    let final_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &new_deg_signal.data_matrix);

    assert_eq!(zero_lag, final_lag);

    assert_eq!(ref_signal.data_matrix.nrows() as i64, new_deg_signal.data_matrix.nrows() as i64 - best_lag_negative2);
}

#[test]
fn align_signal_with_no_lag()
{
    let ref_signal_vec = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];

    let ref_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec.clone()).unwrap();
    let deg_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec).unwrap();
    
    let deg_signal_init_size = ref_signal_mat.nrows();
    let zero_lag = 0;
    let ref_signal = AudioSignal::new(ref_signal_mat, 1);
    let deg_signal = AudioSignal::new(deg_signal_mat, 1);

    let initial_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &deg_signal.data_matrix);
    assert_eq!(initial_lag, zero_lag);

    let (new_deg_signal, _) = alignment::globally_align(&ref_signal, &deg_signal);
    let final_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &new_deg_signal.data_matrix);

    assert_eq!(zero_lag, final_lag);

    assert_eq!(deg_signal_init_size, new_deg_signal.data_matrix.nrows());
}

#[test]
fn align_and_truncate_signal_with_negative_lag()
{
    let ref_signal_vec = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag_negative_2_vec = vec![2.0, 2.0, 2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0];

    let ref_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec).unwrap();
    let deg_signal_mat = Array2::from_shape_vec((deg_signal_lag_negative_2_vec.len(), 1), deg_signal_lag_negative_2_vec).unwrap();
    
    let best_lag_negative2 = -2;
    let ref_signal = AudioSignal::new(ref_signal_mat, 1);
    let deg_signal = AudioSignal::new(deg_signal_mat, 1);
    let original_ref_duration = ref_signal.get_duration();
    let initial_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &deg_signal.data_matrix);
    assert_eq!(initial_lag, best_lag_negative2);

    let (new_ref_signal, new_deg_signal, lag) = alignment::align_and_truncate(&ref_signal, &deg_signal);
    assert_eq!(lag, best_lag_negative2 as f64);

    assert_eq!(original_ref_duration + best_lag_negative2 as f64, new_ref_signal.get_duration());
    assert_eq!(original_ref_duration + best_lag_negative2 as f64, new_deg_signal.get_duration());

}

#[test]
fn align_and_truncate_signal_with_positive_lag()
{
    let ref_signal_vec = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag_2_vec = vec![1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1];

    let ref_signal_mat = Array2::from_shape_vec((ref_signal_vec.len(), 1), ref_signal_vec).unwrap();
    let deg_signal_mat = Array2::from_shape_vec((deg_signal_lag_2_vec.len(), 1), deg_signal_lag_2_vec).unwrap();
    
    let best_lag_positive2 = 2;
    let ref_signal = AudioSignal::new(ref_signal_mat, 1);
    let deg_signal = AudioSignal::new(deg_signal_mat, 1);
    let original_ref_duration = ref_signal.get_duration();
    
    let initial_lag = xcorr::calculate_best_lag(&ref_signal.data_matrix, &deg_signal.data_matrix);
    assert_eq!(initial_lag, best_lag_positive2);

    let (new_ref_signal, new_deg_signal, lag) = alignment::align_and_truncate(&ref_signal, &deg_signal);
    assert_eq!(lag, best_lag_positive2 as f64);

    assert_eq!(original_ref_duration - best_lag_positive2 as f64, new_ref_signal.get_duration());
    assert_eq!(original_ref_duration - best_lag_positive2 as f64, new_deg_signal.get_duration());
}