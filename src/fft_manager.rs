use crate::audio_channel::AudioChannel;
use crate::misc_math;
use num::Zero;
use num::complex::Complex64;
use realfft::RealFftPlanner;

// Constants
const MIN_FFT_SIZE: usize = 32;
pub struct FftManager
{
    planner: RealFftPlanner::<f64>,
    pub fft_size: usize,
    inverse_fft_scale: f64,
    pub samples_per_channel: usize,
    pub time_channel: AudioChannel<f64>,
    pub freq_channel: AudioChannel<Complex64>
}

impl FftManager
{
    pub fn new(samples_per_channel: usize) -> Self
    {
        let fft_size = misc_math::next_pow_two(&samples_per_channel).max(MIN_FFT_SIZE);
        Self
        {
            planner: RealFftPlanner::<f64>::new(),
            fft_size,
            samples_per_channel,
            inverse_fft_scale: 1.0f64 / (fft_size as f64),
            time_channel: AudioChannel::<f64>::new(fft_size),
            freq_channel: AudioChannel::<Complex64>::new(fft_size)
        }
    }

    pub fn freq_from_time_domain(&mut self, time_channel: &mut AudioChannel<f64>, freq_channel: &mut AudioChannel<Complex64>)
    {
        let real_to_complex = self.planner.plan_fft_forward(self.fft_size);
        assert!(time_channel.get_size() <= self.fft_size);
        
        if time_channel.get_size() == self.fft_size
        {
            real_to_complex.process(time_channel.aligned_buffer.as_mut_slice(), freq_channel.aligned_buffer.as_mut_slice()).unwrap();
        }
        else
        {
            time_channel.aligned_buffer.resize(self.fft_size, 0.0f64);
            assert!(time_channel.aligned_buffer.len() == self.fft_size);
            real_to_complex.process(time_channel.aligned_buffer.as_mut_slice(), freq_channel.aligned_buffer.as_mut_slice()).unwrap();
        }
    }
    
    pub fn time_from_freq_domain(&mut self, freq_channel: &mut AudioChannel<Complex64>, time_channel: &mut AudioChannel<f64>)
    {
        let complex_to_real = self.planner.plan_fft_inverse(self.fft_size);

        if time_channel.get_size() == self.fft_size
        {
            complex_to_real.process(&mut freq_channel.aligned_buffer[..self.samples_per_channel], time_channel.aligned_buffer.as_mut_slice()).unwrap();

        }

        else
        {
            time_channel.aligned_buffer.resize(self.fft_size, f64::zero());
            complex_to_real.process(&mut freq_channel.aligned_buffer[..self.samples_per_channel], time_channel.aligned_buffer.as_mut_slice()).unwrap();
        }
    }
    
    pub fn apply_reverse_fft_scaling(&self, time_channel: &mut AudioChannel<f64>)
    {
        assert!(time_channel.get_size() == self.fft_size || time_channel.get_size() == self.fft_size);
        time_channel.aligned_buffer.iter_mut().for_each(|x|{*x = *x * self.inverse_fft_scale;});
    }

    pub fn clear_freq_channel(&mut self)
    {
    }
   
}