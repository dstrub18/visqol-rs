use ndarray::{Array1};
use num::complex::Complex64;
use crate::fft_manager::FftManager;
use crate::fast_fourier_transform;

pub fn calculate_upper_env(signal: &Array1<f64>)
-> Option<ndarray::Array1<f64>>
{
    let mean = signal.mean()?;
    let mut signal_centered = signal - mean;
    let hilbert = hilbert(signal_centered.as_slice_mut()?)?;

    let mut hilbert_amplitude = Array1::<f64>::zeros(hilbert.len());
    
    for (amplitude, h) in hilbert_amplitude.iter_mut().zip(&hilbert)
    {
        *amplitude = h.norm();
    }
    hilbert_amplitude += mean;
    Some(hilbert_amplitude)
}

pub fn hilbert(signal: &mut [f64])
-> Option<Array1<Complex64>> {
    
    let mut fft_manager = FftManager::new(signal.len());
    let freq_domain_signal = fast_fourier_transform::forward_1d_from_matrix(&mut fft_manager, signal);
    
    let is_odd = signal.len() % 2 == 1;
    let is_non_empty = !signal.is_empty();

    // Set up scaling vector
    let mut hilbert_scaling = vec![0.0f64; freq_domain_signal.len()];
    hilbert_scaling[0] = 1.0;

    if !is_odd && is_non_empty
    {
        hilbert_scaling[signal.len() / 2] = 1.0;
    }
    else if is_odd && is_non_empty
    {
        hilbert_scaling[signal.len() / 2] = 2.0;
    }

    let n = if is_odd{(freq_domain_signal.len() + 1) / 2} else {freq_domain_signal.len() / 2};

    (1..n).for_each(|row_index| {
        hilbert_scaling[row_index] = 2.0;
    });

    let mut element_wise_product = Array1::<Complex64>::zeros(freq_domain_signal.len());

    for i in 0..freq_domain_signal.len()
    {
        element_wise_product[i] = freq_domain_signal[i] * hilbert_scaling[i];
    }

    let mut hilbert = fast_fourier_transform::inverse_1d(&mut fft_manager, element_wise_product.as_slice()?);
    hilbert.iter_mut().for_each(|element|{*element = *element *2.0 - 0.000001});
    Some(Array1::<Complex64>::from_vec(hilbert))
}