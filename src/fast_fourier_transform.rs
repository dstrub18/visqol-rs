use ndarray::{Axis, Array1, Array0};
use num::Zero;
use num::complex::Complex64;
use crate::audio_channel::AudioChannel;
use crate::fft_manager::FftManager;

pub fn forward_1d_from_matrix(fft_manager: &mut FftManager, in_matrix: &Array1::<f64>)
-> Array1<Complex64> {
  let mut temp_time_buffer = AudioChannel::<f64>::new(fft_manager.samples_per_channel);
  temp_time_buffer.aligned_buffer = in_matrix.clone().to_vec();
  
  let mut temp_freq_buffer = AudioChannel::<Complex64>::new(fft_manager.fft_size);
  fft_manager.freq_from_time_domain(&mut temp_time_buffer, &mut temp_freq_buffer);
  
  Array1::from_vec(temp_freq_buffer.aligned_buffer)
}

pub fn forward_1d_from_points(fft_manager: &mut FftManager, in_matrix: &mut Array1::<f64>, points: usize)
-> Array1<Complex64> {
  let num_points_to_append = points - in_matrix.len();
  // Continue here :)
  let mut signal = in_matrix.clone();

  let z = Array0::<f64>::zeros([]);
  for _ in 0..num_points_to_append
  {
    signal.push(Axis(0), z.view()).expect("Failed to zero pad signal when computing cross correlation!");
  }

  forward_1d_from_matrix(fft_manager, &signal)

}

pub fn inverse_1d(fft_manager: &mut FftManager, in_matrix: &mut Array1::<Complex64>)
-> Array1<Complex64> {
  let mut temp_freq_buffer = AudioChannel::<Complex64>::new(fft_manager.fft_size);
  temp_freq_buffer.aligned_buffer = in_matrix.clone().to_vec();
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
  Array1::from_vec(out_vec[..length].to_vec())
}

pub fn inverse_1d_conj_sym(fft_manager: &mut FftManager, in_matrix: &mut Array1::<Complex64>)
-> Array1<f64>
{
  let inverse = inverse_1d(fft_manager, in_matrix);

  let mut time_signal = Vec::<f64>::with_capacity(in_matrix.len());

  for element in inverse.iter()
  {
    time_signal.push(element.re);
  }
  Array1::from_vec(time_signal)
}