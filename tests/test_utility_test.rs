use ndarray::Array2;
use num::complex::Complex;
use visqol_rs::test_utility;

#[test]
#[should_panic]
fn test_faulty_dimensions() {
    let a = Array2::<f64>::zeros((2, 2));
    let b = Array2::<f64>::zeros((4, 1));
    test_utility::compare_matrix_dimensions(&a, &b);
}

#[test]
#[should_panic]
fn test_for_different_elements_real() {
    let a = Array2::<f64>::zeros((2, 2));
    let b = Array2::<f64>::ones((2, 2));
    test_utility::compare_real_matrix(&a, &b, 0.00001);
}

#[test]
#[should_panic]
fn test_for_different_elements_complex() {
    let a = Array2::<Complex<f64>>::zeros((2, 2));
    let b = Array2::<Complex<f64>>::ones((2, 2));
    test_utility::compare_complex_matrix(&a, &b, 0.00001);
}

#[test]
fn test_for_identical_real_elements() {
    let a = Array2::<f64>::zeros((2, 2));
    let b = Array2::<f64>::zeros((2, 2));
    test_utility::compare_real_matrix(&a, &b, 0.00001);
}

#[test]
fn test_for_identical_elements_complex() {
    let a = Array2::<Complex<f64>>::zeros((2, 2));
    let b = Array2::<Complex<f64>>::zeros((2, 2));
    test_utility::compare_complex_matrix(&a, &b, 0.00001);
}
