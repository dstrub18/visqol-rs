use ndarray::Array2;
use num::Complex;
use crate::fft_manager::FftManager;
use crate::fast_fourier_transform;
use crate::misc_vector::array2_to_vec;

pub fn calculate_best_lag(signal_1: &Array2<f64>, signal_2: &Array2<f64>)
-> i64  {
    let max_lag = ((signal_1.nrows().max(signal_2.nrows())) - 1) as i64;

    let point_wise_fft_vec = calculate_inverse_fft_pointwise_product(signal_1, signal_2);
    // Negative errors
    let mut corrs = point_wise_fft_vec[point_wise_fft_vec.len() - max_lag as usize..].to_vec();
    // Positive errors
    let mut positives = point_wise_fft_vec[0.. max_lag as usize + 1].to_vec();
    
    corrs.append(&mut positives);



    // Get maximum
    let best_corr = corrs[..]
    .iter()
    .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
    .unwrap().clone();

    // Get maximum index
    let best_corr_idx = corrs.iter().position(|&r| r == best_corr).unwrap();
    
    best_corr_idx as i64 - max_lag
}

pub fn calculate_inverse_fft_pointwise_product(signal_1: &Array2<f64>, signal_2: &Array2<f64>)
-> Vec<f64> {
    let mut signal_1_vec = array2_to_vec(signal_1);
    let mut signal_2_vec = array2_to_vec(signal_2);
    let biggest_vec = if signal_1.nrows() > signal_2.nrows() {signal_1.nrows()} else {signal_2.nrows()};

    if signal_1.nrows() > signal_2.nrows()
    {
        signal_2_vec.resize(biggest_vec, 0.0);
    }
    else if signal_2.nrows() > signal_1.nrows()
    {
        signal_1_vec.resize(biggest_vec, 0.0);
    }

    
    let (_, exp) = frexp((signal_1_vec.len() * 2 - 1) as f32);
    let fft_points = 2usize.pow(exp as u32);
    let mut manager = FftManager::new(fft_points);
    let mut point_wise_product = calculate_fft_pointwise_product(&signal_1_vec, &signal_2_vec, &mut manager, fft_points);
    
    let inverse = fast_fourier_transform::inverse_1d_conj_sym(&mut manager, &mut point_wise_product);

    array2_to_vec(&inverse)
}

pub fn calculate_fft_pointwise_product(signal_1: &Vec<f64>, signal_2: &Vec<f64>, manager: &mut FftManager, fft_points: usize)
-> ndarray::ArrayBase<ndarray::OwnedRepr<Complex<f64>>, ndarray::Dim<[usize; 2]>> {
    let mut signal_2_mat = Array2::from_shape_vec((signal_2.len(), 1), signal_2.clone()).unwrap();
    let mut fft_signal_2 = fast_fourier_transform::forward_1d_from_points(manager, &mut signal_2_mat, fft_points);

    fft_signal_2.iter_mut().for_each(|element|{*element = element.conj()});
    
    let mut signal_1_mat = Array2::from_shape_vec((signal_1.len(), 1), signal_1.clone()).unwrap();
    let fft_signal_1 = fast_fourier_transform::forward_1d_from_points(manager, &mut signal_1_mat, fft_points);
    fft_signal_1 * fft_signal_2
}
pub fn frexp(s : f32) -> (f32, i32) {
    if 0.0 == s {
        return (s, 0);
    } else {
        let lg = s.abs().log2();
        let x = (lg - lg.floor() - 1.0).exp2();
        let exp = lg.floor() + 1.0;
        (s.signum() * x, exp as i32)
    }
}