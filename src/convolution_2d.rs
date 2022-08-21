use ndarray::{Array2, ShapeBuilder};

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
                    if filter_index > 0 {
                        filter_index -= 1;
                    }
                }
            }
            out_matrix[(o_row, o_col)] = sum;
        }
    }
    out_matrix
}

pub fn flatten_matrix(input_matrix: &Array2<f64>) -> Vec<f64> {
    let mut res = Vec::<f64>::new();
    for i in 0..input_matrix.nrows() {
        for j in 0..input_matrix.ncols() {
            res.push(input_matrix[(i, j)]);
        }
    }
    res
}

pub fn add_matrix_boundary(input_matrix: &mut Array2<f64>) -> Array2<f64> {
    let mut output_matrix = copy_matrix_within_padding(input_matrix, 1, 1, 1, 1);

    let _a = output_matrix.row_mut(0);
    for i in 0..output_matrix.ncols() {
        output_matrix.row_mut(0)[i] = output_matrix.row(1)[i];
        output_matrix.row_mut(output_matrix.nrows() - 1)[i] =
            output_matrix.row(output_matrix.nrows() - 2)[i];
    }
    let _nr = output_matrix.nrows();

    for i in 0..output_matrix.nrows() {
        output_matrix.column_mut(0)[i] = output_matrix.column_mut(1)[i];
        output_matrix.column_mut(output_matrix.ncols() - 1)[i] =
            output_matrix.column(output_matrix.ncols() - 2)[i];
    }
    output_matrix
}

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
