use ndarray::{Array2, Axis, Array1};
use num::Zero;
use num::complex::Complex64;
use crate::audio_channel::AudioChannel;
use crate::fft_manager::FftManager;

pub fn forward_1d_from_matrix(fft_manager: &mut FftManager, in_matrix: &Array2::<f64>)
-> ndarray::ArrayBase<ndarray::OwnedRepr<num::Complex<f64>>, ndarray::Dim<[usize; 2]>> {
  let in_num_cols = in_matrix.ncols();
  let mut temp_time_buffer = AudioChannel::<f64>::new(fft_manager.samples_per_channel);
  temp_time_buffer.aligned_buffer = in_matrix.clone().remove_axis(Axis(1)).to_vec();
  
  let mut temp_freq_buffer = AudioChannel::<Complex64>::new(fft_manager.fft_size);
  fft_manager.freq_from_time_domain(&mut temp_time_buffer, &mut temp_freq_buffer);
  
  Array2::from_shape_vec((temp_freq_buffer.get_size(), in_num_cols), temp_freq_buffer.aligned_buffer).unwrap()
}

pub fn forward_1d_from_points(fft_manager: &mut FftManager, in_matrix: &mut Array2::<f64>, points: usize)
-> ndarray::ArrayBase<ndarray::OwnedRepr<num::Complex<f64>>, ndarray::Dim<[usize; 2]>> {
  let num_points_to_append = points - in_matrix.nrows();
  // Continue here :)
  let mut signal = in_matrix.clone();

  let z = Array1::<f64>::zeros(1);
  for _ in 0..num_points_to_append
  {
    signal.push(Axis(0), z.view()).unwrap();
  }

  forward_1d_from_matrix(fft_manager, &signal)

}

pub fn inverse_1d(fft_manager: &mut FftManager, in_matrix: &mut Array2::<Complex64>)
-> ndarray::ArrayBase<ndarray::OwnedRepr<num::Complex<f64>>, ndarray::Dim<[usize; 2]>> {
  let mut temp_freq_buffer = AudioChannel::<Complex64>::new(fft_manager.fft_size);
  temp_freq_buffer.aligned_buffer = in_matrix.clone().remove_axis(Axis(1)).to_vec();
  let mut temp_time_buffer = AudioChannel::<f64>::new(fft_manager.samples_per_channel);
  fft_manager.time_from_freq_domain(&mut temp_freq_buffer,  &mut temp_time_buffer);
  fft_manager.apply_reverse_fft_scaling(&mut temp_time_buffer);

  // This makes very little sense but oh well...
  let mut out_vec = vec![Complex64::zero();fft_manager.samples_per_channel];
  for (i, elem) in out_vec.iter_mut().enumerate()
  {
    elem.re = temp_time_buffer[i];
  }
  let length = fft_manager.samples_per_channel;
  Array2::from_shape_vec((length, 1), out_vec).unwrap()
}

pub fn inverse_1d_conj_sym(fft_manager: &mut FftManager, in_matrix: &mut Array2::<Complex64>)
-> ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>> 
{
  let inverse = inverse_1d(fft_manager, in_matrix);

  let mut time_signal = Vec::<f64>::with_capacity(in_matrix.len());

  for element in inverse.iter()
  {
    time_signal.push(element.re);
  }
  let length = inverse.len();
  Array2::from_shape_vec((length, 1), time_signal).unwrap()
}