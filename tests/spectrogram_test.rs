use ndarray::{Array2};
use visqol_rs::{spectrogram::Spectrogram, test_utility};
use approx::assert_abs_diff_eq;

const TOLERANCE: f64 = 0.0001;
const MIN_ELEM : f64 = -53.2;
const FLOOR : f64 = 0.1;

#[test]
fn convert_to_db_test()
{
    let elements = Array2::<f64>::from_shape_vec((10, 1), vec![10.21, -4.63, 0.54,
    87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76]).unwrap();

    let elements_db_scaled = Array2::<f64>::from_shape_vec((10, 1), vec![10.0903, 6.6558,
    -2.6761, 19.4438, -11.8709, -156.5356, 17.2591, 9.3952, -156.5356, 4.4091]).unwrap();

    
    let mut spectrogram = Spectrogram::new(elements, vec![]);
    spectrogram.convert_to_db();

    test_utility::compare_real_matrix(&spectrogram.data, &elements_db_scaled, TOLERANCE);
}

#[test]
fn minimum_test()
{
    let elements = Array2::<f64>::from_shape_vec((10, 1), vec![10.21, -4.63, 0.54,
    87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76]).unwrap();

    let spectrogram = Spectrogram::new(elements, vec![]);

    assert_abs_diff_eq!(spectrogram.get_minimum(), MIN_ELEM, epsilon = TOLERANCE);
}

#[test]
fn subtract_floor_test()
{
    let elements = Array2::<f64>::from_shape_vec((10, 1), vec![10.21, -4.63, 0.54,
    87.98, 0.065, 0.0, MIN_ELEM, 8.7, 0.0, -2.76]).unwrap();
    
    let elements_floor_subtracted = Array2::<f64>::from_shape_vec((10, 1), vec![10.21 - FLOOR,
    -4.63 - FLOOR, 0.54 - FLOOR, 87.98 - FLOOR, 0.065 - FLOOR, 0.0 - FLOOR,
    MIN_ELEM - FLOOR, 8.7 - FLOOR, 0.0 - FLOOR, -2.76 - FLOOR]).unwrap();

    let mut spectrogram = Spectrogram::new(elements, vec![]);

    spectrogram.subtract_floor(FLOOR);
    test_utility::compare_real_matrix(&spectrogram.data, &elements_floor_subtracted, TOLERANCE);
    

}

#[test]
fn subtract_test()
{

}