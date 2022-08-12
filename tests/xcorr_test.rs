use visqol_rs::{xcorr};
use ndarray::{Array1};

#[test]

fn test_best_lag_same_length()
{
    let ref_signal = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag2 = vec![1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1];
    
    assert_eq!(deg_signal_lag2.len(), 14);
    let ref_signal_mat = Array1::from_vec(ref_signal);
    let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
    assert_eq!(ref_signal_mat.len(), deg_signal_lag2_mat.len());
    let best_lag = xcorr::calculate_best_lag(&ref_signal_mat, &deg_signal_lag2_mat).unwrap();
    
    let expected_result = 2;
    assert_eq!(best_lag, expected_result);
}

#[test]
fn test_best_lag_ref_shorter()
{
    let ref_signal = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag2 = vec![1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2, 2.1, 2.0];

    assert!( ref_signal.len() < deg_signal_lag2.len());
    let ref_signal_mat = Array1::from_vec(ref_signal);
    let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
    let best_lag = xcorr::calculate_best_lag(&ref_signal_mat, &deg_signal_lag2_mat).unwrap();

    let expected_result = 2;
    assert_eq!(best_lag, expected_result);
}

#[test]
fn test_best_lag_ref_longer()
{
    let ref_signal = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag2 = vec![1.2, 0.1, -3.3, 0.1, 1.1, 2.2, 2.1, 7.1, 8.3, 6.8, 2.4, 2.2, 2.2];
    assert!( ref_signal.len() > deg_signal_lag2.len());

    let ref_signal_mat = Array1::from_vec(ref_signal);
    let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
    let best_lag = xcorr::calculate_best_lag(&ref_signal_mat, &deg_signal_lag2_mat).unwrap();

    let expected_result = 2;
    assert_eq!(best_lag, expected_result);
}
#[test]
fn test_negative_best_lag()
{
    let ref_signal = vec![2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0, 2.0, 2.0];
    let deg_signal_lag2 = vec![2.0, 2.0, 2.0, 2.0, 1.0, 0.1, -3.0, 0.1, 1.0, 2.0, 2.0, 6.0, 8.0, 6.0];

    let ref_signal_mat = Array1::from_vec(ref_signal);
    let deg_signal_lag2_mat = Array1::from_vec(deg_signal_lag2);
    let best_lag = xcorr::calculate_best_lag(&ref_signal_mat, &deg_signal_lag2_mat).unwrap();

    let expected_result = -2;
    assert_eq!(best_lag, expected_result);
}


#[test]
fn test_frexp()
{
    let (_,result) = xcorr::frexp(27.0f32);
    let expected_result = 5;

    assert_eq!(result, expected_result);
}
