use ndarray::{Array2, ShapeBuilder};

/// Computes the convolution of `input_matrix` with `fir_filter`
pub fn perform_valid_2d_conv_with_boundary(
    fir_filter: &Array2<f64>,
    input_matrix: &mut Array2<f64>,
) -> Array2<f64> {
    let padded_matrix = add_matrix_boundary(input_matrix);
    let padded_flattened_matrix = flatten_matrix(&padded_matrix);

    let i_r_c = padded_matrix.nrows();
    let i_c_c = padded_matrix.ncols();
    let f_r_c = fir_filter.nrows();
    let f_c_c = fir_filter.ncols();
    let o_r_c = i_r_c - f_r_c + 1;
    let o_c_c = i_c_c - f_c_c + 1;
    let filter_size = f_r_c * f_c_c;

    let flattened_filter = flatten_matrix(fir_filter);

    let mut out_matrix = Array2::<f64>::zeros((o_r_c, o_c_c).f());

    for o_row in 0..o_r_c {
        for o_col in 0..o_c_c {
            let mut sum = 0.0f64;
            let mut filter_index = filter_size - 1;

            for f_col in 0..f_c_c {
                for f_row in 0..f_r_c {
                    let idx = ((f_row + o_row) * i_c_c) + f_col + o_col;
                    sum += padded_flattened_matrix[idx] * flattened_filter[filter_index];
                    filter_index = filter_index.saturating_sub(1);
                }
            }
            out_matrix[(o_row, o_col)] = sum;
        }
    }
    out_matrix
}

fn flatten_matrix(input_matrix: &Array2<f64>) -> Vec<f64> {
    let mut res = Vec::<f64>::new();
    for i in 0..input_matrix.nrows() {
        for j in 0..input_matrix.ncols() {
            res.push(input_matrix[(i, j)]);
        }
    }
    res
}

/// Compute zero-padded matrix and fill zero-padded boundaries with the adjacent non-zero rows and columns
pub fn add_matrix_boundary(input_matrix: &mut Array2<f64>) -> Array2<f64> {
    let mut output_matrix = copy_matrix_within_padding(input_matrix, 1, 1, 1, 1);

    for i in 0..output_matrix.ncols() {
        output_matrix.row_mut(0)[i] = output_matrix.row(1)[i];
        output_matrix.row_mut(output_matrix.nrows() - 1)[i] =
            output_matrix.row(output_matrix.nrows() - 2)[i];
    }

    for i in 0..output_matrix.nrows() {
        output_matrix.column_mut(0)[i] = output_matrix.column_mut(1)[i];
        output_matrix.column_mut(output_matrix.ncols() - 1)[i] =
            output_matrix.column(output_matrix.ncols() - 2)[i];
    }
    output_matrix
}

/// Returns a copy of `input matrix` which is zero-padded by the specified amounts.
pub fn copy_matrix_within_padding(
    input_matrix: &Array2<f64>,
    row_prepad_amount: usize,
    row_postpad_amount: usize,
    col_prepad_amount: usize,
    col_postpad_amount: usize,
) -> Array2<f64> {
    let mut output_matrix = Array2::<f64>::zeros((
        row_prepad_amount + input_matrix.nrows() + row_postpad_amount,
        col_prepad_amount + input_matrix.ncols() + col_postpad_amount,
    ));

    for row_i in 0..input_matrix.nrows() {
        for col_i in 0..input_matrix.ncols() {
            output_matrix[(row_i + row_prepad_amount, col_i + col_prepad_amount)] =
                input_matrix[(row_i, col_i)];
        }
    }
    output_matrix
}

#[cfg(test)]
mod tests {
    use ndarray::{Array, Array2, ShapeBuilder};

    use super::*;

    #[test]
    fn convolve_with_window() {
        let w = vec![
            0.0113033910173052,
            0.0838251475442633,
            0.0113033910173052,
            0.0838251475442633,
            0.619485845753726,
            0.0838251475442633,
            0.0113033910173052,
            0.0838251475442633,
            0.0113033910173052,
        ];
        let window = Array::from_shape_vec((3, 3).f(), w).unwrap();

        let m = vec![
            40.0392, 43.3409, 39.5270, 41.1731, 41.3591, 42.6852, 45.2083, 45.7769, 39.9689,
            43.6190, 41.0119, 40.4244, 41.5932, 43.6027, 42.6204, 43.0624, 42.2610, 42.4725,
            43.4258, 42.9079,
        ];
        let mut matrix = Array::from_shape_vec((5, 4).f(), m).unwrap();

        let result = perform_valid_2d_conv_with_boundary(&window, &mut matrix);

        let r = vec![
            40.6634, 42.8407, 40.6395, 41.0129, 41.5407, 42.4677, 44.2760, 44.2031, 41.2263,
            42.9752, 41.3784, 41.2656, 42.1388, 43.0366, 42.8042, 42.7613, 42.1817, 42.4590,
            43.2709, 42.9377,
        ];
        let expected_result = Array2::<f64>::from_shape_vec((5, 4).f(), r).unwrap();

        use approx::assert_abs_diff_eq;
        for i in 0..result.nrows() {
            for j in 0..result.ncols() {
                assert_abs_diff_eq!(result[(i, j)], expected_result[(i, j)], epsilon = 0.001);
            }
        }
    }

    #[test]
    fn perform_padding() {
        let m = vec![
            40.0392, 43.3409, 39.5270, 41.1731, 41.3591, 42.6852, 45.2083, 45.7769, 39.9689,
            43.6190, 41.0119, 40.4244, 41.5932, 43.6027, 42.6204, 43.0624, 42.2610, 42.4725,
            43.4258, 42.9079,
        ];
        let mut matrix = Array::from_shape_vec((5, 4).f(), m).unwrap();
        let result = add_matrix_boundary(&mut matrix);

        let mut r = Vec::new();
        for i in 0..result.dim().0 {
            for j in 0..result.dim().1 {
                r.push(result[(i, j)]);
            }
        }

        let expected_result = vec![
            40.0392, 40.0392, 42.6852, 41.0119, 43.0624, 43.0624, 40.0392, 40.0392, 42.6852,
            41.0119, 43.0624, 43.0624, 43.3409, 43.3409, 45.2083, 40.4244, 42.261, 42.261, 39.527,
            39.527, 45.7769, 41.5932, 42.4725, 42.4725, 41.1731, 41.1731, 39.9689, 43.6027,
            43.4258, 43.4258, 41.3591, 41.3591, 43.619, 42.6204, 42.9079, 42.9079, 41.3591,
            41.3591, 43.619, 42.6204, 42.9079, 42.9079,
        ];

        assert_eq!(r, expected_result);
    }

    #[test]
    fn copy_with_zeros() {
        let m = vec![
            40.0392, 43.3409, 39.5270, 41.1731, 41.3591, 42.6852, 45.2083, 45.7769, 39.9689,
            43.6190, 41.0119, 40.4244, 41.5932, 43.6027, 42.6204, 43.0624, 42.2610, 42.4725,
            43.4258, 42.9079,
        ];
        let matrix = Array::from_shape_vec((5, 4).f(), m).unwrap();
        let result = copy_matrix_within_padding(&matrix, 1, 1, 1, 1);

        let er = vec![
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 40.0392, 42.6852, 41.0119, 43.0624, 0.0, 0.0,
            43.3409, 45.2083, 40.4244, 42.261, 0.0, 0.0, 39.527, 45.7769, 41.5932, 42.4725, 0.0,
            0.0, 41.1731, 39.9689, 43.6027, 43.4258, 0.0, 0.0, 41.3591, 43.619, 42.6204, 42.9079,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];

        // Extracted from cpp, with armadillo using column memory layout.
        let erm = Array::from_shape_vec((7, 6), er).unwrap();

        for (r_elem, erm_elem) in result.iter().zip(&erm) {
            assert_eq!(r_elem, erm_elem);
        }
    }
}
