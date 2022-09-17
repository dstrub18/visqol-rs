use approx::assert_abs_diff_eq;
use ndarray::Array2;
use num::complex::Complex;

pub fn compare_matrix_dimensions<T>(matrix_a: &Array2<T>, matrix_b: &Array2<T>) {
    assert!(
        matrix_a.ncols() == matrix_b.ncols() && matrix_a.nrows() == matrix_b.nrows(),
        "Matrix dimensions do not match!"
    );
}

pub fn compare_real_matrix(matrix_a: &Array2<f64>, matrix_b: &Array2<f64>, tolerance: f64) {
    compare_matrix_dimensions(matrix_a, matrix_b);
    for (_idx, (elements_a, elements_b)) in matrix_a.iter().zip(matrix_b).enumerate() {
        assert_abs_diff_eq!(elements_a, elements_b, epsilon = tolerance);
    }
}

pub fn compare_complex_matrix(
    matrix_a: &Array2<Complex<f64>>,
    matrix_b: &Array2<Complex<f64>>,
    tolerance: f64,
) {
    compare_matrix_dimensions(matrix_a, matrix_b);
    for (elements_a, elements_b) in matrix_a.iter().zip(matrix_b) {
        assert_abs_diff_eq!(elements_a.re, elements_b.re, epsilon = tolerance);
        assert_abs_diff_eq!(elements_a.im, elements_b.im, epsilon = tolerance);
    }
}

pub fn compare_complex_vec(vec_a: &[Complex<f64>], vec_b: &[Complex<f64>], tolerance: f64) {
    assert_eq!(vec_a.len(), vec_b.len());
    vec_a.iter().zip(vec_b).for_each(|(a, b)| {
        assert_abs_diff_eq!(a.re, b.re, epsilon = tolerance);
        assert_abs_diff_eq!(a.im, b.im, epsilon = tolerance);
    });
}

pub fn compare_real_vec(vec_a: &[f64], vec_b: &[f64], tolerance: f64) {
    assert_eq!(vec_a.len(), vec_b.len());
    vec_a.iter().zip(vec_b).for_each(|(a, b)| {
        assert_abs_diff_eq!(a, b, epsilon = tolerance);
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;
    use num::complex::Complex;

    #[test]
    #[should_panic]
    fn test_faulty_dimensions() {
        let a = Array2::<f64>::zeros((2, 2));
        let b = Array2::<f64>::zeros((4, 1));
        compare_matrix_dimensions(&a, &b);
    }

    #[test]
    #[should_panic]
    fn test_for_different_elements_real() {
        let a = Array2::<f64>::zeros((2, 2));
        let b = Array2::<f64>::ones((2, 2));
        compare_real_matrix(&a, &b, 0.00001);
    }

    #[test]
    #[should_panic]
    fn test_for_different_elements_complex() {
        let a = Array2::<Complex<f64>>::zeros((2, 2));
        let b = Array2::<Complex<f64>>::ones((2, 2));
        compare_complex_matrix(&a, &b, 0.00001);
    }

    #[test]
    fn test_for_identical_real_elements() {
        let a = Array2::<f64>::zeros((2, 2));
        let b = Array2::<f64>::zeros((2, 2));
        compare_real_matrix(&a, &b, 0.00001);
    }

    #[test]
    fn test_for_identical_elements_complex() {
        let a = Array2::<Complex<f64>>::zeros((2, 2));
        let b = Array2::<Complex<f64>>::zeros((2, 2));
        compare_complex_matrix(&a, &b, 0.00001);
    }
}
