use approx::assert_abs_diff_eq;
use ndarray::{Array2};
use num::complex::Complex;

pub fn compare_matrix_dimensions<T>(matrix_a: &Array2::<T>, matrix_b: &Array2::<T>)
{
    assert!(matrix_a.ncols() == matrix_b.ncols() && matrix_a.nrows() == matrix_b.nrows(), "Matrix dimensions do not match!");
}

pub fn compare_real_matrix(matrix_a: &Array2::<f64>, matrix_b: &Array2::<f64>, tolerance: f64)
{
    compare_matrix_dimensions(matrix_a, matrix_b);
    for ( elements_a, elements_b) in matrix_a.iter().zip(matrix_b)
    {
        assert_abs_diff_eq!(elements_a, elements_b, epsilon=tolerance);
    }
}

pub fn compare_complex_matrix(matrix_a: &Array2::<Complex<f64>>, matrix_b: &Array2::<Complex<f64>>, tolerance: f64)
{
    compare_matrix_dimensions(matrix_a, matrix_b);
    for (elements_a, elements_b) in matrix_a.iter().zip(matrix_b)
    {
        assert_abs_diff_eq!(elements_a.re, elements_b.re, epsilon=tolerance);
        assert_abs_diff_eq!(elements_a.im, elements_b.im, epsilon=tolerance);
    }
}

pub fn compare_complex_vec(vec_a: &Vec<Complex<f64>>, vec_b: &Vec<Complex<f64>>, tolerance: f64)
{
    assert_eq!(vec_a.len(), vec_b.len());
    vec_a.iter()
         .zip(vec_b)
         .for_each
         (|(a, b)|
         {
            assert_abs_diff_eq!(a.re, b.re, epsilon=tolerance);
            assert_abs_diff_eq!(a.im, b.im, epsilon=tolerance);
         });
}

pub fn compare_real_vec(vec_a: &Vec<f64>, vec_b: &Vec<f64>, tolerance: f64)
{
    assert_eq!(vec_a.len(), vec_b.len());
    vec_a.iter()
         .zip(vec_b)
         .for_each
         (|(a, b)|
         {
            assert_abs_diff_eq!(a, b, epsilon=tolerance);
         });
}