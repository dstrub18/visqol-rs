use ndarray::{Array, ShapeBuilder, Array2};
use visqol_rs::convolution_2d::*;

#[test]
fn test_2d_convolution()
{
    let w = vec![0.0113033910173052, 0.0838251475442633,
    0.0113033910173052, 0.0838251475442633, 0.619485845753726,
    0.0838251475442633, 0.0113033910173052, 0.0838251475442633,
    0.0113033910173052];
    let window = Array::from_shape_vec((3,3).f(), w).unwrap();

    let m = vec![40.0392, 43.3409, 39.5270, 41.1731, 41.3591, 42.6852,
    45.2083, 45.7769, 39.9689, 43.6190, 41.0119, 40.4244, 41.5932, 43.6027,
    42.6204, 43.0624, 42.2610, 42.4725, 43.4258, 42.9079];
    let mut matrix = Array::from_shape_vec((5,4).f(), m).unwrap();

    let result = perform_valid_2d_conv_with_boundary(&window, &mut matrix);
    
    let r = vec![40.6634, 42.8407, 40.6395, 41.0129, 41.5407, 42.4677,
    44.2760, 44.2031, 41.2263, 42.9752, 41.3784, 41.2656, 42.1388, 43.0366,
    42.8042, 42.7613, 42.1817, 42.4590, 43.2709, 42.9377];
    let expected_result = Array2::<f64>::from_shape_vec((5,4).f(),r ).unwrap();

    use approx::assert_abs_diff_eq;
    for i in 0..result.nrows()
    {
        for j in 0..result.ncols()
        {
            assert_abs_diff_eq!(result[(i,j)], expected_result[(i,j)], epsilon=0.001);
        }
    }
}


#[test]
fn test_perform_padding()
{
    let m = vec![40.0392, 43.3409, 39.5270, 41.1731, 41.3591, 42.6852,
    45.2083, 45.7769, 39.9689, 43.6190, 41.0119, 40.4244, 41.5932, 43.6027,
    42.6204, 43.0624, 42.2610, 42.4725, 43.4258, 42.9079];
    let mut matrix = Array::from_shape_vec((5,4).f(), m).unwrap();
    let result = add_matrix_boundary(&mut matrix);
    
    let mut r = Vec::new();
    for i in 0..result.dim().0
    {
        for j in 0..result.dim().1
        {
            r.push(result[(i,j)]);
        }
    }

    let expected_result = vec![40.0392, 40.0392, 42.6852, 41.0119, 43.0624, 43.0624,
    40.0392, 40.0392, 42.6852, 41.0119, 43.0624, 43.0624,
    43.3409, 43.3409, 45.2083, 40.4244, 42.261, 42.261,
    39.527, 39.527, 45.7769, 41.5932, 42.4725, 42.4725,
    41.1731, 41.1731, 39.9689, 43.6027, 43.4258, 43.4258,
    41.3591, 41.3591, 43.619, 42.6204, 42.9079, 42.9079,
    41.3591, 41.3591, 43.619, 42.6204, 42.9079, 42.9079];

    assert_eq!(r, expected_result);
}

#[test]
fn test_copying_with_zeros()
{
    let m = vec![
    40.0392, 43.3409, 39.5270, 41.1731,
    41.3591, 42.6852, 45.2083, 45.7769,
    39.9689, 43.6190,41.0119, 40.4244,
    41.5932, 43.6027,42.6204,43.0624,
    42.2610, 42.4725, 43.4258, 42.9079];
    let matrix = Array::from_shape_vec((5,4).f(), m).unwrap();
    let result = copy_matrix_within_padding(&matrix,1,1,1,1);

    let er = vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    0.0, 40.0392, 42.6852, 41.0119, 43.0624, 0.0,
    0.0, 43.3409, 45.2083, 40.4244, 42.261, 0.0,
    0.0, 39.527, 45.7769, 41.5932, 42.4725, 0.0,
    0.0, 41.1731, 39.9689, 43.6027, 43.4258,0.0,
    0.0, 41.3591, 43.619, 42.6204, 42.9079, 0.0,
    0.0, 0.0, 0.0, 0.0, 0.0, 0.0,];

    // Extracted from cpp, with armadillo using column memory layout.
    let erm = Array::from_shape_vec((7,6), er).unwrap();

    for (r_elem, erm_elem) in result.iter().zip(&erm)
    {
        assert_eq!(r_elem, erm_elem);
    }
}

