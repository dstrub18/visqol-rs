use ndarray::Array2;
use num::complex::Complex64;
use crate::fft_manager::FftManager;
use crate::fast_fourier_transform;

pub fn calculate_upper_env(signal: &Array2<f64>)
-> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> {
    
    
    let mean = signal.mean().unwrap();
    let mut signal_centered = signal - mean;
    let hilbert = hilbert(&mut signal_centered);

    let mut hilbert_amp = Array2::<f64>::zeros((hilbert.nrows(), hilbert.ncols()));
    
    for i in 0..hilbert.nrows()
    {
        hilbert_amp[(i, 0)] = hilbert[(i,0)].norm();
    }
    hilbert_amp += mean;
    hilbert_amp
}

pub fn hilbert(signal: &mut Array2<f64>)
-> ndarray::ArrayBase<ndarray::OwnedRepr<num::Complex<f64>>, ndarray::Dim<[usize; 2]>> {
    
    // Continue here! You can do it!!!!
    let mut fft_manager = FftManager::new(signal.ncols() * signal.nrows());
    let freq_domain_signal = fast_fourier_transform::forward_1d_from_matrix(&mut fft_manager, signal);
    
    let is_odd = signal.nrows() % 2 == 1;
    let is_non_empty = signal.nrows() > 0;

    // Set up scaling vector
    let mut hilbert_scaling = vec![0.0f64; freq_domain_signal.nrows()];
    hilbert_scaling[0] = 1.0;

    if !is_odd && is_non_empty
    {
        hilbert_scaling[signal.nrows() / 2] = 1.0;
    }
    else if is_odd && is_non_empty
    {
        hilbert_scaling[signal.nrows() / 2] = 2.0;
    }

    let n = if is_odd{(freq_domain_signal.nrows() + 1) / 2} else {freq_domain_signal.nrows() / 2};

    for row_index in 1..n
    {
        hilbert_scaling[row_index] = 2.0;
    }

    let hilbert_scaling_mat = Array2::<f64>::from_shape_vec((hilbert_scaling.len(), 1), hilbert_scaling).unwrap();

    let mut element_wise_product = Array2::<Complex64>::zeros((freq_domain_signal.ncols() * freq_domain_signal.nrows(), 1));

    for i in 0..freq_domain_signal.nrows()
    {
        element_wise_product[(i, 0)] = freq_domain_signal[(i, 0)] * hilbert_scaling_mat[(i, 0)];
    }

    let hilbert = fast_fourier_transform::inverse_1d(&mut fft_manager, &mut element_wise_product);
    // ???
    hilbert * 2.0 - 0.000001
}