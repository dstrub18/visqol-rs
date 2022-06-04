
use crate::audio_signal::AudioSignal;
use crate::analysis_window::AnalysisWindow;
use crate::spectrogram::Spectrogram;
use crate::spectrogram_builder::SpectrogramBuilder;
use crate::gammatone_filterbank::GammatoneFilterbank;
use crate::equivalent_rectangular_bandwidth::equivalent_rectangular_bandwidth;
use crate::misc_vector::array2_to_vec;
use ndarray::{Array2, Axis};
pub struct GammatoneSpectrogramBuilder
{
    filter_bank: GammatoneFilterbank,
    speech_mode: bool
}

impl GammatoneSpectrogramBuilder
{
    fn build(&mut self, signal: &AudioSignal, window: &AnalysisWindow)// -> Result<Spectrogram, ()> 
    {
        let sig = signal.data_matrix.clone();
        let sample_rate = signal.sample_rate;
        let max_freq = if self.speech_mode {Self::SPEECH_MODE_MAX_FREQ} else{ sample_rate / 2};

        // get gammatone coefficients
        let erb_result = equivalent_rectangular_bandwidth::make_filters(sample_rate as usize, self.filter_bank.num_bands, self.filter_bank.min_freq, max_freq as f64);
        let mut coeffs = erb_result.filter_coeffs.clone();
    
        coeffs.invert_axis(Axis(0));
        self.filter_bank.set_filter_coefficients(&coeffs);
        self.filter_bank.reset_filter_conditions();

        let hop_size = window.size * window.overlap as usize;
        assert!(sig.nrows() > window.size, "too few samples!");

        let num_cols = 1 + (sig.nrows() - window.size) / hop_size;

        let mut _out_matrix = Array2::<f64>::zeros((self.filter_bank.num_bands, num_cols));

        
        // run windowing
        // Good job, Daniel :) Continue here
    }
}

impl GammatoneSpectrogramBuilder
{
    pub const SPEECH_MODE_MAX_FREQ: u32 = 8000;
    pub fn new(filter_bank: GammatoneFilterbank, use_speech_mode: bool)
    -> Self     
    {
        Self
        {
            filter_bank,
            speech_mode: use_speech_mode
        }
    }
}